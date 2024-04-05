use std::mem;

use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;
use common::{create_test_user, delete_test_user, URL, DEFAULT_PASSWORD};

#[test]
fn test_auth_user() {
    let client = Client::new();

    let user = create_test_user(&client);

    let mut res = client.post(get_login_url())
        .json(&json!({
            "username": user["username"],
            "password": DEFAULT_PASSWORD,
        })
        )
        .send().unwrap();

    let status = res.status();
    assert_eq!(status, StatusCode::OK);
    

    let headers = std::mem::take(res.headers_mut());
    let body: Value = res.json().unwrap();
    
    let res_user = body.get("user").unwrap();
    assert_eq!(res_user.get("username"), user.get("username"));

    assert_eq!(headers.get("authorization").is_some(), true);

    delete_test_user(&client, user);
}

#[test]
fn test_failed_auth_user() {
    let client = Client::new();

    let user = create_test_user(&client);

    let res = client.post(get_login_url())
        .json(&json!({
            "username": user["username"],
            "password": "big_nope",
        }))
        .send().unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
    delete_test_user(&client, user);
}

#[test]
fn test_authenticated_user_without_header() {
    let client = Client::new();

    let res = client.post(get_authenticated_url())
        .send()
        .unwrap();

    assert_eq!(StatusCode::UNAUTHORIZED, res.status());
}

#[test]
fn test_authenticated_user_invalid_header() {
    let client = Client::new();

    let res = client.post(get_authenticated_url())
        .header("authorization", "asd asd")
        .send()
        .unwrap();

    assert_eq!(StatusCode::UNAUTHORIZED, res.status());
}

#[test]
fn test_authenticated_user_invalid_bearer_token() {
    let client = Client::new();

    let res = client.post(get_authenticated_url())
        .header("authorization", "Bearer asd")
        .send()
        .unwrap();

    assert_eq!(StatusCode::UNAUTHORIZED, res.status());
}

#[test]
fn test_authenticated_user() {
    let client = Client::new();

    let user = create_test_user(&client);

    let res = client.post(get_login_url())
        .json(&json!({
            "username": user["username"],
            "password": DEFAULT_PASSWORD,
        }))
        .send().unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let headers = res.headers();

    let auth_header = headers.get("authorization").unwrap().to_str().unwrap();

    println!("AUTH HEADER: {}", auth_header);
    let auth_res = client.post(get_authenticated_url())
        .header("authorization", auth_header.to_owned())
        .send().unwrap();

    assert_eq!(StatusCode::OK, auth_res.status());

    delete_test_user(&client, user);
}

fn get_authenticated_url() -> String {
    format!("{}/authenticated", URL)
}

fn get_login_url() -> String {
    format!("{}/login", URL)
}