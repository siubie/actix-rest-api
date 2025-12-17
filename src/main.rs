mod config;
mod db;
mod dto;
mod errors;
mod handlers;
mod models;
mod routes;
mod services;

use actix_cors::Cors;
use actix_web::{middleware::{Logger, NormalizePath, TrailingSlash}, web, App, HttpServer};
use config::database::DatabaseConfig;
use dotenv::dotenv;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::health::health_check,
        handlers::user::create_user,
        handlers::user::get_user,
        handlers::user::get_all_users,
        handlers::user::update_user,
        handlers::user::delete_user,
    ),
    components(
        schemas(
            handlers::health::HealthResponse,
            dto::user_dto::CreateUserRequest,
            dto::user_dto::UpdateUserRequest,
            dto::user_dto::UserResponse,
            models::user::User,
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Users", description = "User management endpoints")
    ),
    info(
        title = "Actix-web API",
        version = "1.0.0",
        description = "A simple REST API built with Actix-web, MySQL, and OpenAPI"
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("{}:{}", host, port);

    log::info!("Connecting to database...");
    let db_config = DatabaseConfig::new(&database_url)
        .await
        .expect("Failed to create database connection pool");

    log::info!("Starting server at http://{}", bind_address);
    log::info!("Swagger UI available at http://{}/swagger-ui/", bind_address);

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .app_data(web::Data::new(db_config.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .configure(routes::config)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind(&bind_address)?
    .run()
    .await
}
