use actix_web::{error, HttpResponse};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum HttpError {
    #[display(fmt = "Internal server error")]
    Unauthenticated,
    #[display(fmt = "Timed out")]
    Timeout,
    #[display(fmt = "Invalid params")]
    InvalidParams,
}

impl error::ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            HttpError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            HttpError::Timeout => StatusCode::REQUEST_TIMEOUT,
            HttpError::InvalidParams => StatusCode::BAD_REQUEST,
        }
    }
}