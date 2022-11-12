use std::sync::Arc;
use std::sync::atomic::Ordering;

use diesel::serialize::IsNull::No;
use rocket::request::FromRequest;
use rocket::serde::Deserialize;
use rocket::serde::json::{json, Json, Value};
use rocket::State;

use crate::application::services::domain::article::service::ArticleService;
use crate::application::services::domain::comment::service::CommentService;
use crate::application::services::domain::article::dto::{CreateArticleDto, FeedArticlesDto, FindArticlesDto, UpdateArticleDataDto};
use crate::application::services::domain::user::dto::{NewUserDto, UpdateUserDto};

use crate::domain::user::entity::UpdateUserData;
use crate::error::DbError;
use crate::errors::{Errors, FieldValidator};
use crate::infrastructure::domain::article::repository::ArticleRepository;
use crate::infrastructure::domain::comment::repository::CommentRepository;
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
            return Err(Errors::new(&[("Server", "Error")]));
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

                Errors::new(&[("create ", "could not create article")])
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
        return t.unwrap().find_one(&slug, Some(userid)).map(|article| json!(article))
            .map_err(|err| {
                println!("{}", err);

                Errors::new(&[("err", "user already exist")])
            }
            );
    }


    Err(Errors::new(&[("not found", "Article not found")]))
}

#[derive(FromForm, Default)]
pub struct FindArticles {
    pub tag: Option<String>,
    pub author: Option<String>,
    /// favorited by user
    pub favorited: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}


#[get("/articles?<params..>")]
pub fn get_articles(params: FindArticles, auth: Auth, state: &State<AppState>, user_service: &State<ServiceState<ArticleService<ArticleRepository>>>) -> Result<Value, Errors> {
    let g = &crossbeam::epoch::pin();
    if let shared = user_service.service.load(Ordering::Relaxed, g) {
        if shared.is_null() {
            return Err(Errors::new(&[("email or password", "is invalid")]));
        }
        let t = unsafe { shared.as_ref() };
        let secret = state.secret.clone();
        return t.unwrap().find(FindArticlesDto {
            tag: params.tag,
            author: params.author,
            favorited: params.favorited,
            limit: params.limit,
            offset: params.offset,
        }, auth.id).map(|article| json!({ "articles": article, "articlesCount": article.len() }))
            .map_err(|err| {
                println!("{}", err);

                Errors::new(&[("err", "user already exist")])
            }
            );
    }


    Err(Errors::new(&[("not found", "Article not found")]))
}


#[delete("/articles/<slug>")]
pub fn delete_article(slug: String, auth: Auth, state: &State<AppState>, user_service: &State<ServiceState<ArticleService<ArticleRepository>>>) -> Result<Value, Errors> {
    let g = &crossbeam::epoch::pin();
    if let shared = user_service.service.load(Ordering::Relaxed, g) {
        if shared.is_null() {
            return Err(Errors::new(&[("email or password", "is invalid")]));
        }
        let t = unsafe { shared.as_ref() };
        let secret = state.secret.clone();
        return t.unwrap().delete(&slug, auth.id).map(|article| json!(article))
            .map_err(|err| {
                println!("{}", err);

                Errors::new(&[("err", "user already exist")])
            }
            );
    }


    Err(Errors::new(&[("not found", "Article not found")]))
}


#[post("/articles/<slug>/favorite")]
pub fn favorite_article(slug: String, auth: Auth, state: &State<AppState>, user_service: &State<ServiceState<ArticleService<ArticleRepository>>>) -> Result<Value, Errors> {
    let g = &crossbeam::epoch::pin();
    if let shared = user_service.service.load(Ordering::Relaxed, g) {
        if shared.is_null() {
            return Err(Errors::new(&[("email or password", "is invalid")]));
        }
        let t = unsafe { shared.as_ref() };
        let secret = state.secret.clone();
        return t.unwrap().favorite(&slug, auth.id).map(|article| json!(article))
            .map_err(|err| {
                println!("{}", err);

                Errors::new(&[("err", "user already exist")])
            }
            );
    }


    Err(Errors::new(&[("not found", "Article not found")]))
}

#[delete("/articles/<slug>/favorite")]
pub fn unfavorite_article(slug: String, auth: Auth, state: &State<AppState>, user_service: &State<ServiceState<ArticleService<ArticleRepository>>>) -> Result<Value, Errors> {
    let g = &crossbeam::epoch::pin();
    if let shared = user_service.service.load(Ordering::Relaxed, g) {
        if shared.is_null() {
            return Err(Errors::new(&[("email or password", "is invalid")]));
        }
        let t = unsafe { shared.as_ref() };
        let secret = state.secret.clone();
        return t.unwrap().unfavorite(&slug, auth.id).map(|article| json!(article))
            .map_err(|err| {
                println!("{}", err);

                Errors::new(&[("err", "user already exist")])
            }
            );
    }


    Err(Errors::new(&[("not found", "Article not found")]))
}


#[derive(Deserialize, Default, Clone)]
pub struct UpdateArticleReq {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
    #[serde(skip)]
    pub slug: Option<String>,
    #[serde(rename = "tagList")]
    pub tag_list: Vec<String>,
}

