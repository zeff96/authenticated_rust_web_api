use actix_web::{middleware::from_fn, web};
use handler::{
    auth::{
        authenticate::{user_login_handler, user_logout_handler},
        register::user_registration_handler,
    },
    generic::health_checker_handler,
    posts::{create_post_handler, delete_post_handler, edit_post_handler, get_posts_handler},
};
use middleware::jwt_middleware;

mod handler;
mod middleware;
mod model;
mod queries;
mod utils;
pub use model::AppState;

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(
        web::scope("/api/auth")
            .service(user_registration_handler)
            .service(user_login_handler)
            .service(user_logout_handler),
    );
    conf.service(
        web::scope("/api")
            .wrap(from_fn(jwt_middleware))
            .service(health_checker_handler)
            .service(get_posts_handler)
            .service(create_post_handler)
            .service(edit_post_handler)
            .service(delete_post_handler),
    );
}
