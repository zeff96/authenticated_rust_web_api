use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
//App state
pub struct AppState {
    pub pool: PgPool,
}

// Token claim
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Claim {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

// User model
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}

//User registration model
#[derive(Debug, Deserialize, Serialize)]
pub struct UserRegistration {
    pub name: String,
    pub email: String,
    pub password: String,
}

//User login model
#[derive(Debug, Deserialize, Serialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

//User response model
#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

// Post model
#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// Struct for creating new Post
#[derive(Debug, Deserialize, Serialize)]
pub struct NewPost {
    pub title: String,
    pub content: String,
}

// Struct for updating existing Post
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatePost {
    pub title: Option<String>,
    pub content: Option<String>,
}
