use std::sync::Arc;
use std::sync::atomic::Ordering;
use diesel::serialize::IsNull::No;
use rocket::request::FromRequest;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::Deserialize;
use crate::infrastructure::domain::article::repository::ArticleRepository;
use crate::application::services::domain::article::{service::ArticleService};
use rocket::State;
use crate::application::services::domain::article::dto::CreateArticleDto;

use crate::application::services::domain::user::dto::{NewUserDto, UpdateUserDto};
use crate::domain::user::entity::UpdateUserData;
use crate::error::DbError;
use crate::errors::{Errors, FieldValidator};
use crate::presentation::config::AppState;
use crate::presentation::middleware::auth::Auth;
use crate::presentation::server::ServiceState;

#[derive(Deserialize, Validate)]
pub struct NewArticleData {
    #[validate(length(min = 1))]
    pub title: Option<String>,
    #[validate(length(min = 1))]
    pub description: Option<String>,
    #[validate(length(min = 1))]
    pub body: Option<String>,
    #[serde(rename = "tagList")]
    pub tag_list: Vec<String>,
}


#[post("/article/insert", format = "json", data = "<article_req>")]
pub fn create_article(article_req: Json<NewArticleData>, auth: Auth, state: &State<AppState>, user_service: &State<ServiceState<ArticleService<ArticleRepository>>>) -> Result<Value, Errors> {
    let article = article_req.into_inner();

    let mut extractor = FieldValidator::default();
    let title = extractor.extract("title", article.title);
    let description = extractor.extract("description", article.description);
    let body = extractor.extract("body", article.body);

    extractor.check()?;

    // let secret = state.secret.clone();
    let g = &crossbeam::epoch::pin();
    if let shared = user_service.service.load(Ordering::Relaxed, g) {
        if shared.is_null() {
            return Err(Errors::new(&[("email or password", "is invalid")]));
        }
        let t = unsafe { shared.as_ref() };
        let secret = state.secret.clone();
        return t.unwrap().create(CreateArticleDto {
            author: auth.id,
            title,
            description,
            body,
            tag_list: article.tag_list,
            slug: "".to_string(),
        }).map(|article| json!(article))
            .map_err(|err| {
                println!("{}", err);

                Errors::new(&[("err", "user already exist")])
            }
            );
    }


    Err(Errors::new(&[("email or password", "is invalid")]))
}



#[get("/articles/<slug>")]
pub fn get_article(slug: String, auth: Auth, state: &State<AppState>, user_service: &State<ServiceState<ArticleService<ArticleRepository>>>) -> Result<Value, Errors> {
    let userid = auth.id;

    let g = &crossbeam::epoch::pin();
    if let shared = user_service.service.load(Ordering::Relaxed, g) {
        if shared.is_null() {
            return Err(Errors::new(&[("email or password", "is invalid")]));
        }
        let t = unsafe { shared.as_ref() };
        let secret = state.secret.clone();
        return t.unwrap().find_one(&slug,Some(userid)).map(|article| json!(article))
            .map_err(|err| {
                println!("{}", err);

                Errors::new(&[("err", "user already exist")])
            }
            );
    }


    Err(Errors::new(&[("not found", "Article not found")]))
}