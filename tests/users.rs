use diesel::row::NamedRow;
use rocket::http::{ContentType, Status};
use rocket::local::blocking::LocalResponse;
use crate::common::test_client;

mod common;
use common::{*};

#[test]
/// Register new user, handling repeated registration as well.
fn test_register() {
    let client = test_client().lock().unwrap();
    let response = client
        .post("/api/users/register")
        .header(ContentType::JSON)
        .body(json_string!({"username": USERNAME, "email": EMAIL, "password": PASSWORD}))
        .dispatch();


    let status = response.status();

    // If user was already created we should get an UnprocessableEntity or Ok otherwise.
    //
    // As tests are ran in an indepent order `login()` probably has already created smoketest user.
    // And so we gracefully handle "user already exists" error here.
    if status == Status::Ok {
        check_user_response(response);
    } else if status == Status::UnprocessableEntity {
        check_user_validation_errors(response);
    } else {
        panic!("Got status: {}", status);
    }

}

#[test]
/// Registration with the same email must fail
fn test_register_with_duplicated_email() {
    let client = test_client().lock().unwrap();
    register(&client, "clone", "clone@realworld.io", PASSWORD);

    let response = client
        .post("/api/users/register")
        .header(ContentType::JSON)
        .body(json_string!({

                "username": "clone_1",
                "email": "clone@realworld.io",
                "password": PASSWORD,

        }))
        .dispatch();

    assert_eq!(response.status(), Status::UnprocessableEntity);

    let value = response_json_value(response);
    let error = value
        .get("errors")
        .and_then(|errors| errors.get("err"))
        .and_then(|errors| errors.get(0))
        .and_then(|error| error.as_str());

    assert_eq!(error, Some("user already exist"));
}
#[test]
/// Try logging checking that access Token is present.
fn test_login() {
    let client = test_client().lock().unwrap();
    let response = client
        .post("/api/users/login")
        .header(ContentType::JSON)
        .body(json_string!( {"email": EMAIL, "password": PASSWORD}))
        .dispatch();

    let value = response_json_value(response);
    value

        .get("token")
        .expect("user has token")
        .as_str()
        .expect("token must be a string");
}


#[test]
/// Login with wrong password must fail.
fn test_incorrect_login() {
    let client = test_client().lock().unwrap();
    let response = client
        .post("/api/users/login")
        .header(ContentType::JSON)
        .body(json_string!( {"email": EMAIL, "password": "foo"}))
        .dispatch();

    assert_eq!(response.status(), Status::UnprocessableEntity);

    let value = response_json_value(response);
    let login_error = value
        .get("errors")
        .expect("must have a 'errors' field")
        .get("email or password")
        .expect("must have 'email or password' errors")
        .get(0)
        .expect("must have non empty 'email or password' errors")
        .as_str();

    assert_eq!(login_error, Some("is invalid"));
}

fn check_user_validation_errors(response: LocalResponse) {
    let value = response_json_value(response);
    let username_error = value
        .get("errors")
        .expect("must have a 'errors' field")
        .get("username")
        .expect("must have 'username' errors")
        .get(0)
        .expect("must have non-empty 'username' errors")
        .as_str();

    assert_eq!(username_error, Some("has already been taken"))
}


/// Assert that body contains "user" response with expected fields.
fn check_user_response(response: LocalResponse) {
    let user = response_json_value(response);
    dbg!(&user);


    assert!(user.get("email").is_some());
    assert!(user.get("username").is_some());
    assert!(user.get("bio").is_some());
    assert!(user.get("image").is_some());
    assert!(user.get("token").is_some());
}



#[test]
/// Check that `/user` endpoint returns expected data.
fn test_get_user() {
    let client = test_client().lock().unwrap();
    let token = login(&client);
    dbg!(&token);
    let response = client
        .get("/api/user")
        .header(token_header(token))
        .dispatch();

    check_user_response(response);
}