#[put("/articles/<slug>", format = "json", data = "<article_req>")]
pub fn update_articles(slug: String, article_req: Json<UpdateArticleReq>, auth: Auth, state: &State<AppState>, user_service: &State<ServiceState<ArticleService<ArticleRepository>>>) -> Result<Value, Errors> {
    let article = article_req.into_inner();
    let g = &crossbeam::epoch::pin();
    if let shared = user_service.service.load(Ordering::Relaxed, g) {
        if shared.is_null() {
            return Err(Errors::new(&[("email or password", "is invalid")]));
        }
        let t = unsafe { shared.as_ref() };
        let secret = state.secret.clone();
        return t.unwrap().update(&slug, auth.id,
                                 UpdateArticleDataDto {
                                     title: article.title,
                                     description: article.description,
                                     body: article.body,
                                     slug: article.slug,
                                     tag_list: article.tag_list,
                                 },
        ).map(|article| json!(article))
            .map_err(|err| {
                println!("{}", err);

                Errors::new(&[("err", "user already exist")])
            }
            );
    }


    Err(Errors::new(&[("not found", "Article not found")]))
}

#[derive(FromForm, Default)]
pub struct FeedArticlesReq {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[get("/articles/feed?<params..>")]
pub fn get_articles_feed(params: FeedArticlesReq, auth: Auth, state: &State<AppState>, user_service: &State<ServiceState<ArticleService<ArticleRepository>>>) -> Result<Value, Errors> {
    let g = &crossbeam::epoch::pin();
    if let shared = user_service.service.load(Ordering::Relaxed, g) {
        if shared.is_null() {
            return Err(Errors::new(&[("email or password", "is invalid")]));
        }
        let t = unsafe { shared.as_ref() };
        let secret = state.secret.clone();
        return t.unwrap().feed(FeedArticlesDto {
            offset: params.offset,
            limit: params.limit,
        }, auth.id).map(|article| json!({ "articles": article, "articlesCount": article.len() }))
            .map_err(|err| {
                println!("{}", err);

                Errors::new(&[("err", "user already exist")])
            }
            );
    }


    Err(Errors::new(&[("not found", "Article not found")]))
}


#[derive(Deserialize,Validate)]
pub struct NewCommentReq {
    #[validate(length(min = 1))]
   pub  body: Option<String>,
}


#[post("/articles/<slug>/comments", format = "json", data = "<new_comment>")]
pub fn post_comment(slug: String,
                    new_comment: Json<NewCommentReq>,
                    auth: Auth,
                    state: &State<AppState>, user_service: &State<ServiceState<CommentService<CommentRepository>>>) -> Result<Value, Errors> {
    let new_comment = new_comment.into_inner();

    let mut extractor = FieldValidator::validate(&new_comment);
    let body = extractor.extract("body", new_comment.body);
    extractor.check()?;

    let g = &crossbeam::epoch::pin();
    if let shared = user_service.service.load(Ordering::Relaxed, g) {
        if shared.is_null() {
            return Err(Errors::new(&[("email or password", "is invalid")]));
        }
        let t = unsafe { shared.as_ref() };
        let secret = state.secret.clone();
        return t.unwrap().create(auth.id, &slug, &body).map(|article| json!(article))
            .map_err(|err| {
                println!("{}", err);

                Errors::new(&[("err", "user already exist")])
            }
            );
    }


    Err(Errors::new(&[("not found", "Article not found")]))
}



#[get("/articles/<slug>/comments")]
pub fn get_comments(slug: String,
                    auth: Auth,
                    state: &State<AppState>, user_service: &State<ServiceState<CommentService<CommentRepository>>>) -> Result<Value, Errors> {

    let g = &crossbeam::epoch::pin();
    if let shared = user_service.service.load(Ordering::Relaxed, g) {
        if shared.is_null() {
            return Err(Errors::new(&[("email or password", "is invalid")]));
        }
        let t = unsafe { shared.as_ref() };
        let secret = state.secret.clone();
        return t.unwrap().find_by_slug( &slug).map(|article| json!(article))
            .map_err(|err| {
                println!("{}", err);

                Errors::new(&[("err", "user already exist")])
            }
            );
    }


    Err(Errors::new(&[("not found", "Article not found")]))
}


#[delete("/articles/<slug>/comments/<id>")]
pub fn delete_comment(slug: String, id: i32,
                    auth: Auth,
                    state: &State<AppState>, user_service: &State<ServiceState<CommentService<CommentRepository>>>) -> Result<Value, Errors> {

    let g = &crossbeam::epoch::pin();
    if let shared = user_service.service.load(Ordering::Relaxed, g) {
        if shared.is_null() {
            return Err(Errors::new(&[("email or password", "is invalid")]));
        }
        let t = unsafe { shared.as_ref() };
        let secret = state.secret.clone();
        return t.unwrap().delete( auth.id,&slug,id).map(|article| json!(article))
            .map_err(|err| {
                println!("{}", err);

                Errors::new(&[("err", "user already exist")])
            }
            );
    }


    Err(Errors::new(&[("not found", "Article not found")]))
}