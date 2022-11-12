use diesel::{self, insert_into, RunQueryDsl};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rand::rngs::OsRng;
use rocket::futures::TryStreamExt;
use scrypt::Scrypt;

use crate::domain::comment::entity::{Comment, CommentJson, NewComment};
use crate::domain::comment::repository::Repository;
use crate::domain::user::entity::{NewUser, UpdateUserData, User};
use crate::error::{DbError, Result};
use crate::errors::{Errors, FieldValidator};
use crate::infrastructure::db::DbPool;
use crate::schema::articles;
use crate::schema::comments;
use crate::schema::users;

// type alias to use in multiple places
pub struct CommentRepository {
    db_pool: DbPool,
}

impl CommentRepository {
    pub fn new(db_pool: DbPool) -> Self {
        Self {
            db_pool
        }
    }
}

impl Repository for CommentRepository {
    fn create(&self, author: i32, slug: &str, body: &str) -> Result<CommentJson> {
        let mut conn = self.db_pool.get().unwrap();
        let article_id = articles::table
            .select(articles::id)
            .filter(articles::slug.eq(slug))
            .get_result::<i32>(&mut conn)
            .map_err(Into::<DbError>::into);


        let Ok(article_id) = article_id else {
            return Err(DbError::CustomErroeMessage("Article not found".to_string()));
        };
        let new_comment = &NewComment {
            body,
            author,
            article: article_id,
        };

        let author = users::table
            .find(author)
            .get_result::<User>(&mut conn)
            .map_err(Into::<DbError>::into);
        let Ok(author) = author else {
            return Err(DbError::CustomErroeMessage("author not found".to_string()));
        };
        let rsult = diesel::insert_into(comments::table)
            .values(new_comment)
            .get_result::<Comment>(&mut conn)
            .map_err(Into::<DbError>::into);

        let Ok(comment) = rsult else {
            return Err(DbError::CustomErroeMessage("having problem for insert comment".to_string()));
        };
        Ok(comment.attach(author))
    }

    fn find_by_slug(&self, slug: &str) -> Result<Vec<CommentJson>> {
        let mut conn = self.db_pool.get().unwrap();
        let result = comments::table
            .inner_join(articles::table)
            .inner_join(users::table)
            .select((comments::all_columns, users::all_columns))
            .filter(articles::slug.eq(slug))
            .get_results::<(Comment, User)>(&mut conn)
            .map_err(Into::<DbError>::into);

        let Ok(result) = result else {
            return Err(DbError::CustomErroeMessage("Cannot load comments".to_string()));
        };


        Ok(result
            .into_iter()
            .map(|(comment, author)| comment.attach(author))
            .collect())
    }

    fn delete(&self, author: i32, slug: &str, comment_id: i32) -> Result<bool> {
        use diesel::dsl::exists;
        use diesel::select;
        let mut conn = self.db_pool.get().unwrap();
        let belongs_to_author_result = select(exists(
            articles::table.filter(articles::slug.eq(slug).and(articles::author.eq(author))),
        ))
            .get_result::<bool>(&mut conn);

        if let Err(err) = belongs_to_author_result {
            match err {
                diesel::result::Error::NotFound => return Err(DbError::CustomErroeMessage("Cannot find article by author ".to_string())),
                _ => panic!("Cannot find article by author: {}", err),
            }
        }

        let result = diesel::delete(comments::table.filter(comments::id.eq(comment_id))).execute(&mut conn);
        if let Err(err) = result {
            return Err(DbError::CustomErroeMessage(format!("{}", err)));
        }
        Ok(true)
    }
}

