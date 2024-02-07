use axum::{
    http::StatusCode,
    response::{self, IntoResponse, Response},
};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFailed,
    TicketDeletionFailedNotFound { id: u64 },
    AuthFailedNotAuthTokenCookie,
    AuthFailedTokenWrongFormat,
    AuthFailedCtxNotInRequest,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        // create a placeholder Axum response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // insert the Error into the response extensions
        response.extensions_mut().insert(self);
        response
    }
}

impl Error {
    pub fn client_status(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            // Login Error
            Self::LoginFailed => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            // Auth Error
            Self::AuthFailedCtxNotInRequest
            | Self::AuthFailedNotAuthTokenCookie
            | Self::AuthFailedTokenWrongFormat => (StatusCode::UNAUTHORIZED, ClientError::NO_AOUTH),

            // Model Error
            Self::TicketDeletionFailedNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::IVALID_PARAMS)
            }
            // Fallback Error
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AOUTH,
    IVALID_PARAMS,
    SERVICE_ERROR,
}
