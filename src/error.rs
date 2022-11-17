use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde_json::json;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Forbidden")]
    Forbidden,

    #[display(fmt = "Unauthorized")]
    Unauthorized,

    #[display(fmt = "NotFound: {}", _0)]
    NotFound(String),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::InternalServerError => HttpResponse::InternalServerError()
                .json("Internal server error: please again try later"),
            Error::BadRequest(ref message) => HttpResponse::BadRequest().json(json!({
                "@type": "http://www.w3.org/ns/hydra/core#Error",
                "title": "Bad Request",
                "detail": message
            })),
            Error::Forbidden => HttpResponse::Forbidden().json(json!({
                "@type": "http://www.w3.org/ns/hydra/core#Error",
                "title": "Forbidden",
            })),
            Error::Unauthorized => HttpResponse::Unauthorized().json(json!({
                "@type": "http://www.w3.org/ns/hydra/core#Error",
                "title": "Unauthorized",
            })),
            Error::NotFound(ref message) => HttpResponse::NotFound().json(json!({
                "@type": "http://www.w3.org/ns/hydra/core#Error",
                "title": "Not Found",
                "detail": message
            })),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Error::NotFound(err.to_string()),
            _ => Error::InternalServerError,
        }
    }
}
