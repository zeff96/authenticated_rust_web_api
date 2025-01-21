use actix_web::{
    cookie::{
        time::{Duration, OffsetDateTime},
        Cookie,
    },
    post, web, HttpResponse, Responder,
};
use argon2::PasswordHash;
use serde_json::json;

use crate::{
    model::UserLogin,
    queries::get_user_with_email,
    utils::{generate_access_token, generate_refresh_token, verify_hashed_password},
    AppState,
};

#[post("/login")]
pub async fn user_login_handler(
    state: web::Data<AppState>,
    body: web::Json<UserLogin>,
) -> actix_web::Result<impl Responder> {
    let pool = &state.pool;

    match get_user_with_email(pool, &body.email).await {
        Ok(user) => {
            let db_password = PasswordHash::new(&user.password)
                .map_err(|_| {
                    actix_web::error::ErrorInternalServerError("Error parsing hashed password")
                })?
                .to_string();

            if let Err(_) = verify_hashed_password(&body.password, &db_password) {
                return Err(actix_web::error::ErrorUnauthorized(
                    json!({"error": "Invalid credentials. Please try agian!"}),
                ));
            }

            let access_token =
                generate_access_token(&user.id.to_string(), "secret").map_err(|_| {
                    actix_web::error::ErrorInternalServerError(
                        json!({"error": "Error generating access token!"}),
                    )
                })?;

            let refresh_token =
                generate_refresh_token(&user.id.to_string(), "secret").map_err(|_| {
                    actix_web::error::ErrorInternalServerError(
                        json!({"error": "Error generating access token!"}),
                    )
                })?;

            let refresh_token_cookie = Cookie::build("refresh_token", refresh_token)
                .http_only(true)
                .path("/")
                .expires(OffsetDateTime::now_utc() + Duration::days(365))
                .finish();

            let access_token_cookie = Cookie::build("access_token", access_token)
                .http_only(true)
                .path("/")
                .finish();

            Ok(HttpResponse::Ok()
                .cookie(access_token_cookie)
                .cookie(refresh_token_cookie)
                .json(json!({
                    "status": "success",
                    "message": "Use logged in successfully",
                })))
        }
        Err(_) => {
            return Err(actix_web::error::ErrorNotFound(json!({
                "status": "fail",
                "message": "User with provided email does not exists. Please try again!"
            })))
        }
    }
}

#[post("/logout")]
pub async fn user_logout_handler() -> actix_web::Result<impl Responder> {
    let mut access_cookie = Cookie::build("access_token", "")
        .http_only(true)
        .path("/")
        .finish();
    access_cookie.make_removal();

    let mut refresh_cookie = Cookie::build("refresh_token", "")
        .http_only(true)
        .path("/")
        .expires(OffsetDateTime::now_utc() - Duration::days(1))
        .finish();
    refresh_cookie.make_removal();

    Ok(HttpResponse::Ok()
        .cookie(access_cookie)
        .cookie(refresh_cookie)
        .json(json!({
            "status": "success",
            "message": "User logged out successfully"
        })))
}
