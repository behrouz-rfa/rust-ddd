use crate::schema::{follows, users};
use diesel;
use diesel::dsl::exists;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::select;
use diesel::sql_types::Bool;


use crate::domain::profile::Profile;
use crate::infrastructure::db::DbPool;
use crate::domain::profile::repository::Repository;
use crate::domain::user::entity::User;
use crate::error::{DbError, Result};

pub struct ProfileRepository {
    db_pool: DbPool,
}

impl ProfileRepository {
    pub fn new(db_pool: DbPool) -> Self {
        Self {
            db_pool
        }
    }
}


impl Repository for ProfileRepository {
    fn find(&self, name: &str, user_id: i32) -> Result<Profile> {
        let mut conn = self.db_pool.get().unwrap();

        let user = users::table
            .filter(users::username.eq(name))
            .get_result::<User>(&mut conn)
            .map_err(Into::<DbError>::into)
            .ok();

        if let Some(user) = user {
            let follwing = self.is_following(&user, user_id)?;
            return Ok(user.to_profile(follwing));
        }
        return Err(DbError::NoFound(" not found".to_string()));
    }

    fn is_following(&self, user: &User, user_id: i32) -> Result<bool> {
        let mut conn = self.db_pool.get().unwrap();

        if let Ok(t) = select(exists(follows::table.find((user_id, user.id))))
            .execute(&mut conn){
            return Ok(true);
        }
        Err(DbError::NoFound("".to_string()))
    }

    fn follow(&self, followed_name: &str, follower_id: i32) -> Result<Profile> {
        let mut conn = self.db_pool.get().unwrap();

        let followed = users::table
            .filter(users::username.eq(followed_name))
            .get_result::<User>(&mut conn)
            .expect("Cannot load followed");

        diesel::insert_into(follows::table)
            .values((
                follows::followed.eq(followed.id),
                follows::follower.eq(follower_id),
            ))
            .execute(&mut conn)
            .expect("Cannot follow");

        Ok(followed.to_profile(true))
    }

    fn unfollow(&self, followed_name: &str, follower_id: i32) -> Result<Profile> {
        let mut conn = self.db_pool.get().unwrap();

        let followed = users::table
            .filter(users::username.eq(followed_name))
            .get_result::<User>(&mut conn)
            .expect("Cannot load followed");

        diesel::delete(follows::table.find((follower_id, followed.id)))
            .execute(&mut conn)
            .expect("Cannot unfollow");

        Ok(followed.to_profile(false))
    }
}

