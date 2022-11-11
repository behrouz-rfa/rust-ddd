use crate::domain::comment::entity::Comment;
use crate::error::Result;
pub trait Repository {
   fn create(&self, user: &Comment) -> Result<Comment>;
   fn find(&self)-> Vec<Comment>;
   fn find_by(&self,user: &Comment)-> Result<Comment>;
   fn find_one(&self, id: &i32) -> Result<Comment>;
   fn update(&self, id:i32, c: &Comment) -> Result<Comment>;

}