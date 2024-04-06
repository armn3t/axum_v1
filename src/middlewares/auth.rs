use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Json, Response},
    extract::State,
};

use serde_json::json;

use crate::{get_conn, libs::{auth::authn_token, jwt::{get_jwt_token, AUTHORIZATION}}, AppStateType};

pub async fn get_token(mut req: Request<Body>, next: Next) -> Response {
    println!("GET TOKEN MIDDLEWARE {:?}", req.headers());
    req.extensions_mut().insert("whatever");
    next.run(req).await
}

pub async fn is_authenticated(State(state): AppStateType, mut req: Request<Body>, next: Next) -> Response {
    let auth_header = req.headers().get(AUTHORIZATION);

    println!("IS AUTHENTICATED MW");

    let unauth_res = Json(json!({"message": "unauthorized"}));

    match auth_header {
        Some(header) => {
            
            match get_jwt_token(header) {
                Ok(token) => {
                    println!("TOKEN IN AUTH MW: {}", token);

                    let conn = get_conn(&state.pool).await;
                    let authn_result = authn_token(conn, token).await;

                    match authn_result {
                        Ok(user) => {
                            req.extensions_mut().insert(user);
                            next.run(req).await
                        },
                        Err(err) => {
                            println!("User error: {}", err);
                            return (StatusCode::UNAUTHORIZED, unauth_res).into_response();   
                        }
                    }
                },
                Err(err) => {
                    println!("Error decoding auth header: {}", err);
                    (StatusCode::UNAUTHORIZED, unauth_res).into_response()
                }
            }
        },
        None => {
            println!("HEADER NOT PROVIDED");
            (StatusCode::UNAUTHORIZED, Json(json!({"message": "unauthorized"}))).into_response()
        }
    }
}