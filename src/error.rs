
use diesel::result::{DatabaseErrorKind, Error};
pub type Result<T> = std::result::Result<T, Error>;
pub enum DbError{
    DuplicatedEmail,
    DuplicatedUsername,
}

impl From<Error> for DbError {
    fn from(err: Error) -> DbError {
        if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, info) = &err {
            match info.constraint_name() {
                Some("users_username_key") => return DbError::DuplicatedUsername,
                Some("users_email_key") => return DbError::DuplicatedEmail,
                _ => {}
            }
        }
        panic!("Error creating user: {:?}", err)
    }
}
