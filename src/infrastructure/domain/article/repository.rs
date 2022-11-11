use crate::domain::article::entity::{Article, ArticleJson, FeedArticles, FindArticles};
use crate::domain::article::repository::Repository;
use crate::domain::comment::entity::Comment;
use crate::domain::user::entity::User;
use crate::error::{Result, DbError};
use crate::infrastructure::db::DbPool;

use crate::schema::articles;
use crate::schema::favorites;
use crate::schema::follows;
use crate::schema::users;

use diesel::pg::PgConnection;
use diesel::{self, insert_into, RunQueryDsl};
use diesel::prelude::*;
use crate::infrastructure::domain::article::dto::{NewArticle, UpdateArticleData};
const SUFFIX_LEN: usize = 6;
const DEFAULT_LIMIT: i64 = 20;
pub struct ArticleRepository {
    db_pool: DbPool,
}

impl ArticleRepository {
    pub fn new(db_pool: DbPool) -> Self {
        Self {
            db_pool
        }
    }
}

impl Repository for ArticleRepository {
    fn create(&self, article: Article) -> Result<ArticleJson> {
        use crate::schema::{users::dsl::*};
        let new_articel = &NewArticle {
            slug: &*article.slug,
            title: &*article.title,
            description: &*article.description,
            body: &*article.body,
            author: article.author,
            tag_list: &article.tag_list,
        };
        let mut conn = self.db_pool.get().unwrap();

        let user_find = users
            .filter(id.eq(article.id))
            .first::<User>(&mut conn)
            .map_err(Into::<DbError>::into)
            .ok();

        if let Some(author) = user_find {
            let result = diesel::insert_into(crate::schema::articles::table)
                .values(new_articel)
                .get_result::<Article>(&mut conn)
                .map_err(Into::<DbError>::into);


            if let Ok(us) = result {
                return Ok(us.attach(author, false));
            }
        }

        Err(DbError::CustomErroeMessage("Error while we creating articles".to_string()))
    }

    fn find(&self, params: &FindArticles, user_id: Option<i32>) -> Vec<ArticleJson> {
        todo!()
    }

    fn find_by(&self, user: &Comment) -> crate::error::Result<Comment> {
        todo!()
    }

    fn find_one(&self, slug: &str, user_id: Option<i32>) -> Result<ArticleJson> {
        use crate::schema::{articles::dsl::*};
        let mut conn = self.db_pool.get().unwrap();
        let article = articles
            .filter(slug.eq(slug))
            .first::<Article>(&mut conn)
            .map_err(Into::<DbError>::into)
            .ok();

        if let Some(article) = article {
            return Ok(populate(&mut conn, article, false));
        }

        Err(DbError::CustomErroeMessage("Error while we creating articles".to_string()))
    }

    fn update(&self, slug: &str, user_id: i32, data: UpdateArticleData) -> Result<ArticleJson> {
        todo!()
    }

    fn feed(&self, params: &FeedArticles, user_id: i32) -> Result<Vec<ArticleJson>> {
        let mut conn = self.db_pool.get().unwrap();
     let result =    articles::table
            .filter(
                articles::author.eq_any(
                    follows::table
                        .select(follows::followed)
                        .filter(follows::follower.eq(user_id)),
                ),
            )
            .inner_join(users::table)
            .left_join(
                favorites::table.on(articles::id
                    .eq(favorites::article)
                    .and(favorites::user.eq(user_id))),
            )
            .select((
                articles::all_columns,
                users::all_columns,
                favorites::user.nullable().is_not_null(),
            ))
            .limit(params.limit.unwrap_or(DEFAULT_LIMIT))
            .offset(params.offset.unwrap_or(0))
            .load::<(Article, User, bool)>(&mut conn)
            .expect("Cannot load feed")
            .into_iter()
            .map(|(article, author, favorited)| article.attach(author, favorited))
            .collect();

        Ok(result)
    }

    fn favorite(&self, slug: &str, user_id: i32) -> crate::error::Result<ArticleJson> {
        let mut conn = self.db_pool.get().unwrap();
      let result =   conn.transaction::<_, diesel::result::Error, _>(|conn| {
            let article = diesel::update(articles::table.filter(articles::slug.eq(slug)))
                .set(articles::favorites_count.eq(articles::favorites_count + 1))
                .get_result::<Article>( conn)?;

            insert_into(favorites::table)
                .values((
                    favorites::user.eq(user_id),
                    favorites::article.eq(article.id),
                ))
                .execute( conn)?;

            Ok(populate( conn, article, true))
        })
            .map_err(|err| eprintln!("articles::favorite: {}", err))
            .ok();

        if let Some(t) = result {
            return Ok(t);
        }
        return Err(DbError::CustomErroeMessage("".to_string()))
    }

    fn unfavorite(&self, slug: &str, user_id: i32) -> crate::error::Result<ArticleJson> {
        let mut conn = self.db_pool.get().unwrap();
    let result =    conn.transaction::<_, diesel::result::Error, _>(|conn| {
            let article = diesel::update(articles::table.filter(articles::slug.eq(slug)))
                .set(articles::favorites_count.eq(articles::favorites_count - 1))
                .get_result::<Article>(conn)?;

            diesel::delete(favorites::table.find((user_id, article.id))).execute(conn)?;

            Ok(populate(conn, article, false))
        })
            .map_err(|err| eprintln!("articles::unfavorite: {}", err))
            .ok();
        if let Some(t) = result {
            return Ok(t);
        }
        return Err(DbError::CustomErroeMessage("".to_string()))
    }

    fn delete(&self, slug: &str, user_id: i32) -> Result<bool> {
        let mut conn = self.db_pool.get().unwrap();

        let result = diesel::delete(
            articles::table.filter(articles::slug.eq(slug).and(articles::author.eq(user_id))),
        )
            .execute(&mut conn);
        if let Err(err) = result {
           return Err(DbError::CustomErroeMessage(format!("articles::delete: {}", err)));
        }
        Ok(true)
    }
}

fn populate(conn: &mut PgConnection, article: Article, favorited: bool) -> ArticleJson {
    use crate::schema::{users::dsl::*};
    let author = users
        .filter(id.eq(article.author))
        .first::<User>(conn)
        .map_err(Into::<DbError>::into)
        .unwrap();

    article.attach(author, favorited)
}
