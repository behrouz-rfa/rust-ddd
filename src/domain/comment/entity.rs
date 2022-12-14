
use crate::schema::comments;
use diesel::prelude::*;
use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize,Serialize, json, json::Json};
use crate::domain::user::entity::User;
use crate::presentation::config::DATE_FORMAT;

#[derive(Queryable)]
pub struct Comment {
    pub id: i32,
    pub body: String,
    pub article: i32,
    pub author: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Comment {
    pub fn attach(self, author: User) -> CommentJson {
        CommentJson {
            id: self.id,
            body: self.body,
            author,
            created_at: self.created_at.format(DATE_FORMAT).to_string(),
            updated_at: self.updated_at.format(DATE_FORMAT).to_string(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentJson {
    pub id: i32,
    pub body: String,
    pub author: User,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable)]
#[table_name = "comments"]
pub struct NewComment<'a> {
  pub body: &'a str,
  pub author: i32,
  pub article: i32,
}

