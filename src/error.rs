use std::fmt::Display;

use actix_web::{http::StatusCode, HttpResponse, HttpResponseBuilder, ResponseError};

#[derive(Debug)]
pub enum CustomError {
    InvalidToken,
    UserExists, //possibly or other error
    InvalidJSON,
    NotFoundJSON,
    JSONAddFail,
    UserNotMatched, //possibly or other error
}

impl Into<HttpResponse> for CustomError {
    fn into(self) -> HttpResponse {
        self.error_response()
    }
}

impl actix_web::error::ResponseError for CustomError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponseBuilder::new(self.status_code())
            .body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::InvalidJSON => StatusCode::BAD_REQUEST,
            Self::InvalidToken => StatusCode::UNAUTHORIZED,
            Self::UserExists => StatusCode::BAD_REQUEST,
            Self::UserNotMatched => StatusCode::NOT_FOUND,
            Self::NotFoundJSON => StatusCode::NOT_FOUND,
            Self::JSONAddFail => StatusCode::CONFLICT
        }
    }
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::InvalidJSON => "Input JSON has invalid or unsupported structure",
            Self::InvalidToken => "Api-Key is invalid",
            Self::UserExists => "Failed to add new user. User may already exists",
            Self::UserNotMatched => "Failed to match user. User may not exist or credentials are invalid",
            Self::NotFoundJSON => "Failed to find JSON",
            Self::JSONAddFail => "Failed to Add JSON. JSON structure may not be supported"
        };

        f.write_str(s)?;
        Ok(())
    }
}

