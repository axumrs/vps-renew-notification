pub mod config;
pub mod db;
mod err;
pub mod filter;
pub mod handler;
pub mod id;
pub mod jwt;
pub mod middleware;
pub mod model;
pub mod password;
pub mod payload;
mod resp;
pub mod route;
mod state;

pub use err::{Error, Kind as ErrorKind};
pub use resp::*;
pub use state::*;

pub type Result<T> = std::result::Result<T, crate::Error>;
