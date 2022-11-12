use diesel::sql_types::Bool;
use crate::domain::comment::entity::CommentJson;
use crate::domain::profile::Profile;
use crate::domain::comment::repository::Repository as CommentRepository;
use crate::error::Result;

pub struct CommentService<R>
    where R: CommentRepository {
    comment_repository: R,
}

impl<R> CommentService<R>
    where R: CommentRepository {
    pub fn new(comment_repository: R) -> Self {
        Self {
            comment_repository
        }
    }
    pub fn create(&self, author: i32, slug: &str, body: &str) -> Result<CommentJson> {
        self.comment_repository.create(author, slug,body)
    }
    pub fn find_by_slug(&self, slug: &str )-> Result<Vec<CommentJson>> {
        self.comment_repository.find_by_slug(slug)
    }

    pub fn delete(&self, author: i32, slug: &str, comment_id: i32) -> Result<bool> {
        self.comment_repository.delete(author, slug,comment_id)
    }
}