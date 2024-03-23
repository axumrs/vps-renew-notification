pub mod config;
pub mod db;
mod err;
pub mod filter;
pub mod id;
pub mod jwt;
pub mod model;
mod state;

pub use err::{Error, Kind as ErrorKind};
pub use state::*;

pub type Result<T> = std::result::Result<T, crate::Error>;
