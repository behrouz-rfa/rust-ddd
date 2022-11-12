use crate::domain::comment::entity::{Comment, CommentJson};
use crate::error::Result;
pub trait Repository {
   fn create(&self, author: i32, slug: &str, body: &str) -> Result<CommentJson>;
   fn find_by_slug(&self,slug: &str)-> Result<Vec<CommentJson>>;
   fn delete(&self, author: i32, slug: &str, comment_id: i32) -> Result<bool>;

}