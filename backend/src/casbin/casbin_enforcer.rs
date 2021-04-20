use std::sync::Arc;

use anyhow::Result;

use sqlx::MySqlPool;
use sqlx_adapter::casbin::prelude::*;
use async_std::sync::RwLock;

use crate::casbin::model;

macro_rules! enum_str {
    (pub enum $name:ident {
        $($variant:ident),*,
    }) => {
        #[derive(Debug)]
        pub enum $name {
            #[allow(dead_code)]
            $($variant),*
        }

        impl $name {
            pub fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }

            #[allow(dead_code)]
            pub fn from_str(string: &str) -> Option<Self> {
                match string {
                    $(stringify!($variant) => (Some($name::$variant))),*,
                    _ => None,
                }
            }
        }
    };
}

macro_rules! unique_id {
    ($x:expr, $y:expr) => {
        format!("{}#{}", $x, $y)
    }
}

#[derive(Clone)]
pub struct CasbinData {
    enforcer: Arc<RwLock<CachedEnforcer>>
}

enum_str! {
    pub enum Action {
        Read,
        Write,
        Delete,
        Edit,
    }
}

enum_str! {
    pub enum Role {
        Admin,
        Creator,
        Moderator,
        User,
        Guest,
    }
}

impl Role {
    pub fn can_lock_post(role: &str) -> bool {
        matches!(role, "Admin" | "Creator" | "Moderator")
    }
}

pub enum Object {
    Forum(u64),
    Post(u64),
    Subforum(u64),
    Comment,
}

impl Object {
    //pub fn verify(object: &str) -> bool {
    //    match object {
    //        "comment" => true,
    //        _ => match &object.split('/').collect::<Vec<&str>>()[..] {
    //            ["Forum", num] | ["Post", num] | ["Subforum", num] => {
    //                num.parse::<u64>().is_ok()
    //            },
    //            _ => false,
    //        },
    //    }
    //}

    pub fn name(&self) -> String {
        match self {
            Self::Forum(id) => format!("Forum/{}", id),
            Self::Post(id) => format!("Post/{}", id),
            Self::Subforum(id) => format!("Subforum/{}", id),
            Self::Comment => "Comment".to_string(),
        }
    }

    pub fn all_subforums() -> String {
        "Subforum/:id".to_string()
    }

    pub fn all_forums() -> String {
        "Forum/:id".to_string()
    }

    pub fn all_posts() -> String {
        "Post/:id".to_string()
    }
}

pub fn default_perms(role: Role, forum_id: Option<u64>) -> Vec<Vec<String>> {
    let forum = match forum_id {
        Some(id) => Object::Forum(id).name(),
        None => Object::all_forums(),
    };

    vec![
        vec![
            role.name().to_string(),
            forum.clone(),
            Object::all_subforums(),
            Action::Write.name().to_string(),
            "allow".to_string(),
        ],
        vec![
            role.name().to_string(),
            forum.clone(),
            Object::all_subforums(),
            Action::Read.name().to_string(),
            "allow".to_string(),
        ],
        vec![
            role.name().to_string(),
            forum.clone(),
            forum.clone(),
            Action::Read.name().to_string(),
            "allow".to_string(),
        ],
        vec![
            role.name().to_string(),
            forum,
            Object::all_posts(),
            Action::Write.name().to_string(),
            "allow".to_string(),
        ],
    ]
}

pub fn mod_perms(role: Role, forum_id: Option<u64>) -> Vec<Vec<String>> {
    let forum = match forum_id {
        Some(id) => Object::Forum(id).name(),
        None => Object::all_forums(),
    };

    vec![
        vec![
            role.name().to_string(),
            forum.clone(),
            forum.clone(),
            Action::Write.name().to_string(),
            "allow".to_string(),
        ],
        vec![
            role.name().to_string(),
            forum.clone(),
            Object::all_posts(),
            Action::Delete.name().to_string(),
            "allow".to_string(),
        ],
        vec![
            role.name().to_string(),
            forum,
            Object::Comment.name(),
            Action::Delete.name().to_string(),
            "allow".to_string(),
        ]
    ]
}

