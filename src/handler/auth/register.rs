use actix_web::{post, web, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;

use crate::{
    model::UserRegistration, queries::user_registration, utils::generate_hash_password, AppState,
};

#[post("/register")]
pub async fn user_registration_handler(
    state: web::Data<AppState>,
    body: web::Json<UserRegistration>,
) -> actix_web::Result<impl Responder> {
    let pool = &state.pool;

    let id = Uuid::new_v4();
    let hashed_password = generate_hash_password(&body.password).map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Failed to hash password: {}", e))
    })?;

    match user_registration(pool, &id, &body.name, &body.email, &hashed_password).await {
        Ok(user) => Ok(HttpResponse::Created().json(json!({
            "status": "success",
            "message": "User created successfully!",
            "user": user
        }))),
        Err(err) => {
            if let Some(pg_error) = err.as_database_error() {
                if pg_error.is_unique_violation() {
                    return Err(actix_web::error::ErrorConflict(json!({
                    "status": "fail",
                    "message": "Email already exists. Please try again!"
                    })));
                }
            }
            Err(actix_web::error::ErrorInternalServerError(json!({
            "status": "fail",
            "message": format!("Database error: {}", err)
            })))
        }
    }
}
