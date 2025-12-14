use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;
use validator::ValidationErrors;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ValidationErrorResponse {
    pub error: String,
    pub errors: Vec<FieldError>,
}

#[derive(Debug, Serialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum AppError {
    DatabaseError(String),
    NotFound(String),
    BadRequest(String),
    ValidationError(ValidationErrors),
    InternalServerError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::ValidationError(errors) => write!(f, "Validation error: {}", errors),
            AppError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::ValidationError(errors) => {
                let field_errors: Vec<FieldError> = errors
                    .field_errors()
                    .iter()
                    .flat_map(|(field, errors)| {
                        errors.iter().map(move |error| FieldError {
                            field: field.to_string(),
                            message: error.message.as_ref()
                                .map(|m| m.to_string())
                                .unwrap_or_else(|| format!("Validation failed for {}", field)),
                        })
                    })
                    .collect();

                HttpResponse::build(StatusCode::BAD_REQUEST).json(ValidationErrorResponse {
                    error: "VALIDATION_ERROR".to_string(),
                    errors: field_errors,
                })
            }
            _ => {
                let (status, error_type, message) = match self {
                    AppError::DatabaseError(msg) => {
                        (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR", msg)
                    }
                    AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg),
                    AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg),
                    AppError::InternalServerError(msg) => {
                        (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR", msg)
                    }
                    _ => unreachable!(),
                };

                HttpResponse::build(status).json(ErrorResponse {
                    error: error_type.to_string(),
                    message: message.to_string(),
                })
            }
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Resource not found".to_string()),
            _ => AppError::DatabaseError(err.to_string()),
        }
    }
}
