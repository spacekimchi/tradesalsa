use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt::Debug;

// Custom error handler function
pub fn e500<T>(e: T) -> ErrorResponse
where
    T: Debug + std::fmt::Display + 'static,
{
    ErrorResponse::InternalServerError(e.to_string())
}

// Custom error response struct
pub struct ErrorResponse {
    status_code: StatusCode,
    message: String,
}

impl ErrorResponse {
    #[allow(non_snake_case)]
    pub fn InternalServerError(message: String) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message,
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (self.status_code, self.message).into_response()
    }
}