impl CasbinData {
    pub async fn new<M: TryIntoModel, A: TryIntoAdapter>(m: M, a: A) -> Result<Self> {
        let enforcer: CachedEnforcer = CachedEnforcer::new(m, a).await?;
        enforcer.get_role_manager()
            .write()
            .map(|mut r| r.matching_fn(None, Some(casbin::function_map::key_match2))).unwrap();

        Ok(CasbinData {
            enforcer: Arc::new(RwLock::new(enforcer)),
        })
    }

    pub async fn enforce(
        &self,
        user_id: String,
        impl_id: u64,
        forum_id: u64,
        object: Object,
        action: Action
    ) -> Result<bool, casbin::Error> {
        let cloned_enforcer = self.enforcer.clone();
        let object = object.name();
        let action = action.name();
        let forum_id = Object::Forum(forum_id).name();

        let id = unique_id!(user_id, impl_id);

        let mut lock = cloned_enforcer.write().await;
            let user_roles = lock.get_roles_for_user(&id, Some(&forum_id));
            let id = if user_roles.is_empty() {
                Role::Guest.name().to_string()
            } else {
                id
            };
            let case = vec![id, forum_id, object, action.to_string()];
            let val = lock.enforce_mut(case);
        drop(lock);
        val
    }

    pub async fn get_roles(
        &self,
        forum_id: u64,
        role: Role,
        pool: &MySqlPool
    ) -> Vec<crate::users::User> {
        let enforcer = self.enforcer.clone();
        let domain = Object::Forum(forum_id).name();

        let lock = enforcer.write().await;
            let users = lock.get_users_for_role(role.name(), Some(&domain));
            let mut new_users = Vec::with_capacity(users.len());
            for user in users {
                let split_index = user.rfind('#').unwrap();
                let (id, num) = user.split_at(split_index);

                let num: u64 = num[1..].parse().unwrap();

                let user = crate::users::get_user(id.to_string(), num, &pool).await.unwrap();
                new_users.push(user);
            }
        drop(lock);

        new_users
    }

    pub async fn lock_post(&self, post_id: u64, forum_id: u64, role: Role
    ) -> Result<bool, casbin::Error> {
        let enforcer = self.enforcer.clone();
        let domain = Object::Forum(forum_id).name();
        let post = Object::Post(post_id).name();
        let policy = vec![role.name(), &domain, &post, Action::Write.name(), "deny"]
            .into_iter().map(|s| s.to_string()).collect();

        let mut lock = enforcer.write().await;
            let users = lock.add_policy(policy).await;
        drop(lock);

        users
    }

    pub async fn get_permissions(
        &self,
        role: &Role,
        forum_id: u64,
        subforum_id: Option<u64>,
    ) -> Result<model::Permissions> {
        let enforcer = self.enforcer.clone();

        let role = role.name();

        let forum = Object::Forum(forum_id).name();
        let subforum = match subforum_id {
            Some(id) => Object::Subforum(id).name(),
            None => Object::all_subforums(),
        };

        let can_post_posts = vec![
            role.to_string(),
            forum.clone(),
            subforum.clone(),
            Action::Write.name().to_string()
        ];
        let can_view_posts = vec![
            role.to_string(),
            forum.clone(),
            subforum,
            Action::Read.name().to_string()
        ];

        let lock = enforcer.write().await;
        let can_post_posts = {
            lock.enforce(can_post_posts)?
        };
        let can_view_posts = {
            lock.enforce(can_view_posts)?
        };
        drop(lock);

        Ok(model::Permissions {
            can_post_posts,
            can_view_posts,
        })
    }

    pub async fn setup_admin(&self) {
        let cloned_enforcer = self.enforcer.clone();
        let admin_policies: Vec<Vec<_>> = vec![
            vec![Role::Admin.name(), "Forum/:id", "Forum/:id", Action::Read.name(), "allow"],
            vec![Role::Admin.name(), "Forum/:id", "Forum/:id", Action::Write.name(), "allow"],
            vec![Role::Admin.name(), "Forum/:id", "Subforum/:id", Action::Read.name(), "allow"],
            vec![Role::Admin.name(), "Forum/:id", "Subforum/:id", Action::Write.name(), "allow"],
            vec![Role::Admin.name(), "Forum/:id", "Post/:id", Action::Write.name(), "allow"],
            vec![Role::Admin.name(), "Forum/:id", "Post/:id", Action::Delete.name(), "allow"],
        ].iter_mut().map(|vec: &mut Vec<_>| vec.iter_mut().map(|s| s.to_string()).collect()).collect();

        let mut lock = cloned_enforcer.write().await;

            for policy in admin_policies {
                let _val = lock.add_policy(policy).await;
            }

            let _val = lock.add_role_for_user(
                "ba29e13c-9449-11eb-9392-0242ac110002#1",
                Role::Admin.name(),
                Some("Forum/:id")
                ).await;

            let _val = lock.add_role_for_user(
                "12f63776-866e-11eb-9392-0242ac110002#1",
                Role::Admin.name(),
                Some("Forum/:id")
                ).await;

            let _val = lock.add_role_for_user(
                "65bed8d0-9cc6-11eb-9392-0242ac110002#1",
                Role::Admin.name(),
                Some("Forum/:id")
                ).await;

            let _val = lock.add_role_for_user(
                "b50709cc-9d04-11eb-9392-0242ac110002#1",
                Role::Admin.name(),
                Some("Forum/:id")
                ).await;
        drop(lock);
    }

