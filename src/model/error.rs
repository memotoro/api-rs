use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(thiserror::Error, Debug, Clone)]
pub enum AppError {
    #[error("error with application {0}")]
    ApplicationError(String),
}

#[derive(Serialize, Clone, Debug)]
pub struct ErrorResponse {
    code: ErrorCode,
    message: String,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    DataError,
    ApplicationError,
}

impl From<AppError> for ErrorCode {
    fn from(error: AppError) -> ErrorCode {
        match error {
            AppError::ApplicationError(_) => ErrorCode::ApplicationError,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::ApplicationError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    code: self.clone().into(),
                    message: self.to_string(),
                }),
            )
                .into_response(),
        }
    }
}
