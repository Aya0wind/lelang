pub use le_error::*;

mod le_error;
pub mod error_list;

pub type Result<T> = std::result::Result<T, LEError>;