use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::Executor;
use std::env;

use blog::config;
use blog::AppState;

pub async fn create_run_migrations(
    database_url: &str,
    database_name: &str,
) -> Result<(), sqlx::Error> {
    let postgres_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let db_exist: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname = $1)")
            .bind(database_name)
            .fetch_one(&postgres_pool)
            .await?;

    if !db_exist {
        let create_db_query = format!("CREATE DATABASE {}", database_name);
        postgres_pool.execute(&*create_db_query).await?;
    }

    let target_database_url = format!("{}/{}", database_url, database_name);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&target_database_url)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "actix-web=info");
    }

    env_logger::init();

    let database_name = "blog";

    let database_url = env::var("DATABASE_URL").expect("Database url must be set");

    create_run_migrations(&database_url, database_name)
        .await
        .expect("Database setup failed");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Database connection error");

    let app_state = web::Data::new(AppState { pool });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .wrap(cors)
            .configure(config)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
