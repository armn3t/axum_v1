use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};
use uuid::Uuid;

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
    delete_test_user(&client, user1);
    delete_test_user(&client, user2);
}

#[test]
fn test_update_users() {
    let client = Client::new();
    let user1 = create_test_user(&client);

    let new_name = "new_name";
    let new_username = format!("new_username_{}", Uuid::new_v4().to_string());

    let res = client.patch(format!("{}/users/{}", URL, user1["id"]))
        .json(&json!({
            "name": new_name,
            "username": new_username,
        }))
        .send().unwrap();
    
    assert_eq!(res.status(), StatusCode::OK);
    let body: Value = res.json().unwrap();

    assert_eq!(body.get("username").unwrap().as_str().unwrap(), new_username);
    assert_eq!(body.get("name").unwrap().as_str().unwrap(), new_name);
    assert_eq!(body.get("id").unwrap().as_str(), user1["id"].as_str());
    delete_test_user(&client, user1);
}
