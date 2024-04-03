use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;
use common::{create_test_user, delete_test_user, URL};

#[test]
fn test_get_users() {
    let client = Client::new();

    let user1 = create_test_user(&client);
    let user2 = create_test_user(&client);

    let res = client.get(format!("{}/users", URL)).send().unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let body: Value = res.json().unwrap();
    let users = body.get("users").unwrap();

    println!("USERS IN TEST: {}", users);

    assert!(users.as_array().unwrap().len() > 0);
    assert!(users.as_array().unwrap().contains(&user1));
    assert!(users.as_array().unwrap().contains(&user2));

    //Cleanup
    // delete_test_user(&client, user1);
    // delete_test_user(&client, user2);
}
