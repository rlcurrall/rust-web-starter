use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::error::Result;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub age: i32,
    pub active: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewUser {
    pub email: String,
    pub name: String,
    pub age: i32,
    pub active: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUser {
    pub email: String,
    pub name: String,
    pub age: i32,
    pub active: bool,
}

pub struct UsersQuery {
    pub active: Option<bool>,
    pub age_gt: Option<i32>,
    pub age_lt: Option<i32>,
}

pub async fn get_count(conn: &PgPool, query: UsersQuery) -> Result<i64> {
    let res: i64 = sqlx::query!(
        "SELECT COUNT(1) AS count FROM users
        WHERE ($1 OR active = $2) AND ($3 OR age > $4) AND ($5 OR age < $6)",
        query.active.is_none(),
        query.active.unwrap_or(true),
        query.age_gt.is_none(),
        query.age_gt.unwrap_or(0),
        query.age_lt.is_none(),
        query.age_lt.unwrap_or(0),
    )
    .fetch_one(conn)
    .await?
    .count
    .unwrap_or(0);

    Ok(res)
}

pub async fn get_by_id(user_id: i32, conn: &PgPool) -> Result<User> {
    let res = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(conn)
        .await?;

    Ok(res)
}

pub async fn get_all(
    conn: &PgPool,
    page: i64,
    per_page: i64,
    query: UsersQuery,
) -> Result<Vec<User>> {
    let offset = (page - 1) * per_page;

    let res = sqlx::query_as!(
        User,
        "SELECT * FROM users
        WHERE ($1 OR active = $2) AND ($3 OR age > $4) AND ($5 OR age < $6)
        LIMIT $7 OFFSET $8",
        query.active.is_none(),
        query.active.unwrap_or(true),
        query.age_gt.is_none(),
        query.age_gt.unwrap_or(0),
        query.age_lt.is_none(),
        query.age_lt.unwrap_or(0),
        per_page,
        offset
    )
    .fetch_all(conn)
    .await?;

    Ok(res)
}

pub async fn create(new_user: NewUser, conn: &PgPool) -> Result<User> {
    let res = sqlx::query_as!(
        User,
        "INSERT INTO users (name, email, age, active) VALUES ($1, $2, $3, $4) RETURNING *",
        new_user.name,
        new_user.email,
        new_user.age,
        new_user.active,
    )
    .fetch_one(conn)
    .await?;

    Ok(res)
}

pub async fn update(user_id: i32, change: UpdateUser, conn: &PgPool) -> Result<User> {
    let res = sqlx::query_as!(
        User,
        "UPDATE users SET name = $1, email = $2, age = $3, active = $4 WHERE id = $5 RETURNING *",
        change.name,
        change.email,
        change.age,
        change.active,
        user_id
    )
    .fetch_one(conn)
    .await?;

    Ok(res)
}

pub async fn delete(user_id: i32, conn: &PgPool) -> Result<()> {
    sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
        .execute(conn)
        .await?;

    Ok(())
}
