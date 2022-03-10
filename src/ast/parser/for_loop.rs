use crate::ast::BExpr;
#[derive(Debug)]
pub struct ForLoop{
    init_statement:BExpr,
    condition:BExpr,
    iterate:BExpr,
}