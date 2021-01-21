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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostForumRequest {
    pub forum_name: String,
}

#[derive(Serialize)]
pub struct Forums {
    _embedded: ForumList,
    _links: ForumsLinks,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Subforum {
    id: u64,
    subforum_name: String,
    forum_id: u64,
    #[serde(rename = "_links")]
    links: SubforumLinks,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostSubforumRequest {
    pub subforum_name: String,
    pub forum_id: u64,
}

#[derive(Serialize)]
pub struct Subforums {
    _embedded: SubforumList,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubforumList {
    subforum_list: Vec<Subforum>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForumList {
    forum_list: Vec<Forum>,
}

#[derive(Serialize)]
pub struct ForumLinks {
    #[serde(rename = "self")]
    pub _self: Link,
    pub forums: Link,
    pub subforums: Link,
}

#[derive(Serialize)]
pub struct SubforumLinks {
    #[serde(rename = "self")]
    pub _self: Link,
    pub forum: Link,
    pub posts: Link,
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

fn gen_forum_links(forum_id: u64) -> ForumLinks {
    ForumLinks {
        _self: Link { href: format!("<url>/api/forums/{}", forum_id) },
        forums: Link { href: format!("<url>/api/forums") },
        subforums: Link { href: format!("<url>/api/forums/{}/subforums", forum_id) },
    }
}

fn gen_sub_links(subforum_id: u64, forum_id: u64) -> SubforumLinks {
    SubforumLinks {
        _self: Link { href: format!("<url>/api/subforums/{}", subforum_id) },
        forum: Link { href: format!("<url>/api/forums/{}", forum_id) },
        posts: Link { href: format!("<url>/api/forums/{}/subforums", forum_id) },
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

pub async fn post_subforum(subforum_request: PostSubforumRequest,
                            pool: &MySqlPool) -> Result<Subforum> {
    let mut tx = pool.begin().await?;

    let subforum_id = sqlx::query!(
        "INSERT INTO subforums (subforum_name, forum_id) VALUES (?, ?)",
        subforum_request.subforum_name,
        subforum_request.forum_id)
        .execute(&mut tx)
        .await?
        .last_insert_id();

    tx.commit().await?;

    Ok(Subforum {
        subforum_name: subforum_request.subforum_name,
        forum_id: subforum_request.forum_id,
        id: subforum_id,
        links: gen_sub_links(subforum_id, subforum_request.forum_id),
    })
}

pub async fn get_subforums(forum_id: u64, pool: &MySqlPool) -> Result<Subforums> {
    let results = sqlx::query!(
        "SELECT subforum_id, subforum_name FROM subforums")
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

    Ok(Subforums { _embedded: SubforumList { subforum_list: subforums } })

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
            _links: ForumsLinks { _self: Link { href: format!("<url>/api/forums") } },
        })
}
