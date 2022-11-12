#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::{Build, Error, Rocket};

use crate::presentation::server::{Server, ServiceState};


#[macro_use]
extern crate diesel;

#[macro_use]
extern crate validator_derive;


mod infrastructure;
mod presentation;
mod domain;
mod schema;
mod application;
mod error;
mod errors;


use std::sync::Arc;
use std::sync::atomic::Ordering;
use crossbeam::epoch::Atomic;
use rocket::serde::json::{json, Value};

use dotenv::dotenv;
use rocket::{Request, State};
use rocket::fairing::AdHoc;
use rocket::request::{FromRequest, Outcome};

use crate::infrastructure::db::{DbPool, get_dbpool};
use crate::application::services::domain::comment::service::CommentService;
use crate::application::services::domain::user::service::UserService;
use crate::application::services::domain::article::service::ArticleService;
use crate::application::services::domain::profile::service::ProfileService;
use crate::infrastructure::domain::user::repository::UserRepository;
use crate::infrastructure::domain::article::repository::ArticleRepository;
use crate::infrastructure::domain::comment::repository::CommentRepository;
use crate::infrastructure::domain::profile::repository::ProfileRepository;
use crate::presentation::http_handler::users::{get_user, insert_users, login_user, update_user};
use crate::presentation::http_handler::profile::{get_profile, follow, unfollow};
use crate::presentation::http_handler::article::{create_article, get_article, delete_article, delete_comment, favorite_article, get_articles_feed, get_comments, get_articles, post_comment, unfavorite_article};
use crate::presentation::config::{AppState, from_env};

#[catch(404)]
pub fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[launch]
pub fn rocket() -> _ {

    //get db pool for `db`
    //you can clone th DbPool and pass them to  multi Repository
    let pool = get_dbpool();
    //create a new instance of UserRepository
    let user_repo = UserRepository::new(pool.clone());
    // Create UserService and pass user repository as an Dependency for UserService
    let user_service = UserService::new(user_repo);

    //create a new instance of ArticleRepository
    let article_repo = ArticleRepository::new(pool.clone());
    // Create UserService and pass user repository as an Dependency for ArticleService
    let articel_service = ArticleService::new(article_repo);


    //create a new instance of ProfileRepository
    let profile_repo = ProfileRepository::new(pool.clone());
    // Create UserService and pass user repository as an Dependency for ProfileService
    let profile_service = ProfileService::new(profile_repo);

    //create a new instance of CommentRepository
    let comment_repo = CommentRepository::new(pool.clone());
    // Create UserService and pass user repository as an Dependency for CommentService
    let comment_service = CommentService::new(comment_repo);

    //Build rocket launcher adn pass all route and state to the manage and attach
    rocket::custom(from_env())
        .manage(ServiceState { service: Atomic::new(user_service) })
        .manage(ServiceState { service: Atomic::new(articel_service) })
        .manage(ServiceState { service: Atomic::new(profile_service) })
        .manage(ServiceState { service: Atomic::new(comment_service) })
        .attach(AppState::manage())
        .mount("/api", routes![
                insert_users,
                get_user,
                login_user,
                update_user,
                create_article,get_article,
                delete_comment,favorite_article,delete_article,
                get_articles_feed,get_comments,get_articles
                ,post_comment,unfavorite_article,
                get_profile,follow,unfollow
            ])

        .register("/", catchers![not_found])
}

