
use anyhow::Result;
use bb8::PooledConnection;
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};

use pwhash::bcrypt;

use crate::{models::user::User, repositories::users::UsersRepository};

use super::jwt::decode_jwt_token;

pub async fn hash_password(password: &str) -> String {
    bcrypt::hash(password).expect("Password gets hashed correctly")
}

pub async fn verify_password(password: &str, password_hash: &str) -> bool {
    bcrypt::verify(password, &password_hash)
}

pub async fn authn_token(mut conn: PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>, token: String) -> Result<User> {

    let token_user_data = decode_jwt_token(token)?;

    let user = UsersRepository::find(&mut conn, token_user_data.id).await?;

    Ok(user)
}