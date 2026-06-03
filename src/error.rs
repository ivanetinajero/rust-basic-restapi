use axum::{http::StatusCode, response::IntoResponse};
use sea_orm::DbErr;

pub enum ApiError {
    // This error is returned when the API returns a non-success status code.
    Internal(Box<dyn std::error::Error + Send + Sync>),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::Internal(err) => {
                println!("Internal server error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR).into_response()
            }
        }
    }
}

impl From<DbErr> for ApiError {
    fn from(vaue: DbErr) -> Self {
        Self::Internal(Box::new(vaue))
    }
}
