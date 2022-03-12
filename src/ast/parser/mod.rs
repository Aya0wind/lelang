

mod common;
mod function_parser;
mod variable_parser;
mod statement;
mod condition;
mod for_loop;
mod while_loop;

pub use common::*;
pub use function_parser::*;
pub use variable_parser::*;
pub use statement::*;
pub use condition::*;
pub use for_loop::*;
pub use while_loop::*;

use std::collections::HashMap;
use anyhow::Result;

use crate::ast::{BExpr, FunctionNode};
use crate::lexer::Operator;


pub fn get_operator_precedence(op:&Operator)->usize{
    match op {
        Operator::Plus => {20}
        Operator::Sub => {20}
        Operator::Mul => {40}
        Operator::Div => {40}
        Operator::Assign => {10}
        Operator::Equal => {10}
        Operator::GreaterThan => {10}
        Operator::LessThan => {10}
        Operator::GreaterOrEqualThan => {10}
        Operator::LessOrEqualThan => {10}
    }
}


pub type VParseResult = Result<BExpr>;
pub type FParseResult = Result<FunctionNode>;
