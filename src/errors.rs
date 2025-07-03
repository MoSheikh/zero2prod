use std::fmt::Display;

use actix_web::ResponseError;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ValidationErrorResponse<'a> {
    pub message: &'a str,
    pub errors: Vec<String>,
}

impl<'a> Display for ValidationErrorResponse<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{message}:\n{errors}",
            message = self.message,
            errors = self.errors.join("\n")
        )
    }
}

impl<'a> ResponseError for ValidationErrorResponse<'a> {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        actix_web::HttpResponse::build(self.status_code()).json(self)
    }
}
