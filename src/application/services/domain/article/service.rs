use crate::application::services::domain::article::dto::CreateArticleDto;
use crate::domain::article::entity::{Article, ArticleJson};
use crate::domain::article::repository::Repository as ArticleRepository;
use crate::error::Result;

pub struct ArticleService<R>
    where R: ArticleRepository {
    article_repository: R,
}

impl<R> ArticleService<R>
    where R: ArticleRepository {
    pub fn new(article_repository: R) -> Self {
        Self {
            article_repository
        }
    }

    pub fn create(&self, dto: CreateArticleDto) -> Result<ArticleJson> {
        let item = Article::try_from(dto)?;
        self.article_repository.create(item)
    }

    pub fn find_one(&self, slug: &str, user_id: Option<i32>) -> Result<ArticleJson> {
        self.article_repository.find_one(slug, user_id)
    }
}