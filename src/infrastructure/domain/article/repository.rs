use crate::domain::article::entity::{Article, ArticleJson, FeedArticles, FindArticles};
use crate::domain::article::repository::Repository;
use crate::domain::comment::entity::Comment;
use crate::domain::user::entity::User;
use crate::error::{Result, DbError};
use crate::infrastructure::db::{DbPool};
use crate::infrastructure::db::OffsetLimit;

use crate::schema::articles;
use crate::schema::favorites;
use crate::schema::follows;
use crate::schema::users;
use diesel::dsl;
use diesel::pg::{Pg, PgConnection};
use diesel::{self, insert_into, RunQueryDsl};
use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sql_types::{Bigint, Bool, Integer, Text};
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

    fn find(&self, params: &FindArticles, user_id: Option<i32>) -> Result<Vec<ArticleJson>> {
        let mut conn = self.db_pool.get().unwrap();
        let mut query = articles::table
            .inner_join(users::table)
            .left_join(
                favorites::table.on(articles::id
                    .eq(favorites::article)
                    .and(favorites::user.eq(user_id.unwrap_or(0)))), // TODO: refactor
            )
            .select((
                articles::all_columns,
                users::all_columns,
                favorites::user.nullable().is_not_null(),
            ))
            .into_boxed();
        if let Some(ref author) = params.author {
            query = query.filter(users::username.eq(author))
        }
        if let Some(ref tag) = params.tag {
            query = query.or_filter(articles::tag_list.contains(vec![tag]))
        }
        if let Some(ref favorited) = params.favorited {
            let result = users::table
                .select(users::id)
                .filter(users::username.eq(favorited))
                .get_result::<i32>(&mut conn);
            match result {
                Ok(id) => {
                    query = query.filter(sql::<Bool>(&format!("articles.id IN (SELECT favorites.article FROM favorites WHERE favorites.user = {})", id)));
                }
                Err(err) => match err {
                    diesel::result::Error::NotFound => return Ok(vec![]),
                    _ => panic!("Cannot load favorited user: {}", err),
                },
            }
        }

        let result = query
            .offset_and_limit(
                params.offset.unwrap_or(0),
                params.limit.unwrap_or(DEFAULT_LIMIT),
            )
            .load_and_count::<(Article, User, bool)>(&mut conn)
            .map(|(res, count)| {
                (
                    res.into_iter()
                        .map(|(article, author, favorited)| article.attach(author, favorited))
                        .collect::<Vec<ArticleJson>>(),
                    count,
                )
            });

        if let Ok(v) = result {
            return Ok(v.0);
        }

        Err(DbError::NoFound("".to_string()))
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
        let mut conn = self.db_pool.get().unwrap();
        let article = diesel::update(articles::table.filter(articles::slug.eq(slug)))
            .set(&data)
            .get_result::<Article>(&mut conn)
            .map_err(Into::<DbError>::into)
            ;
        return match article {
            Ok(article) => {
                let favorited = is_favorite(&mut conn, &article, user_id);
                Ok(populate(&mut conn, article, favorited))
            }
            Err(e) => { Err(e) }
        }
    }

    fn feed(&self, params: &FeedArticles, user_id: i32) -> Result<Vec<ArticleJson>> {
        let mut conn = self.db_pool.get().unwrap();
        let result = articles::table
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
        let result = conn.transaction::<_, diesel::result::Error, _>(|conn| {
            let article = diesel::update(articles::table.filter(articles::slug.eq(slug)))
                .set(articles::favorites_count.eq(articles::favorites_count + 1))
                .get_result::<Article>(conn)?;

            insert_into(favorites::table)
                .values((
                    favorites::user.eq(user_id),
                    favorites::article.eq(article.id),
                ))
                .execute(conn)?;

            Ok(populate(conn, article, true))
        })
            .map_err(|err| eprintln!("articles::favorite: {}", err))
            .ok();

        if let Some(t) = result {
            return Ok(t);
        }
        return Err(DbError::CustomErroeMessage("".to_string()));
    }

    fn unfavorite(&self, slug: &str, user_id: i32) -> crate::error::Result<ArticleJson> {
        let mut conn = self.db_pool.get().unwrap();
        let result = conn.transaction::<_, diesel::result::Error, _>(|conn| {
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
        return Err(DbError::CustomErroeMessage("".to_string()));
    }
    fn tags(&self)-> Result<Vec<String>> {
        let mut conn = self.db_pool.get().unwrap();
      let results =   articles::table
            .select(diesel::dsl::sql::<Text>("distinct unnest(tag_list)"))
            .load::<String>(&mut conn)
            .map_err(Into::<DbError>::into);


        return match results {
            Ok(r)=> Ok(r),
            Err(e)=> Err(e)
        }
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

fn is_favorite(conn: &mut PgConnection, article: &Article, user_id: i32) -> bool {
    use diesel::dsl::exists;
    use diesel::select;

    select(exists(favorites::table.find((user_id, article.id))))
        .get_result::<bool>( conn)
        .map_err(Into::<DbError>::into)
        .is_ok()
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
