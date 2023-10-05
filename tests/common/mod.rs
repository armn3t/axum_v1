use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};
use uuid::Uuid;

pub static URL: &'static str = "http://127.0.0.1:5005";

fn create_counter() -> impl FnMut() -> i32 {
    let mut counter: i32 = 0;
    return move || -> i32 {
        counter += 1;
        return counter;
    };
}

// static A = create_counter();

pub fn create_test_user(client: &Client) -> Value {
    let username = format!("foo_{}", Uuid::new_v4());

    let res = client
        .post(format!("{}/users", URL))
        .json(&json!({
          "name": "bar",
          "username": username
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

// pub fn create_test_crate(client: &Client, rustacean: Value) -> Value {
//     let res = client
//         .post(format!("{}/crates", URL))
//         .json(&json!({
//             "rustacean_id": rustacean["id"],
//             "code": "foo",
//             "name": "foo crate",
//             "version": "0.1",
//             "description": "Some basic description"
//         }))
//         .send()
//         .unwrap();

//     assert_eq!(res.status(), StatusCode::CREATED);

//     res.json().unwrap()
// }

// pub fn delete_test_crate(client: &Client, create: Value) {}
