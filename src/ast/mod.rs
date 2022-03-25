use anyhow::Result;

pub use nodes::*;
pub use parser::*;

use crate::error::{SyntaxError, TokenType};
use crate::lexer::{KeyWord, LELexer, LEToken};

mod nodes;
mod parser;

#[derive(Debug)]
pub struct Ast {
    pub globals: Vec<VariableNode>,
    pub function_definitions: Vec<FunctionDefinition>,
    pub extern_functions: Vec<ExternFunction>,
}


impl Ast {
    pub fn from_tokens(tokens: LELexer) -> Result<Self> {
        let mut ast = Self { globals: vec![], function_definitions: vec![], extern_functions: vec![] };
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
                        match keyword {
                            KeyWord::Declare => {
                                tokens.consume_keyword()?;
                                let function_prototype = parse_function_prototype(&mut tokens)?;
                                tokens.consume_semicolon()?;
                                self.extern_functions.push(function_prototype);
                            }
                            KeyWord::FunctionDefine => {
                                let function = parse_function(&mut tokens)?;
                                self.function_definitions.push(function);
                            }
                            KeyWord::VariableDeclare => {
                                let variable = parse_variable_declaration(&mut tokens)?;
                                self.globals.push(variable);
                            }
                            _ => {
                                return Err(SyntaxError::unexpect_token(TokenType::FunctionDeclare, tokens.current_result()?.clone(), tokens.line()).into());
                            }
                        }
                    } else {
                        return Err(SyntaxError::unexpect_token(TokenType::FunctionDeclare, tokens.current_result()?.clone(), tokens.line()).into());
                    }
                }
            }
        }
        Ok(())
    }
}
