use axum::response::{IntoResponse, Response};
use axum::{http, Json};
use maglev::auth::AuthError;
use maglev::error::google::ErrorResponse;
use std::borrow::Cow;

/// A common error type that can be used throughout the API.
///
/// Can be returned in a `Result` from an API handler function.
///
/// This represents both API errors as well as internal recoverable errors,
/// and maps them to appropriate status codes along with error messages.
#[derive(Debug, thiserror::Error, maglev::HttpError)]
#[allow(unused)]
pub enum Error {
    /// Return `401 Unauthorized`
    #[error("authentication required")]
    #[http_error(UNAUTHORIZED)]
    Unauthorized,

    /// Return `403 Forbidden`
    #[error("user may not perform that action")]
    #[http_error(FORBIDDEN)]
    Forbidden,

    /// Return `404 Not Found`
    #[error("request path not found")]
    #[http_error(NOT_FOUND)]
    NotFound,

    /// Return `400 Bad Request`
    ///
    /// This also serializes the `errors` map to JSON
    #[error("bad request: {message}")]
    #[http_error(BAD_REQUEST, "{message}")]
    BadRequest { message: Cow<'static, str> },

    /// Return `500 Internal Server Error` on a `anyhow::Error`.
    ///
    /// `anyhow::Error` is used in a few places to capture context and backtraces
    /// on unrecoverable (but technically non-fatal) errors which could be highly useful for
    /// debugging.
    ///
    /// The actual error message is not returned to the client for security reasons.
    #[error("server error")]
    #[http_error(INTERNAL_SERVER_ERROR, "an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // Trace server errors since we don't return the detailed error in the response body
        if self.http_code().is_server_error() {
            tracing::error!("{} {:?}", self.http_code(), self);
        }

        // Construct a response
        let body = Json(ErrorResponse::new(
            self.http_code().as_u16(),
            self.http_message(),
        ));
        (self.http_code(), body).into_response()
    }
}

// error conversions that simply wrap with anyhow::Error
// maglev::anyhow_from!(clients::ClientError);
// maglev::anyhow_from!(maglev::crypto::CryptoError);

impl From<AuthError> for Error {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::Unauthorized => Error::Unauthorized,
            err => anyhow::Error::new(err)
                .context("Unexpected auth error")
                .into(),
        }
    }
}

impl Error {
    /// Convenient constructor for `Error::UnprocessableEntity`.
    ///
    /// Multiple for the same key are collected into a list for that key
    #[allow(unused)]
    pub fn bad_req(message: impl Into<Cow<'static, str>>) -> Self {
        Self::BadRequest {
            message: message.into(),
        }
    }
}
