use serde::{Deserialize, Serialize};

use crate::users::User;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AlterPermissionsRequest {
    pub can_post: bool,
    pub can_view: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleRequest {
    pub role: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRequest {
    pub role: String,
    pub user: String,
    pub imp_id: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Permissions {
    pub can_post_posts: bool,
    pub can_view_posts: bool,
}

#[derive(Serialize)]
pub struct PermissionsList {
    pub user: Permissions,
    pub guest: Permissions,
}

#[derive(Serialize)]
pub struct RoleList {
    pub creators: Vec<User>,
    pub moderators: Vec<User>,
    pub users: Vec<User>,
}
