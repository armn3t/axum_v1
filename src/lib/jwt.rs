
use axum::http::HeaderValue;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use chrono;
use serde::{Serialize, Deserialize};
use serde_json::{from_str, json};
use anyhow::{bail, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenUserData {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // aud: String,         // Optional. Audience
    exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize,          // Optional. Issued at (as UTC timestamp)
    // iss: String,         // Optional. Issuer
    // nbf: usize,          // Optional. Not Before (as UTC timestamp)
    sub: String,         // Optional. Subject (whom token refers to)
}

pub const AUTHORIZATION: &str = "authorization";
pub const BEARER: &str = "Bearer ";

pub fn create_jwt_token(token_data: TokenUserData) -> String {

    let now = chrono::Utc::now();
    let my_claims = Claims {
        iat: now.timestamp() as usize,
        exp: (now + chrono::Duration::minutes(60)).timestamp() as usize,
        sub: json!(token_data).to_string()
    };

    encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref())).unwrap()
}

pub fn decode_jwt_token(token: String) -> Result<TokenUserData> {
    let decoded = decode::<Claims>(
        &token, 
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS256));

    match decoded {
        Ok(token_data) => {
            Ok(from_str::<TokenUserData>(token_data.claims.sub.as_str())?)
        },
        Err(err) => {
            println!("Error decoding jwt token: {:?}", err);
            bail!("Error decoding jwt token")
        }
    }
}

pub fn create_jwt_header(token: String) -> String {
    format!("{}{}", BEARER, token)
}

pub fn get_jwt_token(header: &HeaderValue) -> anyhow::Result<String> {
    let header_val = header.to_str()?;
    println!("HEADER: {} - {}", header_val.trim(), header_val.contains("Bearer"));
    if !header_val.trim().starts_with(BEARER) {
        // both work
        // return Err(anyhow::anyhow!("Not a Bearer token"))
        anyhow::bail!("Not a Bearer token")
    }

    match header_val.split_once(" ") {
        Some((_, token)) => {
            Ok(token.to_string())
        },
        None => anyhow::bail!("Not a proper Bearer token") 
    }
}
