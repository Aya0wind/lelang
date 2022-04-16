pub use nodes::Ast;

use crate::error::SyntaxError;

pub mod parser;
pub mod nodes;

pub type ParseResult<T> = std::result::Result<T, SyntaxError>;