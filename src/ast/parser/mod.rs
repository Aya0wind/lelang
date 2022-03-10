use std::collections::HashMap;

use anyhow::Result;
use lazy_static::lazy_static;

pub use common::*;
pub use function_parser::*;
pub use statement::*;
pub use variable_parser::*;

use crate::ast::{BExpr, FunctionNode};

mod common;
mod function_parser;
mod variable_parser;
mod statement;
mod condition;
mod for_loop;
mod while_loop;

lazy_static! {
    pub static ref BINOP_PRECEDENCE:HashMap<&'static str,usize> = HashMap::from([
        ("+",20),
        ("-",20),
        ("*",40),
        ("/",40),
        (">",10),
        ("<",10),
    ]);
}

pub type VParseResult = Result<BExpr>;
pub type FParseResult = Result<FunctionNode>;
