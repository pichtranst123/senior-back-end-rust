use axum::{extract::rejection::JsonRejection, response::IntoResponse};
use chrono::ParseError;
use config::ConfigError;
use thiserror::Error;
use validator::{ValidationError, ValidationErrors};

use crate::{resps::Web, utils::validation::extract_validation_error};

#[derive(Debug, Error)]
pub enum AppError {
  #[error("Json parsing error")]
  JsonParsing(#[from] JsonRejection),

  #[error("Invalid single field")]
  InvalidSingleField(#[from] ValidationError),

  #[error("Invalid multiple fields")]
  InvalidMultipleFields(#[from] ValidationErrors),

  #[error("Invalid Configuration")]
  ConfigError(#[from] ConfigError),

  #[error("Query Error fields")]
  SqlxError(#[from] sqlx::Error),

  #[error("Invalid Askama fields")]
  AskamaError(#[from] askama::Error),

  #[error("Invalid Askama fields")]
  SerdeJsonError(#[from] serde_json::Error),

  #[error("Invalid Askama fields")]
  ChronoError(#[from] ParseError),
}

impl IntoResponse for AppError {
  fn into_response(self) -> axum::response::Response {
    match self {
      AppError::JsonParsing(e) => Web::bad_request("Invalid request body", e),
      AppError::InvalidSingleField(e) => Web::bad_request("One of the request fields might be incorrect", e),
      AppError::InvalidMultipleFields(e) => {
        Web::bad_request("Multiple fields in the request body are invalid", extract_validation_error(&e))
      },
      AppError::ConfigError(e) => Web::internal_error("Configuration files error", e),
      AppError::SqlxError(e) => Web::internal_error("Sqlx query errors", e),
      AppError::AskamaError(e) => Web::bad_request("Askama error", e),
      AppError::SerdeJsonError(e) => Web::internal_error("can deserialize error", e),
      AppError::ChronoError(e) => Web::internal_error("chrono data error", e),
    }
  }
}
