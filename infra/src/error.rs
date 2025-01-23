use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
  Json,
};
use sea_orm::{DbErr, TransactionError};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
  #[error("not_found_error")]
  NotFoundError,

  #[error("validation_error")]
  ValidationError,

  #[error("custom_error")]
  CustomError(String, String),

  #[error("internal_server_error")]
  DatabaseError(#[from] DbErr),

  #[error("transaction_error")]
  TransactionError(#[from] TransactionError<DbErr>),
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
  pub ok: bool,
  pub code: String,
  pub message: String,
}

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    let (status_code, code, message) = match self {
      AppError::NotFoundError => (
        StatusCode::NOT_FOUND,
        self.to_string(),
        "Not found error".to_string(),
      ),
      AppError::ValidationError => (
        StatusCode::BAD_REQUEST,
        self.to_string(),
        "Failed to validate request".to_string(),
      ),
      AppError::DatabaseError(e) => (
        StatusCode::INTERNAL_SERVER_ERROR,
        "internal_server_error".to_string(),
        e.to_string(),
      ),
      AppError::TransactionError(e) => (
        StatusCode::INTERNAL_SERVER_ERROR,
        "internal_server_error".to_string(),
        e.to_string(),
      ),
      AppError::CustomError(code, message) => {
        (StatusCode::INTERNAL_SERVER_ERROR, code, message.to_string())
      }
    };

    let response = ErrorResponse {
      ok: false,
      code,
      message,
    };

    (status_code, Json(response)).into_response()
  }
}
