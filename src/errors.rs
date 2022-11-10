//! erros use for Error request validation
//! user pass paramnter as json and alsoe as query paramete
//! we use this validation to check and passs error
//!
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::status;
use rocket::response::{self, Responder};
use rocket::serde::json::{json, Json};
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug)]
pub struct Errors {
    errors: ValidationErrors,
}

pub type FieldName = &'static str;
pub type FieldErrorCode = &'static str;

impl Errors {
    /// create new error for validation  first parameter filed name second parameter for error
    pub fn new(errs: &[(FieldName, FieldErrorCode)]) -> Self {
        let mut errors = ValidationErrors::new();
        for (field, code) in errs {
            errors.add(field, ValidationError::new(code));
        }
        Self { errors }
    }
}


impl<'r> Responder<'r, 'static> for Errors {
    /// return error as json to response rocket
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        use validator::ValidationErrorsKind::Field;

        let mut errors = json!({});
        for (field, field_errors) in self.errors.into_errors() {
            if let Field(field_errors) = field_errors {
                errors[field] = field_errors
                    .into_iter()
                    .map(|field_error| field_error.code)
                    .collect();
            }
        }

        status::Custom(
            Status::UnprocessableEntity,
            Json(json!({ "errors": errors })),
        )
        .respond_to(req)
    }
}

///this use for form FieldValidator from request [Form,Json,QueryPara]
pub struct FieldValidator {
    errors: ValidationErrors,
}

impl Default for FieldValidator {
    /// create default FieldValidator
    fn default() -> Self {
        Self {
            errors: ValidationErrors::new(),
        }
    }
}

impl FieldValidator {
    /// this fn use for validate model
    /// we need this function to create an extractor
    /// # Examples
    ///```
    ///  let new_user = new_user.into_inner().user;
    ///     let mut extractor = FieldValidator::validate(&new_user);
    ///```
    pub fn validate<T: Validate>(model: &T) -> Self {
        Self {
            errors: model.validate().err().unwrap_or_else(ValidationErrors::new),
        }
    }

    /// Convenience method to trigger early returns with ? operator.
    pub fn check(self) -> Result<(), Errors> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(Errors {
                errors: self.errors,
            })
        }
    }
    /// this fn use for validate filed
    /// # Examples
    ///```
    ///     let user = new_user.into_inner();
    ///     let mut extractor = FieldValidator::default();
    ///     let email = extractor.extract("email", user.email);
    ///     let username = extractor.extract("username", user.username);
    ///     let password = extractor.extract("password", user.password);
    ///     extractor.check()?;
    ///```
    pub fn extract<T>(&mut self, field_name: &'static str, field: Option<T>) -> T
    where
        T: Default,
    {
        field.unwrap_or_else(|| {
            self.errors
                .add(field_name, ValidationError::new("can't be blank"));
            T::default()
        })
    }
}
