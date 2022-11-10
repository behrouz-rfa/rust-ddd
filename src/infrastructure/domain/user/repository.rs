use diesel::pg::PgConnection;
use crate::domain::user::entity::{NewUser, User};
use diesel::{insert_into, RunQueryDsl};
use rand::rngs::OsRng;
use rocket::futures::TryStreamExt;
use scrypt::password_hash::{PasswordHasher, SaltString};
use scrypt::Scrypt;
use crate::domain::user::repository::Repository;
use crate::infrastructure::db::DbPool;
use crate::error::Result;
use crate::schema::users;


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
            .hash_password("&*user.username".as_bytes(), &salt)
            .expect("hash error")
            .to_string()
            .to_owned();

        let new_user = &NewUser {
            username: &*user.username,
            email: &*user.email,
            hash: &hash2[..],
        };

        // let usize = insert_into(users)
        //     .values(new_user)
        //     .execute(&mut conn)
        //     .map_err(Into::into);

        let u = diesel::insert_into(users)
            .values(new_user)
            .get_result::<User>(&mut conn)
            .map_err(Into::into);


        return u;
    }

    fn find(&self) -> Result<Vec<User>> {
        todo!()
    }

    fn find_one(&self, id: &i32) -> Result<User> {
        todo!()
    }

    fn update(&self, id: &i32, user: &User) -> Result<User> {
        todo!()
    }
}


