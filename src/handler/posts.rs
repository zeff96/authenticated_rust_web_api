use actix_web::web::ReqData;
use actix_web::{delete, error, get, patch, post, web, HttpResponse, Responder};
use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

use crate::model::{Claim, NewPost, UpdatePost};
use crate::queries::{create_post, delete_post, get_posts, update_post};
use crate::utils::parse_uuid;
use crate::AppState;

//retrive posts from db

#[get("/posts")]
pub async fn get_posts_handler(state: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let pool = &state.pool;

    match get_posts(pool).await {
        Ok(posts) => Ok(HttpResponse::Ok().json(json!({
            "stataus": "success",
            "result": posts.len(),
            "posts": posts
        }))),
        Err(e) => Err(error::ErrorInternalServerError(json!({
            "status": "fail",
            "message": format!("Failed to retrieve posts: {}", e)
        }))),
    }
}

// Create post and persist on the db
#[post("/posts")]
pub async fn create_post_handler(
    state: web::Data<AppState>,
    body: web::Json<NewPost>,
    req: Option<ReqData<Claim>>,
) -> actix_web::Result<impl Responder> {
    let pool = &state.pool;

    let id = Uuid::new_v4();
    let user_id = if let Some(token) = req {
        parse_uuid(&token.sub)?
    } else {
        return Err(error::ErrorNotFound("No claims found in the request data"));
    };
    let created_at = Utc::now();
    let updated_at = Utc::now();

    match create_post(
        pool,
        &id,
        &user_id,
        &body.title,
        &body.content,
        &created_at,
        &updated_at,
    )
    .await
    {
        Ok(post) => Ok(HttpResponse::Created().json(json!({
            "status": "success",
            "post": post
        }))),
        Err(e) => {
            if let Some(pg_error) = e.as_database_error() {
                if pg_error.is_unique_violation() {
                    return Err(error::ErrorConflict(json!({
                        "status": "fail",
                        "message": "Post with given title already exists!"
                    })));
                }
            }
            Err(error::ErrorInternalServerError(json!({
                "status": "fail",
                "message": format!("Error from database: {}", e)
            })))
        }
    }
}

// Update post with a given id
#[patch("/posts/{id}")]
pub async fn edit_post_handler(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdatePost>,
) -> actix_web::Result<impl Responder> {
    let pool = &state.pool;
    let id = path.into_inner();
    match update_post(pool, body.title.as_deref(), body.content.as_deref(), &id).await {
        Ok(post) => Ok(HttpResponse::Ok().json(json!({
            "status": "success",
            "post": post
        }))),
        Err(e) => match e {
            sqlx::error::Error::RowNotFound => {
                return Err(error::ErrorNotFound(json!({
                    "status": "fail",
                    "message": "Post with given id not found!"
                })))
            }
            _ => Err(error::ErrorInternalServerError(json!({
                "status": "fail",
                "message": format!("Error from database: {}", e)
            }))),
        },
    }
}

//Delete post with a given id
#[delete("/posts/{id}")]
pub async fn delete_post_handler(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let pool = &state.pool;
    let id = path.into_inner();

    match delete_post(pool, &id).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => match e {
            sqlx::error::Error::RowNotFound => {
                return Err(error::ErrorNotFound(json!({
                    "status": "fail",
                    "message": "Post with given id not found!"
                })))
            }
            _ => Err(error::ErrorInternalServerError(json!({
                "status": "fail",
                "message": format!("Error from database: {}", e)
            }))),
        },
    }
}
