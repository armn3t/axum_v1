use axum:: {
    extract::State,
    http::{HeaderMap, StatusCode},
    response::Json, Extension,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{
    get_conn,
    libs::jwt::create_jwt_header, models::user::{NewUserInput, User},
    repositories::users::UsersRepository,
    AppStateType
};
use crate::libs::{auth, jwt::{TokenUserData, create_jwt_token, AUTHORIZATION}};


#[derive(Deserialize, Debug)]
pub struct AuthUser {
    username: String,
    password: String,
}

pub async fn login(State(state): AppStateType, Json(payload): Json<AuthUser>) -> (StatusCode, HeaderMap, Json<Value>) {
    let mut conn = get_conn(&state.pool).await;
    println!("AUTH: {:?}", payload);
    let user_opt: Option<User> = UsersRepository::find_by_username(&mut conn, &payload.username).await;

    let mut headers = HeaderMap::new();
    match user_opt {
        Some(user) => {
            println!("CHECK: {} - {}", user.password, auth::hash_password(&payload.password));
            if user.password != auth::hash_password(&payload.password) {
                return (StatusCode::UNAUTHORIZED, headers, Json(json!({"message": "unauth"})));
            }

            let token = create_jwt_token(TokenUserData { id: user.id });

            headers.insert(AUTHORIZATION, create_jwt_header(token).parse().unwrap());

            (StatusCode::OK, headers, Json(json!({"user": user})))
        },
        None => {
            
            (StatusCode::NOT_FOUND, headers, Json(json!({"message": "User not found"})))
        }
    }
}

pub async fn authenticated(Extension(user): Extension<User>) -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({"message": "authenticated", "username": user.username})))
}

pub async fn register(State(state): AppStateType, Json(payload): Json<NewUserInput>) -> (StatusCode, HeaderMap, Json<Value>) {
    let mut conn = get_conn(&state.pool).await;

    let user_existing = UsersRepository::find_by_username(&mut conn, &payload.username).await;

    let mut headers = HeaderMap::new();

    if user_existing.is_some() {
        return (StatusCode::BAD_REQUEST, headers, Json(json!({"message": "username unavailable"})));
    }

    let user = UsersRepository::create(&mut conn, payload).await.unwrap();

    let token = create_jwt_token(TokenUserData { id: user.id });

    headers.insert(AUTHORIZATION, create_jwt_header(token).parse().unwrap());

    (StatusCode::OK, headers, Json(json!(user)))
}