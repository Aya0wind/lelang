mod common;
mod function_parser;
mod variable_parser;
mod statement;
pub use statement::*;
pub use function_parser::*;
pub use variable_parser::*;
pub use common::*;

use lazy_static::lazy_static;
use std::collections::HashMap;
use anyhow::Result;
use crate::ast::{FunctionNode, BExpr, Expr};

lazy_static! {
    pub static ref BinopPrecedence:HashMap<&'static str,usize> = HashMap::from([
        ("+",20),
        ("-",20),
        ("*",40),
        ("/",40),
        (">",10),
        ("<",10),
    ]);
}

pub type VParseResult = Result<BExpr>;
pub type FParseResult= Result<FunctionNode>;
