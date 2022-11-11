use crate::domain::user::entity::{UpdateUserData, User};
use crate::error::Result;
pub trait Repository {
   fn create(&self, user: &User) -> Result<User>;
   fn find(&self)-> Vec<User>;
   fn find_by(&self,user: &User)-> Result<User>;
   fn find_one(&self, id: &i32) -> Result<User>;
   fn update(&self, id:i32, user: &UpdateUserData) -> Result<User>;

}