use crate::domain::profile::Profile;
use crate::domain::user::entity::User;
use crate::error::Result;

pub trait Repository {
    fn find(&self, name: &str, user_id: i32) -> Result<Profile>;
    fn is_following(&self, user: &User, user_id: i32) -> Result<bool>;
    fn follow(&self, followed_name: &str, follower_id: i32) -> Result<Profile>;
    fn unfollow(&self, followed_name: &str, follower_id: i32) -> Result<Profile>;
}