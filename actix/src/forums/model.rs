use sqlx::MySqlPool;
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Forum {
    pub id: u64,
    pub forum_name: String,
    #[serde(rename = "_links")]
    pub links: ForumLinks,
}

#[derive(Serialize)]
pub struct ForumLinks {
    #[serde(rename = "self")]
    pub _self: Link,
    pub forums: Link,
    pub subforums: Link,
}

#[derive(Serialize)]
pub struct Forums {
    _embedded: ForumList,
    _links: ForumsLinks,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForumList {
    forum_list: Vec<Forum>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForumsLinks {
    _self: Link,
}

#[derive(Serialize)]
pub struct Link {
    href: String
}

fn gen_links(forum_id: u64) -> ForumLinks {
    ForumLinks {
        _self: Link { href: format!("<url>/api/forums/{}", forum_id) },
        forums: Link { href: format!("<url>/api/forums") },
        subforums: Link { href: format!("<url>/api/forums/{}/subforums", forum_id) },
    }
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
        links: gen_links(forum_id),
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
            links: gen_links(rec.forum_id),
        })
        .collect();

        Ok(Forums {
            _embedded: ForumList { forum_list: forums },
            _links: ForumsLinks { _self: Link { href: format!("<url>/api/forums") } },
        })
}
