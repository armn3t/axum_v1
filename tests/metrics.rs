use reqwest::{blocking::Client, StatusCode};

pub mod common;
use common::URL;

#[test]
fn test_metrics_endpoint() {
    let client = Client::new();

    let res = client.get(format!("{}/metrics", URL)).send().unwrap();

    assert_eq!(StatusCode::OK, res.status());
}