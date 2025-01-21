use chrono::{DateTime, Utc};
use sqlx::{postgres::PgQueryResult, PgPool};
use uuid::Uuid;

use crate::model::{Post, User, UserResponse};

//insert user into the database
pub async fn user_registration(
    pool: &PgPool,
    id: &Uuid,
    name: &str,
    email: &str,
    password: &str,
) -> sqlx::error::Result<UserResponse> {
    sqlx::query_as!(
        UserResponse,
        r#"
            INSERT INTO users(id, name, email, password)
            VALUES($1, $2, $3, $4)
            RETURNING id, name, email
        "#,
        id,
        name,
        email,
        password
    )
    .fetch_one(pool)
    .await
}

//query user from database for authentication
pub async fn get_user_with_email(pool: &PgPool, email: &str) -> sqlx::Result<User> {
    sqlx::query_as!(
        User,
        r#"
            SELECT id, name, email, password FROM users
            WHERE email = $1
        "#,
        email
    )
    .fetch_one(pool)
    .await
}

// get all posts from db
pub async fn get_posts(pool: &PgPool) -> sqlx::Result<Vec<Post>> {
    sqlx::query_as!(
        Post,
        r#"
            SELECT id, user_id, title, content, created_at, updated_at
            FROM posts
        "#,
    )
    .fetch_all(pool)
    .await
}

// Insert new created post into the database
pub async fn create_post(
    pool: &PgPool,
    id: &Uuid,
    user_id: &Uuid,
    title: &str,
    content: &str,
    created_at: &DateTime<Utc>,
    updated_at: &DateTime<Utc>,
) -> sqlx::Result<Post> {
    sqlx::query_as!(
        Post,
        r#"
            INSERT INTO posts(id, user_id, title, content, created_at, updated_at)
            VALUES($1, $2, $3, $4, $5, $6)
            RETURNING id, user_id, title, content, created_at, updated_at
        "#,
        id,
        user_id,
        title,
        content,
        created_at,
        updated_at
    )
    .fetch_one(pool)
    .await
}

// Update a given existing post
pub async fn update_post(
    pool: &PgPool,
    title: Option<&str>,
    content: Option<&str>,
    id: &Uuid,
) -> sqlx::Result<Post> {
    sqlx::query_as!(
        Post,
        r#"
            UPDATE posts
            SET
                title = COALESCE($1, title),
                content = COALESCE($2, content)
            WHERE id = $3
            RETURNING id, user_id, title, content, created_at, updated_at
        "#,
        title,
        content,
        id
    )
    .fetch_one(pool)
    .await
}

// Delete a post with a given id
pub async fn delete_post(pool: &PgPool, id: &Uuid) -> sqlx::Result<PgQueryResult> {
    sqlx::query!("DELETE FROM posts where id = $1", id)
        .execute(pool)
        .await
}
