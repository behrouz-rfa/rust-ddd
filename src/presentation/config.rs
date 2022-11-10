//! config file use to create `AppState' for secret
//! and alsoe we put some Const Variable like as
//! `SECRET` to encode and decode Jwt token

use rocket::config::Config;
use rocket::fairing::AdHoc;
use rocket::figment::Figment;
use std::collections::HashMap;
use std::env;

/// Debug only secret for JWT encoding & decoding.
const SECRET: &'static str = "8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=";

/// js toISOString() in test suit can't handle chrono's default precision
pub const DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";

pub const TOKEN_PREFIX: &'static str = "Token ";

/// `AppState` use to keep secret key
pub struct AppState {
    /// secret must keep as Bytes
    pub secret: Vec<u8>,

}


impl AppState {
    /// A ad-hoc fairing that can be created from a function or closure.
    ///
    /// This enum can be used to create a fairing from a simple function or closure
    /// without creating a new structure or implementing `Fairing` directly.
    ///
    /// # Usage
    ///
    /// Use [`AdHoc::on_ignite`], [`AdHoc::on_liftoff`], [`AdHoc::on_request()`], or
    /// [`AdHoc::on_response()`] to create an `AdHoc` structure from a function or
    /// closure. Then, simply attach the structure to the `Rocket` instance.
    ///
    /// # Example
    ///
    /// The following snippet creates a `Rocket` instance with two ad-hoc fairings.
    /// The first, a liftoff fairing named "Liftoff Printer", simply prints a message
    /// indicating that Rocket has launched. The second named "Put Rewriter", a
    /// request fairing, rewrites the method of all requests to be `PUT`.
    ///
    /// ```rust
    /// use rocket::fairing::AdHoc;
    /// use rocket::http::Method;
    ///
    /// rocket::build()
    ///     .attach(AdHoc::on_liftoff("Liftoff Printer", |_| Box::pin(async move {
    ///         println!("...annnddd we have liftoff!");
    ///     })))
    ///     .attach(AdHoc::on_request("Put Rewriter", |req, _| Box::pin(async move {
    ///         req.set_method(Method::Put);
    ///     })));
    /// ```
    pub fn manage() -> AdHoc {
        // for add secret to App state we need to use AdHoc
        // we check is SECRET_KEY is exist on environment
        // if SECRET_KEY dose not exist panic with an error occur
        AdHoc::on_ignite("Manage config", |rocket| async move {
            let secret = env::var("SECRET_KEY").unwrap_or_else(|err| {
                if cfg!(debug_assertions) {
                    SECRET.to_string()
                } else {
                    panic!("No SECRET_KEY enviroment variable found {:?}", err)
                }
            });
            // pass secret as bytes
            rocket.manage(AppState {
                secret: secret.into_bytes(),
            })
        })
    }
}

///  from_enc load some configuration like
/// [PORT,DATABASE_URL] from Environment
/// and merge them to   Config::figment() for rocekt use
pub fn from_env() -> Figment {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8082".to_string())
        .parse::<u16>()
        .expect("PORT environment variable should pars to an integer");

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    let database_url = env::var("DATABASE_URL")
        .expect("No DATABASE_URL set in environment");
    database_config.insert("url", database_url);
    databases.insert("diesel_postgres_pool", database_config);
    Config::figment()
        .merge(("port", port))
        .merge(("database", databases))
}