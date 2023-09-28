use diesel::prelude::*;
use diesel::{result::Error, QueryResult};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use bb8::{Pool, PooledConnection};

use crate::models::user::{NewUser, User};

use crate::schema;

pub struct UsersRepository;

impl UsersRepository {
    pub async fn find_multiple(
        conn: &mut PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>,
    ) -> QueryResult<Vec<User>> {
        let users = schema::users::table.limit(100).load(conn).await?;
        Ok(users)
    }

    pub async fn find(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<User> {
        let user = schema::users::table.find(id).get_result(conn).await?;
        Ok(user)
    }

    // pub fn find_multiple(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<User>> {
    //     users::table.limit(limit).load(c)
    // }

    // pub fn create(c: &mut AsyncPgConnection, new_rustacean: NewUser) -> QueryResult<User> {
    //     diesel::insert_into(users::table)
    //         .values(new_rustacean)
    //         .get_result(c)
    //         .await
    // }

    // pub fn update(c: &mut AsyncPgConnection, id: i32, user: User) -> QueryResult<User> {
    //     diesel::update(users::table.find(id))
    //         .set((users::name.eq(user.name), users::username.eq(user.username)))
    //         .get_result(c)
    // }

    // pub fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
    //     diesel::delete(users::table.find(id)).execute(c)
    // }
}
