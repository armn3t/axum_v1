use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};
use uuid::Uuid;

pub static URL: &'static str = "http://127.0.0.1:5005";

pub const DEFAULT_PASSWORD: &str = "secret";

pub fn create_test_user(client: &Client) -> Value {
    let username = format!("foo_{}", Uuid::new_v4());

    let res = client
        .post(format!("{}/users", URL))
        .json(&json!({
          "name": "bar",
          "username": username,
          "password": DEFAULT_PASSWORD,
        }))
        .send()
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);
    res.json().unwrap()
}

pub fn delete_test_user(client: &Client, user: Value) {
    let res = client
        .delete(format!("{}/users/{}", URL, user["id"]))
        .send()
        .unwrap();
    println!("delete url: {}", format!("{}/users/{}", URL, user["id"]));
    assert_eq!(res.status(), StatusCode::NO_CONTENT)
}
