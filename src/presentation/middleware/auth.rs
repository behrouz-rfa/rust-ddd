//! Auth work as middleware
//! we impl FromRequest for Auth to check is user has valid jwt token or not

use rocket::serde::{Serialize,Deserialize};
use jsonwebtoken as jwt;
use jwt::{DecodingKey,EncodingKey};
use rocket::outcome::Outcome;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use crate::presentation::config;
use crate::presentation::config::AppState;

/// Auth use keep exportion date id of user and username
#[derive(Debug,Deserialize,Serialize)]
pub struct Auth {
    pub exp: i64,
    pub id: i32,
    pub username: String,
}

impl Auth {
    /// this fn use to encode user info to JWT token
    pub fn token(&self,secret:&[u8])-> String {
        let encoded_key = EncodingKey::from_base64_secret(std::str::from_utf8(secret).unwrap());
        jwt::encode(&jwt::Header::default(),self,&encoded_key.unwrap()).expect("jwt")
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = ();
    /// Extract Auth token from the "Authorization" header.
    ///
    /// Handlers with Auth guard will fail with 503 error.
    /// Handlers with Option<Auth> will be called with None.
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Auth, Self::Error> {
        let state = req.rocket().state::<AppState>().unwrap();
        if let Some(auth) = extract_auth_from_request(req, &state.secret) {
            Outcome::Success(auth)
        } else {
            Outcome::Failure((Status::Forbidden, ()))
        }
    }
}

/// extract the authorization header fro m user `Request`
fn extract_auth_from_request(request: &Request,secret:&[u8])->Option<Auth>{
    request
        .headers()
        .get_one("authorization")
        .and_then(extract_token_from_header)
        .and_then(|token| decode_token(token, secret))
}
/// extract the authorization token  from   `header: &str`
fn extract_token_from_header(header: &str) -> Option<&str> {

    if header.starts_with(config::TOKEN_PREFIX) {
        Some(&header[config::TOKEN_PREFIX.len()..])
    } else {
        None
    }
}

/// Decode token into `Auth` struct. If any error is encountered, log it
/// an return None.
fn decode_token(token: &str, secret: &[u8]) -> Option<Auth> {
    use jwt::{Algorithm, Validation};

    let decoding_key = DecodingKey::from_base64_secret(std::str::from_utf8(secret).unwrap());

    jwt::decode(
        token,
        &decoding_key.unwrap(),
        &Validation::new(Algorithm::HS256),
    )
        .map_err(|err| {
            eprintln!("Auth decode error: {:?}", err);
        })
        .ok()
        .map(|token_data| token_data.claims)
}
