use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use validator::Validate;
use crate::config::database::DatabaseConfig;
use crate::dto::user_dto::{CreateUserRequest, UpdateUserRequest};
use crate::errors::AppError;
use crate::services::user_service::UserService;

#[utoipa::path(
    post,
    path = "/api/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = UserResponse),
        (status = 400, description = "Bad request")
    ),
    tag = "Users"
)]
#[post("/api/users")]
pub async fn create_user(
    db: web::Data<DatabaseConfig>,
    request: web::Json<CreateUserRequest>,
) -> Result<impl Responder, AppError> {
    // Validate the request
    request.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let user = UserService::create_user(db.get_pool(), request.into_inner()).await?;
    Ok(HttpResponse::Created().json(user))
}

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    params(
        ("id" = i64, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found")
    ),
    tag = "Users"
)]
#[get("/api/users/{id}")]
pub async fn get_user(
    db: web::Data<DatabaseConfig>,
    path: web::Path<i64>,
) -> Result<impl Responder, AppError> {
    let user = UserService::get_user(db.get_pool(), path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(user))
}

#[utoipa::path(
    get,
    path = "/api/users",
    responses(
        (status = 200, description = "List of all users", body = Vec<UserResponse>)
    ),
    tag = "Users"
)]
#[get("/api/users")]
pub async fn get_all_users(
    db: web::Data<DatabaseConfig>,
) -> Result<impl Responder, AppError> {
    let users = UserService::get_all_users(db.get_pool()).await?;
    Ok(HttpResponse::Ok().json(users))
}

#[utoipa::path(
    put,
    path = "/api/users/{id}",
    params(
        ("id" = i64, Path, description = "User ID")
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated successfully", body = UserResponse),
        (status = 404, description = "User not found"),
        (status = 400, description = "Bad request")
    ),
    tag = "Users"
)]
#[put("/api/users/{id}")]
pub async fn update_user(
    db: web::Data<DatabaseConfig>,
    path: web::Path<i64>,
    request: web::Json<UpdateUserRequest>,
) -> Result<impl Responder, AppError> {
    // Validate the request
    request.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let user = UserService::update_user(db.get_pool(), path.into_inner(), request.into_inner()).await?;
    Ok(HttpResponse::Ok().json(user))
}

#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    params(
        ("id" = i64, Path, description = "User ID")
    ),
    responses(
        (status = 204, description = "User deleted successfully"),
        (status = 404, description = "User not found")
    ),
    tag = "Users"
)]
#[delete("/api/users/{id}")]
pub async fn delete_user(
    db: web::Data<DatabaseConfig>,
    path: web::Path<i64>,
) -> Result<impl Responder, AppError> {
    UserService::delete_user(db.get_pool(), path.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
