use crate::ast::nodes::TypeDeclarator;
use crate::ast::parser::array::parse_array_declarator;
use crate::ast::ParseResult;
use crate::lexer::{KeyWord, LELexer, LEToken};

pub fn parse_type_declarator(lexer: &mut LELexer) -> ParseResult<TypeDeclarator> {
    match lexer.current_result()? {
        LEToken::Identifier(identifier) => {
            let identifier = lexer.consume_identifier()?;
            Ok(TypeDeclarator::TypeIdentifier(identifier))
        }
        LEToken::KeyWord(KeyWord::Ref) => {
            lexer.consume_keyword()?;
            let ref_type = parse_type_declarator(lexer)?;
            Ok(TypeDeclarator::Reference(Box::new(ref_type)))
        }
        LEToken::LeftBracket => {
            Ok(TypeDeclarator::Array(Box::new(parse_array_declarator(lexer)?)))
        }
        _ => { unreachable!() }
    }
}