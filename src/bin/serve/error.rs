//! Typed API error with proper HTTP status code mapping.
//!
//! Implements `IntoResponse` so handlers can return `Result<T, ApiError>`
//! instead of always returning HTTP 200.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

/// API error response body returned to the client.
#[derive(Debug, Serialize, ToSchema)]
#[non_exhaustive]
pub struct ApiErrorBody {
    /// Machine-readable error type identifier (e.g. `"bad_request"`).
    pub error: String,
    /// Human-readable error message describing what went wrong.
    pub message: String,
}

/// Typed API error that maps to proper HTTP status codes.
///
/// Handlers return `Result<Json<T>, ApiError>` so that error conditions
/// produce semantically correct HTTP responses instead of HTTP 200.
#[derive(Debug)]
#[non_exhaustive]
pub enum ApiError {
    /// 400 — malformed request body, invalid JSON, or missing required field.
    BadRequest(String),
    /// 422 — SQL tokenization or parsing failed (syntax error in input).
    UnprocessableEntity(String),
    /// 404 — referenced resource not found (e.g. schema file path).
    NotFound(String),
    /// 500 — internal server error (serialization failure, unexpected panic).
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match self {
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "bad_request", msg),
            ApiError::UnprocessableEntity(msg) => (StatusCode::UNPROCESSABLE_ENTITY, "unprocessable_entity", msg),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, "not_found", msg),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", msg),
        };
        let body = ApiErrorBody { error: error_type.to_string(), message };
        (status, Json(body)).into_response()
    }
}
