use crate::db::user_db::UserDb;
use crate::dto::user_dto::{CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::errors::AppError;
use crate::models::user::User;
use sqlx::MySqlPool;

pub struct UserService;

impl UserService {
    fn user_to_response(user: User) -> UserResponse {
        UserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
            created_at: user.created_at.to_rfc3339(),
            updated_at: user.updated_at.to_rfc3339(),
        }
    }

    pub async fn create_user(
        pool: &MySqlPool,
        request: CreateUserRequest,
    ) -> Result<UserResponse, AppError> {
        if request.name.trim().is_empty() {
            return Err(AppError::BadRequest("Name cannot be empty".to_string()));
        }

        if request.email.trim().is_empty() {
            return Err(AppError::BadRequest("Email cannot be empty".to_string()));
        }

        let user = UserDb::create_user(pool, &request.name, &request.email).await?;
        Ok(Self::user_to_response(user))
    }

    pub async fn get_user(pool: &MySqlPool, id: i64) -> Result<UserResponse, AppError> {
        let user = UserDb::get_user_by_id(pool, id).await?;
        Ok(Self::user_to_response(user))
    }

    pub async fn get_all_users(pool: &MySqlPool) -> Result<Vec<UserResponse>, AppError> {
        let users = UserDb::get_all_users(pool).await?;
        Ok(users.into_iter().map(Self::user_to_response).collect())
    }

    pub async fn update_user(
        pool: &MySqlPool,
        id: i64,
        request: UpdateUserRequest,
    ) -> Result<UserResponse, AppError> {
        if let Some(ref name) = request.name {
            if name.trim().is_empty() {
                return Err(AppError::BadRequest("Name cannot be empty".to_string()));
            }
        }

        if let Some(ref email) = request.email {
            if email.trim().is_empty() {
                return Err(AppError::BadRequest("Email cannot be empty".to_string()));
            }
        }

        let user = UserDb::update_user(
            pool,
            id,
            request.name.as_deref(),
            request.email.as_deref(),
        )
        .await?;
        Ok(Self::user_to_response(user))
    }

    pub async fn delete_user(pool: &MySqlPool, id: i64) -> Result<(), AppError> {
        let rows_affected = UserDb::delete_user(pool, id).await?;
        if rows_affected == 0 {
            return Err(AppError::NotFound(format!("User with id {} not found", id)));
        }
        Ok(())
    }
}
