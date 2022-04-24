pub mod generator;
pub mod builder;
pub mod context;

pub type Result<T> = std::result::Result<T, crate::error::CompileError>;
