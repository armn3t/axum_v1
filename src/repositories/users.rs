use diesel::prelude::*;
use diesel::QueryResult;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use bb8::PooledConnection;

use crate::lib::auth;
use crate::models::user::{NewUserFields, UpdatableFieldsUser};
use crate::models::user::{NewUserInput, User};

use crate::schema::users;

pub struct UsersRepository;

impl UsersRepository {
    pub async fn find_multiple(
        conn: &mut PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>,
    ) -> QueryResult<Vec<User>> {
        let users = users::table.limit(100).load(conn).await?;
        Ok(users)
    }

    pub async fn find(
        conn: &mut PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>,
        id: i32,
    ) -> QueryResult<User> {
        let user = users::table.find(id).get_result(conn).await?;
        Ok(user)
    }
    
    pub async fn find_by_username(
        conn: &mut PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>,
        username: &str,
    ) -> Option<User> {
        match diesel::QueryDsl::filter(users::dsl::users, users::username.eq(username))
            .first::<User>(conn).await {
                Ok(user) => {
                    Some(user)
                },
                Err(err) => {
                    println!("No such user: {}", username);
                    None
                }
            }
    }

    pub async fn create(
        conn: &mut PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>,
        new_user: NewUserInput,
    ) -> QueryResult<User> {
        let NewUserInput { name, username, password } = new_user;
        let new_user_fields = NewUserFields {
            name,
            username,
            password: auth::hash_password(&password),
            api_token: Uuid::new_v4().to_string(),
        };
        diesel::insert_into(users::table)
            .values(new_user_fields)
            .get_result(conn)
            .await
    }

    pub async fn update(
        conn: &mut PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>,
        id: i32,
        user: UpdatableFieldsUser,
    ) -> QueryResult<User> {
        // user.password = Some(format!("hash_pretend_{}", user.password));
        diesel::update(users::table.find(id))
            .set(user)
            .get_result(conn)
            .await
    }

    pub async fn delete(
        conn: &mut PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>,
        id: i32,
    ) -> QueryResult<usize> {
        diesel::delete(users::table.find(id))
            .execute(conn)
            .await
    }
}