    pub async fn setup_users(&self) {
        let user_policies = default_perms(Role::User, None);
        let guest_policies = default_perms(Role::Guest, None);
        let mut moderator_policies = default_perms(Role::Moderator, None);
        moderator_policies.append(&mut mod_perms(Role::Moderator, None));
        let mut creator_policies = default_perms(Role::Creator, None);
        creator_policies.append(&mut mod_perms(Role::Creator, None));

        let cloned_enforcer = self.enforcer.clone();
        let mut lock = cloned_enforcer.write().await;
            let val = lock.add_policies(user_policies).await;
            let val = lock.add_policies(guest_policies).await.and(val);
            let val = lock.add_policies(moderator_policies).await.and(val);
            let val = lock.add_policies(creator_policies).await.and(val);
        drop(lock);
        if let Err(e) = val {
            println!("Could not add new perms: {:?}", e);
        }
    }

    pub async fn setup(&self) {
        self.setup_admin().await;
        self.setup_users().await;
    }

    pub async fn get_user_role(&self, user: &str, imp_id: u64, domain: &Object) -> Vec<String> {
        let domain = domain.name();
        let id = unique_id!(user, imp_id);
        let cloned_enforcer = self.enforcer.clone();
        let mut lock = cloned_enforcer.write().await;
            let val = lock.get_roles_for_user(&id, Some(&domain));
        drop(lock);
        val
    }

    pub async fn remove_policy(&self, policy: Vec<String>) -> Result<bool, casbin::Error> {
        let cloned_enforcer = self.enforcer.clone();
        let mut lock = cloned_enforcer.write().await;
            let val = lock.remove_policy(policy).await;
        drop(lock);
        val

    }

    pub async fn has_policy(&self, policy: Vec<String>) -> bool {
        let cloned_enforcer = self.enforcer.clone();
        let lock = cloned_enforcer.write().await;
            let val = lock.has_policy(policy);
        drop(lock);
        val

    }

    pub async fn add_policy(&self, policy: Vec<String>) -> Result<bool, casbin::Error> {
        let cloned_enforcer = self.enforcer.clone();
        let mut lock = cloned_enforcer.write().await;
            let val = lock.add_policy(policy).await;
        drop(lock);
        val
    }

    pub async fn add_user_to_group(
        &self,
        user: &str,
        impl_id: u64,
        group: &Role,
        domain: &Object,
    ) -> Result<bool, casbin::Error> {
        let domain = match domain {
            Object::Forum(_) => domain.name(),
            _ => return Ok(false),
        };

        let id = unique_id!(user, impl_id);

        let cloned_enforcer = self.enforcer.clone();
        let mut lock = cloned_enforcer.write().await;
            let val = lock.add_role_for_user(&id, &group.name(), Some(&domain)).await;
        drop(lock);
        val
    }

    pub async fn remove_user_from_group(
        &self,
        user: &str,
        impl_id: u64,
        group: &Role,
        domain: &Object,
    ) -> Result<bool, casbin::Error> {
        let domain = match domain {
            Object::Forum(_) => domain.name(),
            _ => return Ok(false),
        };

        let id = unique_id!(user, impl_id);

        let cloned_enforcer = self.enforcer.clone();
        let mut lock = cloned_enforcer.write().await;
            let val = lock.delete_role_for_user(&id, &group.name(), Some(&domain)).await;
        drop(lock);
        val
    }
}
