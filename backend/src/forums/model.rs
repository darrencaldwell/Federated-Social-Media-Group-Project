use sqlx::MySqlPool;
use anyhow::Result;
use serde::{Serialize, Deserialize};
use serde::ser::{Serializer, SerializeStruct};

/// Represents a forum
impl Serialize for Forum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
            S: Serializer {
        let mut state = serializer.serialize_struct("Forum", 3)?;
        state.serialize_field("id", &self.id.to_string())?;
        state.serialize_field("forumName", &self.forum_name)?;
        state.serialize_field("_links", &self.links)?;
        state.end()
    }
}
pub struct Forum {
    pub id: u64,
    pub forum_name: String,
    pub links: ForumLinks,
}

/// Represents a request to create a new [Forum]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostForumRequest {
    pub forum_name: String,
}

/// Root of list of [Forum]
#[derive(Serialize)]
pub struct Forums {
    _embedded: ForumList,
    _links: ForumsLinks,
}

impl Serialize for Subforum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
            S: Serializer {
        let mut state = serializer.serialize_struct("Subforum", 4)?;
        state.serialize_field("id", &self.id.to_string())?;
        state.serialize_field("subforumName", &self.subforum_name)?;
        state.serialize_field("forumId", &self.forum_id.to_string())?;
        state.serialize_field("_links", &self.links)?;
        state.end()
    }
}
/// Represents a single subforum within the database
pub struct Subforum {
    id: u64,
    subforum_name: String,
    forum_id: u64,
    links: SubforumLinks,
}

/// Represents a request to create a new [Subforum]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostSubforumRequest {
    pub subforum_name: String,
}

/// Root of list of [Subforum]
#[derive(Serialize)]
pub struct Subforums {
    _embedded: SubforumList,
    _links: SubForumsLinks,
}

/// List of [Subforum]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubforumList {
    subforum_list: Vec<Subforum>,
}

/// List of [Forum]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForumList {
    forum_list: Vec<Forum>,
}

/// Links used for each [Forum]
#[derive(Serialize)]
pub struct ForumLinks {
    #[serde(rename = "self")]
    pub _self: Link,
    pub forums: Link,
    pub subforums: Link,
}

/// Links used for each [Subforum]
#[derive(Serialize)]
pub struct SubforumLinks {
    #[serde(rename = "self")]
    pub _self: Link,
    pub forum: Link,
    pub posts: Link,
}

/// Links used for list [Forums]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForumsLinks {
    _self: Link,
}

/// Links used for list [SubforumsForums]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubForumsLinks {
    _self: Link,
}

#[derive(Serialize)]
pub struct Link {
    href: String
}

fn gen_forum_links(forum_id: u64) -> ForumLinks {
    ForumLinks {
        _self: Link { href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/forums/{}", forum_id) },
        forums: Link { href: "https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/forums".to_string() },
        subforums: Link { href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/forums/{}/subforums", forum_id) },
    }
}

fn gen_sub_links(subforum_id: u64, forum_id: u64) -> SubforumLinks {
    SubforumLinks {
        _self: Link { href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/subforums/{}", subforum_id) },
        forum: Link { href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/forums/{}", forum_id) },
        posts: Link { href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/subforums/{}/posts", subforum_id) },
    }
}

pub async fn get_subforum(subforum_id: u64, pool: &MySqlPool) -> Result<Subforum> {
    let rec = sqlx::query!(
        "SELECT subforum_name, forum_id FROM subforums WHERE subforum_id = ?",
        subforum_id)
        .fetch_one(pool)
        .await?;

    Ok(Subforum {
        id: subforum_id,
        subforum_name: rec.subforum_name,
        forum_id: rec.forum_id,
        links: gen_sub_links(subforum_id, rec.forum_id),
    })

}

pub async fn post_subforum(forum_id: u64, subforum_request: PostSubforumRequest,
                            pool: &MySqlPool) -> Result<Subforum> {
                                
    let mut tx = pool.begin().await?;

    let subforum_id = sqlx::query!(
        "INSERT INTO subforums (subforum_name, forum_id) VALUES (?, ?)",
        subforum_request.subforum_name,
        forum_id)
        .execute(&mut tx)
        .await?
        .last_insert_id();

    tx.commit().await?;

    Ok(Subforum {
        subforum_name: subforum_request.subforum_name,
        forum_id,
        id: subforum_id,
        links: gen_sub_links(subforum_id, forum_id),
    })
}

pub async fn get_subforums(forum_id: u64, pool: &MySqlPool) -> Result<Subforums> {
    let results = sqlx::query!(
        "SELECT subforum_id, subforum_name FROM subforums WHERE forum_id = ?", forum_id)
        .fetch_all(pool)
        .await?;

    let subforums: Vec<Subforum> = results
        .into_iter()
        .map(|rec| Subforum {
            id: rec.subforum_id,
            subforum_name: rec.subforum_name,
            forum_id,
            links: gen_sub_links(rec.subforum_id, forum_id),
        })
        .collect();

    Ok(Subforums {
        _embedded: SubforumList { subforum_list: subforums },
        _links: SubForumsLinks { _self: Link { href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/forums/{}/subforums", forum_id) } },
    })

}

pub async fn get_forum(forum_id: u64, pool: &MySqlPool) -> Result<Forum> {
    let forum_name = sqlx::query!(
        "SELECT forum_name FROM forums where forum_id = ?",
        forum_id)
        .fetch_one(pool)
        .await?
        .forum_name;

    Ok(Forum {
        id: forum_id,
        forum_name,
        links: gen_forum_links(forum_id),
    })

}

pub async fn post_forum(forum_request: PostForumRequest,
                            pool: &MySqlPool) -> Result<Forum> {
    let mut tx = pool.begin().await?;

    let forum_id = sqlx::query!(
        "INSERT INTO forums (forum_name) VALUES (?)",
        forum_request.forum_name)
        .execute(&mut tx)
        .await?
        .last_insert_id();

    tx.commit().await?;

    Ok(Forum {
        forum_name: forum_request.forum_name,
        id: forum_id,
        links: gen_forum_links(forum_id),
    })
}

pub async fn get_forums(pool: &MySqlPool) -> Result<Forums> {
    let results = sqlx::query!(
        "SELECT forum_id, forum_name FROM forums")
        .fetch_all(pool)
        .await?;

    let forums: Vec<Forum> = results
        .into_iter()
        .map(|rec| Forum {
            id: rec.forum_id,
            forum_name: rec.forum_name,
            links: gen_forum_links(rec.forum_id),
        })
        .collect();

        Ok(Forums {
            _embedded: ForumList { forum_list: forums },
            _links: ForumsLinks { _self: Link { href: "https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/forums".to_string() } },
        })
}
