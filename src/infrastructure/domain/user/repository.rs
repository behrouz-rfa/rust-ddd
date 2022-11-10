use diesel::pg::PgConnection;
use crate::domain::user::entity::{NewUser, User};
use diesel::{self, insert_into, RunQueryDsl};
use rand::rngs::OsRng;
use rocket::futures::TryStreamExt;
use scrypt::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use scrypt::Scrypt;
use crate::domain::user::repository::Repository;
use crate::infrastructure::db::DbPool;
use crate::error::{DbError, Result};
use crate::errors::{Errors, FieldValidator};
use diesel::prelude::*;


// type alias to use in multiple places
pub struct UserRepository {
    db_pool: DbPool,
}

impl UserRepository {
    pub fn new(db_pool: DbPool) -> Self {
        Self {
            db_pool
        }
    }
}

impl Repository for UserRepository {
    fn create(&self, user: &User) -> Result<User> {
        use crate::schema::{users::dsl::*};
        let mut conn = self.db_pool.get().unwrap();
        let salt = SaltString::generate(&mut OsRng);
        let hash2 = Scrypt
            .hash_password(&*user.hash.as_bytes(), &salt)
            .expect("hash error")
            .to_string()
            .to_owned();

        let new_user = &NewUser {
            username: &*user.username,
            email: &*user.email,
            hash: &hash2[..],
        };


        let u = diesel::insert_into(users)
            .values(new_user)
            .get_result::<User>(&mut conn)
            .map_err(Into::into);


        return u;
    }

    fn find(&self) -> Result<Vec<User>> {
        todo!()
    }

    //TODO fix login match
    fn find_by(&self, req: &User) -> Result<User> {
        use crate::schema::{users::dsl::*};
        let mut conn = self.db_pool.get().unwrap();
        let password = &*req.hash;


        let user_find = users
            .filter(email.eq(&*req.email))
            .first::<User>(&mut conn)
            .optional()
            .map_err(Into::<DbError>::into);


        if let Ok(Some(user)) = user_find {
            if verify(&user.hash, password) {
                return Ok(user);
            }
        }

        eprintln!(
            "login attempt for '{}' failed: password doesn't match",
            &*req.email
        );
        return Err(DbError::NoFound("not found".to_string()));
        // } else {
        //     eprintln!(
        //         "login attempt for '{}' failed: password doesn't match",
        //         email
        //     );
        //     return     Err(Errors::new(&[("email or password", "is invalid")]))
        // }
    }

    fn find_one(&self, id: &i32) -> Result<User> {
        todo!()
    }

    fn update(&self, id: &i32, user: &User) -> Result<User> {
        todo!()
    }
}


pub fn verify(hash: &str, password: &str) -> bool {

    if let Ok(parsed_hash) = PasswordHash::new(hash) {
        return Scrypt
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|err| eprintln!("login_user: scrypt_check: {}", err)).is_ok();
    }

    false
}