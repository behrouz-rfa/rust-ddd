use diesel::sql_types::Bool;
use crate::application::services::domain::article::dto::{CreateArticleDto, FeedArticlesDto, FindArticlesDto, UpdateArticleDataDto};
use crate::domain::article::entity::{Article, ArticleJson, FeedArticles, FindArticles};
use crate::domain::article::repository::Repository as ArticleRepository;
use crate::error::Result;
use crate::infrastructure::domain::article::dto::UpdateArticleData;

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

    pub fn find(&self, params: FindArticlesDto, user_id:i32) -> Result<Vec<ArticleJson>> {
        let p = FindArticles::try_from(params)?;
        self.article_repository.find(&p, Some(user_id))
    }

    pub fn feed(&self, params: FeedArticlesDto,user_id:i32)->Result<Vec<ArticleJson>> {
        let fa = FeedArticles::try_from(params)?;
        self.article_repository.feed(&fa,user_id)
    }

    pub fn update(&self,slug: &str, user_id: i32,data: UpdateArticleDataDto)-> Result<ArticleJson>{
        let upd = UpdateArticleData::try_from(data)?;
        self.article_repository.update(slug,user_id,upd)
    }

    pub fn favorite(&self, slug: &str,user_id: i32)-> Result<ArticleJson> {
        self.article_repository.favorite(slug,user_id)
    }

    pub fn unfavorite(&self, slug: &str,user_id: i32)-> Result<ArticleJson> {
        self.article_repository.unfavorite(slug,user_id)
    }
    pub fn delete(&self, slug: &str,user_id: i32)->  Result<bool> {
        self.article_repository.delete(slug,user_id)
    }
    pub fn tags(&self)-> Result<Vec<String>> {
        self.article_repository.tags()
    }
}