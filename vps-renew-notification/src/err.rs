use axum::{response::IntoResponse, Json};

use crate::JsonResp;

#[derive(Debug)]
pub enum Kind {
    Db,
    Jwt,
    Unauthorized,
    Validate,
    IncorrectAuth,
    Bcrypt,
    NotExists,
    Chrono,
    InvalidParameter,
    Reqwest,
}

#[derive(Debug)]
pub struct Error {
    pub kind: Kind,
    pub message: String,
    pub cause: Option<Box<dyn std::error::Error>>,
}

impl Error {
    fn new(kind: Kind, message: String, cause: Option<Box<dyn std::error::Error>>) -> Self {
        Self {
            kind,
            message,
            cause,
        }
    }
    pub fn with_cause(kind: Kind, cause: Box<dyn std::error::Error>) -> Self {
        Self::new(kind, cause.to_string(), Some(cause))
    }
    pub fn from_string(kind: Kind, message: String) -> Self {
        Self::new(kind, message, None)
    }
    pub fn from_str(kind: Kind, msg: &str) -> Self {
        Self::from_string(kind, msg.to_string())
    }
    pub fn unauthorized() -> Self {
        Self::from_str(Kind::Unauthorized, "未授权")
    }
    pub fn incorrect_auth() -> Self {
        Self::from_str(Kind::IncorrectAuth, "用户名/密码错误")
    }
    pub fn not_exists(msg: &str) -> Self {
        Self::from_str(Kind::NotExists, msg)
    }
    pub fn invalid_parameter(msg: &str) -> Self {
        Self::from_str(Kind::InvalidParameter, msg)
    }
    pub fn code(&self) -> i32 {
        -1
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Self::with_cause(Kind::Db, Box::new(e))
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        Self::with_cause(Kind::Jwt, Box::new(e))
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(e: validator::ValidationErrors) -> Self {
        Self::with_cause(Kind::Validate, Box::new(e))
    }
}

impl From<bcrypt::BcryptError> for Error {
    fn from(e: bcrypt::BcryptError) -> Self {
        Self::with_cause(Kind::Bcrypt, Box::new(e))
    }
}

impl From<chrono::ParseError> for Error {
    fn from(e: chrono::ParseError) -> Self {
        Self::with_cause(Kind::Chrono, Box::new(e))
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::with_cause(Kind::Reqwest, Box::new(e))
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let resp = JsonResp::err(self);
        Json(resp).into_response()
    }
}
