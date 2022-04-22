use crate::ast::nodes::{Identifier, TypeDeclarator};
use crate::ast::parser::array::parse_array_declarator;
use crate::error::{LEError, Result};
use crate::error::{SyntaxError, TokenType};
use crate::lexer::{KeyWord, LELexer, LEToken, Position};

pub fn parse_type_declarator(lexer: &mut LELexer) -> Result<TypeDeclarator> {
    let current_token = lexer.current().ok_or_else(|| LEError::new_syntax_error(
        SyntaxError::missing_token(vec![TokenType::Identifier, TokenType::LeftBracket]),
        lexer.pos(),
    ))?;
    match current_token {
        LEToken::Identifier(identifier) => {
            let pos = lexer.pos();
            let identifier = lexer.consume_identifier()?;
            Ok(TypeDeclarator::TypeIdentifier(Identifier { name: identifier, pos }))
        }
        LEToken::KeyWord(KeyWord::Ref) => {
            lexer.consume_keyword()?;
            let ref_type = parse_type_declarator(lexer)?;
            Ok(TypeDeclarator::Reference(Box::new(ref_type)))
        }
        LEToken::LeftBracket => {
            Ok(TypeDeclarator::Array(Box::new(parse_array_declarator(lexer)?)))
        }
        _ => {
            Err(LEError::new_syntax_error(
                SyntaxError::unexpect_token(vec![TokenType::Identifier, TokenType::LeftBracket], current_token),
                lexer.pos(),
            ))
        }
    }
}