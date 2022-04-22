pub use le_error::*;

pub mod error_list;
mod le_error;

pub type Result<T> = std::result::Result<T, LEError>;