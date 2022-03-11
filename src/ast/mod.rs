
mod nodes;
mod parser;



use std::collections::HashMap;
use anyhow::Result;

pub use nodes::*;
pub use parser::*;
use crate::error::{SyntaxError, TokenType};
use crate::lexer::{KeyWord, LELexer, LEToken};

#[derive(Debug)]
pub struct Ast {
    pub globals: HashMap<String, VariableNode>,
    pub functions: HashMap<String, FunctionNode>,
}


impl Ast {
    pub fn from_tokens(mut tokens: LELexer) -> Result<Self> {
        let mut ast = Self { globals: HashMap::new(), functions: HashMap::new() };
        ast.parse(tokens)?;
        Ok(ast)
    }

    fn parse(&mut self, mut tokens: LELexer) -> Result<()> {
        loop {
            let next_token = tokens.current();
            match next_token {
                None => { break; }
                Some(token) => {
                    if let LEToken::KeyWord(keyword) = token {
                        if &KeyWord::FunctionDeclare == keyword {
                            let function = parse_function(&mut tokens)?;
                            self.functions.insert(function.name.clone(), function);
                        } else if &KeyWord::VariableDeclare == keyword {
                            let variable = parse_statement(&mut tokens)?;
                            if let Statement::VariableDeclare(variable) = variable {
                                self.globals.insert(variable.name.clone(), variable);
                            }
                        } else {
                            return Err(SyntaxError::unexpect_token(TokenType::FunctionDeclare, tokens.current_result()?.clone(), tokens.line()).into());
                        }
                    }
                }
            }
        }
        Ok(())
    }
}