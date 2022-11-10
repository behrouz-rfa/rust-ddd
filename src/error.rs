//!error use for customis Database erro
//! we can track the error and create custom erro
//! base on `Error::DatabaseError`

use thiserror::Error as ThisError;
use diesel::result::{DatabaseErrorKind, Error};

pub type Result<T> = std::result::Result<T, DbError>;

/// `DatabaseError` this Error::DatabaseError we can create format! error an pass error as {0} param
/// #Example
/// ```
/// DbError::DatabaseError("error database ".to_string())
/// ```
///
/// `NoFound` use for unknown error and we pass error as param to
/// #Example
/// ```
/// DbError::NotFound("error".to_string())
/// ```
///
#[derive(Clone, Debug, ThisError)]
pub enum DbError {

    #[error("An error ocurred during database interaction. {0}")]
    DatabaseError(String),
    #[error("Not found any result. {0}")]
    NoFound(String),
}

impl From<Error> for DbError {

    /// track the error from Database error
    /// we can check the error base on `DatabaseErrorKind`
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
