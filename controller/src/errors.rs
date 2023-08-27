use axum::{http::StatusCode, response::IntoResponse, Json};
use log::error;
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Validation error")]
    InvalidRequest(#[from] ValidationErrors),

    #[error("GRPC client connection error")]
    ClientConnectError(#[from] tonic::transport::Error),

    #[error("Serialization error")]
    SerializationError(#[from] serde_json::Error),

    #[error("Database error")]
    DatabaseError(#[from] kv::Error),

    #[error("Scheduling error")]
    SchedulingError(#[from] tonic::Status),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApiError::InvalidRequest(json_rejection) => {
                (StatusCode::BAD_REQUEST, json_rejection.to_string())
            }
            ApiError::ClientConnectError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            ApiError::SerializationError(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            ApiError::DatabaseError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            ApiError::SchedulingError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        let payload = json!({
            "message": message,
        });

        error!("{:?}", Json(&payload));

        (status, Json(payload)).into_response()
    }
}
