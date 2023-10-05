use diesel::prelude::*;
use diesel::{result::Error, QueryResult};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use bb8::{Pool, PooledConnection};

use crate::models::user::{NewUser, User};

use crate::schema;

// use crate::PoolConn;

pub struct UsersRepository;

impl UsersRepository {
    pub async fn find_multiple(
        conn: &mut PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>,
    ) -> QueryResult<Vec<User>> {
        let users = schema::users::table.limit(100).load(conn).await?;
        Ok(users)
    }

    pub async fn find(
        conn: &mut PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>,
        id: i32,
    ) -> QueryResult<User> {
        let user = schema::users::table.find(id).get_result(conn).await?;
        Ok(user)
    }

    pub async fn create(
        conn: &mut PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>,
        new_user: NewUser,
    ) -> QueryResult<User> {
        diesel::insert_into(schema::users::table)
            .values(new_user)
            .get_result(conn)
            .await
    }

    pub async fn update(
        conn: &mut PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>,
        id: i32,
        user: User,
    ) -> QueryResult<User> {
        diesel::update(schema::users::table.find(id))
            .set((
                schema::users::name.eq(user.name),
                schema::users::username.eq(user.username),
            ))
            .get_result(conn)
            .await
    }

    pub async fn delete(
        conn: &mut PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>,
        id: i32,
    ) -> QueryResult<usize> {
        diesel::delete(schema::users::table.find(id))
            .execute(conn)
            .await
    }
}
