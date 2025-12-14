use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
    pub name: String,

    #[validate(email(message = "Invalid email format"))]
    #[validate(length(max = 255, message = "Email must not exceed 255 characters"))]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
    pub name: Option<String>,

    #[validate(email(message = "Invalid email format"))]
    #[validate(length(max = 255, message = "Email must not exceed 255 characters"))]
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}
