use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum ApiError {
    InternalServerError,
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Something unexpected happened"),
            ),
        }
        .into_response()
    }
}
