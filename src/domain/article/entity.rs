use chrono::{DateTime, Duration, Utc};
use diesel::pg::Pg;
use diesel::prelude::*;
use crate::schema::articles;
use rocket::serde::{Deserialize,Serialize, json, json::Json};
use crate::domain::user::entity::User;
use crate::presentation::config::DATE_FORMAT;

#[derive(Queryable,Serialize)]
pub struct Article {
    pub(crate) id: i32,
    pub(crate) slug: String,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) body: String,
    pub(crate) author: i32,
    pub(crate) tag_list: Vec<String>,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
    pub(crate) favorites_count: i32,
}

impl Article {
    pub fn attach(self, author: User, favorited: bool) -> ArticleJson {
        ArticleJson {
            id: self.id,
            slug: self.slug,
            title: self.title,
            description: self.description,
            body: self.body,
            author,
            tag_list: self.tag_list,
            created_at: self.created_at.format(DATE_FORMAT).to_string(),
            updated_at: self.updated_at.format(DATE_FORMAT).to_string(),
            favorites_count: self.favorites_count,
            favorited,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleJson {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub author: User,
    pub tag_list: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    pub favorites_count: i32,
    pub favorited: bool,
}
#[derive(FromForm, Default)]
pub struct FindArticles {
    pub tag: Option<String>,
    pub author: Option<String>,
    /// favorited by user
    pub favorited: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(FromForm, Default)]
pub struct FeedArticles {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}



