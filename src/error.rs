use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    InvalidToken,
    WrongCredential,
    MissingCredential,
    TokenCreation,
    InternalServerError,
    UserDoesNotExist,
    UserAlreadyExits,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal server error occured",
            ),
            Self::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            Self::MissingCredential => (StatusCode::BAD_REQUEST, "Missing credential"),
            Self::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create token"),
            Self::WrongCredential => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            Self::UserDoesNotExist => (StatusCode::UNAUTHORIZED, "User does not exist"),
            Self::UserAlreadyExits => (StatusCode::BAD_REQUEST, "User already exists"),
        };
        (status, Json(json!({ "error": err_msg }))).into_response()
    }
}