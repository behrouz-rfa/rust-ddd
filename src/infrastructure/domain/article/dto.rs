
use diesel::pg::Pg;
use diesel::prelude::*;
use crate::schema::articles;
use rocket::serde::{Deserialize, Serialize, json, json::Json};
use crate::domain::article::entity::Article;
use crate::domain::user::entity::User;
use crate::presentation::config::DATE_FORMAT;
use diesel;

#[derive(Insertable)]
#[table_name = "articles"]
pub struct NewArticle<'a> {
   pub title: &'a str,
    pub description: &'a str,
    pub body: &'a str,
    pub  slug: &'a str,
    pub  author: i32,
    pub(crate) tag_list: &'a Vec<String>,
}


// impl From<Article> for NewArticle<'_> {
//     fn from(value: Article) -> Self {
//         Self {
//
//
//         }
//     }
// }


#[derive(Deserialize, AsChangeset, Default, Clone)]
#[table_name = "articles"]
pub struct UpdateArticleData {
  pub title: Option<String>,
  pub description: Option<String>,
  pub body: Option<String>,
   #[serde(skip)]
  pub slug: Option<String>,
   #[serde(rename = "tagList")]
  pub tag_list: Vec<String>,
}