use thiserror::Error as ThisError;
use diesel::result::{DatabaseErrorKind, Error};

pub type Result<T> = std::result::Result<T, DbError>;
#[derive(Clone, Debug, ThisError)]
pub enum DbError {
    #[error("An error ocurred during database interaction. {0}")]
    DatabaseError(String),
    #[error("Not found any result. {0}")]
    NoFound(String),
}

impl From<Error> for DbError {
    fn from(err: Error) -> Self {
        if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, info) = &err {
            return match info.constraint_name() {
                Some(db_error) => DbError::DatabaseError(db_error.to_string()),
                _ => DbError::DatabaseError("fatal".to_string())
            };
        }
        panic!("Error creating user: {:?}", err)
    }
}
