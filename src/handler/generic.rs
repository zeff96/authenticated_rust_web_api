use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

#[get("/healthchecker")]
pub async fn health_checker_handler() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Hello and welcome to actix web and sqlx"
    }))
}
