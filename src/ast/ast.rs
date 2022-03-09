use std::collections::HashMap;
use anyhow::Result;
use crate::ast::{FunctionNode, VariableNode};
use crate::ast::parser::{parse_function, parse_variable_declaration};
use crate::error::{SyntaxError, TokenType};
use crate::lexer::{KeyWord, LEToken, LELexer};


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
        loop{
            let next_token = tokens.next();
            match next_token {
                None => {break;}
                Some(token) => {
                    if let LEToken::KeyWord(keyword) = token {
                        if KeyWord::FunctionDeclare == keyword {
                            let function = parse_function(&mut tokens)?;
                            self.functions.insert(function.name.clone(),function );
                        } else if KeyWord::VariableDeclare == keyword {
                            let variable = parse_variable_declaration(&mut tokens)?;
                            self.globals.insert(variable.name.clone(), variable);
                        } else {
                            return Err(SyntaxError::unexpect_token(TokenType::FunctionDeclare, tokens.current().clone(), tokens.line()).into());
                        }
                    }
                }
            }
        }
        Ok(())
    }
}