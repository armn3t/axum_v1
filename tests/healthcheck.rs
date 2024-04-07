use reqwest::{blocking::Client, StatusCode};

pub mod common;
use common::URL;

#[test]
fn test_healthcheck() {
    let client = Client::new();

    let res = client.get(format!("{}/healthcheck", URL)).send().unwrap();

    assert_eq!(res.status(), StatusCode::OK);
}