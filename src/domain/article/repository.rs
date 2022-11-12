
use crate::domain::article::entity::{Article, ArticleJson, FeedArticles, FindArticles};
use crate::domain::comment::entity::Comment;
use crate::error::Result;
use crate::infrastructure::domain::article::dto::UpdateArticleData;

pub trait Repository {
   fn create(&self, user: Article) -> Result<ArticleJson>;
   fn find(&self,   params: &FindArticles, user_id: Option<i32>,)-> Result<Vec<ArticleJson>>;
   fn find_by(&self,user: &Comment)-> Result<Comment>;
   fn find_one(&self,slug: &str,  user_id: Option<i32>) -> Result<ArticleJson>;
   fn update(&self, slug: &str, user_id: i32,  data: UpdateArticleData) -> Result<ArticleJson>;
   fn feed(&self, params: &FeedArticles, user_id: i32) -> Result<Vec<ArticleJson>>;
   fn favorite(&self, slug: &str, user_id: i32) -> Result<ArticleJson>;
   fn unfavorite(&self, slug: &str, user_id: i32) -> Result<ArticleJson>;
   fn delete(&self, slug: &str, user_id: i32) -> Result<bool>;
   fn tags(&self,) -> Result<Vec<String>>;

}