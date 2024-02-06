use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFailed(String),
    TicketDeletionFailedNotFound { id: u64 },
    AuthFailedNotAuthTokenCookie,
    AuthFailedTokenWrongFormat,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");
        match self {
            Error::LoginFailed(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
            }
            Error::TicketDeletionFailedNotFound { id } => {
                (StatusCode::NOT_FOUND, format!("Ticket {id} not found")).into_response()
            }
            Error::AuthFailedNotAuthTokenCookie => {
                (StatusCode::UNAUTHORIZED, "Auth failed").into_response()
            }
            Error::AuthFailedTokenWrongFormat => {
                (StatusCode::UNAUTHORIZED, "Auth failed").into_response()
            }

        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}
