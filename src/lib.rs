mod config;
mod err;
pub mod types;

pub use crate::config::*;
pub use err::{Error, Kind as ErrorKind};
pub use types::*;

pub type Result<T> = std::result::Result<T, crate::Error>;
