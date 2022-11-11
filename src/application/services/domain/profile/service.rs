use crate::application::services::domain::article::dto::CreateArticleDto;
use crate::domain::article::entity::{Article, ArticleJson};
use crate::domain::profile::Profile;
use crate::domain::profile::repository::Repository as ProfileRepository;
use crate::error::Result;

pub struct ProfileService<R>
    where R: ProfileRepository {
    profile_repository: R,
}

impl<R> ProfileService<R>
    where R: ProfileRepository {
    pub fn new(profile_repository: R) -> Self {
        Self {
            profile_repository
        }
    }
    pub fn find(&self, name: &str, user_id: i32) -> Result<Profile> {
        self.profile_repository.find(name, user_id)
    }
    pub fn follow(&self, followed_name: &str, follower_id: i32) -> Result<Profile> {
        self.profile_repository.follow(followed_name, follower_id)
    }

    pub fn unfollow(&self, followed_name: &str, follower_id: i32) -> Result<Profile> {
        self.profile_repository.unfollow(followed_name, follower_id)
    }
}