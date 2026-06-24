use crate::error::error::CompilerError;
use crate::interfaces::lexer_interface::{consume_identifier, consume_token, consume_type, is_peek_match_token};
use crate::lexer::lexer::Lexer;
use crate::lexer::tokens::Token;
use crate::parser::parse_tree::{Function, Type, TypedArgument};
use crate::parser::parser::closure;

pub fn function(lexer: &mut Lexer) -> Result<Function, CompilerError> {
    let _  = consume_token(lexer, &Token::Fn)?;

    let identifier = consume_identifier(lexer)?;

    let _  = consume_token(lexer, &Token::LParen)?;
    let arguments = function_declaration_arguments(lexer)?;
    let _  = consume_token(lexer, &Token::RParen)?;

    let return_type = if is_peek_match_token(lexer, &Token::Arrow)? {
        let _  = consume_token(lexer, &Token::Arrow)?;
        consume_type(lexer)?
    } else {
        Type::Void
    };

    let body = closure(lexer)?;

    Ok(Function {
        identifier,
        return_type,
        arguments,
        body
    })
}

fn function_declaration_arguments(lexer: &mut Lexer) -> Result<Vec<TypedArgument>, CompilerError> {
    let mut arguments = Vec::new();
    if !is_peek_match_token(lexer, &Token::RParen)? {
        let identifier = consume_identifier(lexer)?;
        let _ = consume_token(lexer, &Token::Colon)?;
        let arg_type = consume_type(lexer)?;
        arguments.push(TypedArgument {
            identifier,
            arg_type
        })
    }

    while !is_peek_match_token(lexer, &Token::RParen)? {
        let _ = consume_token(lexer, &Token::Comma)?;

        let identifier = consume_identifier(lexer)?;
        let _ = consume_token(lexer, &Token::Colon)?;
        let arg_type = consume_type(lexer)?;
        arguments.push(TypedArgument {
            identifier,
            arg_type
        })
    }
    Ok(arguments)
}