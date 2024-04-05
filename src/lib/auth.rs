
use anyhow::{Result, bail};
use bb8::PooledConnection;
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};

use crate::{models::user::User, repositories::users::UsersRepository};

use super::jwt::decode_jwt_token;

pub fn hash_password(password: &str) -> String {
    return format!("hash_pretend_{}", password);
}

pub async fn authn_token(mut conn: PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>, token: String) -> Result<User> {

    let token_user_data = decode_jwt_token(token)?;

    println!("TOKEN USER DATA {:?}", token_user_data);    


    let user = UsersRepository::find(&mut conn, token_user_data.id).await?;

    println!("USER: {:?}", user);
    Ok(user)
}