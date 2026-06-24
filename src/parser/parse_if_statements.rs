use crate::error::error::CompilerError;
use crate::interfaces::lexer_interface::{consume_token, is_peek_match_token, next};
use crate::lexer::lexer::Lexer;
use crate::lexer::tokens::Token;
use crate::parser::parse_expressions::expression;
use crate::parser::parse_tree::{Conditional, IfStatement};
use crate::parser::parser::closure;

pub fn if_statement(lexer: &mut Lexer) -> Result<IfStatement, CompilerError> {
    let _ = consume_token(lexer, &Token::If)?;
    let condition = expression(lexer)?;
    let body = closure(lexer)?;
    let conditional = Conditional {
        condition,
        body
    };
    let mut conditionals = Vec::from([conditional]);
    let mut otherwise = None;

    while is_peek_match_token(lexer, &Token::Else)? {
        let _ = next(lexer)?;

        if is_peek_match_token(lexer, &Token::If)? {
            let _ = next(lexer)?;
            let condition = expression(lexer)?;
            let body = closure(lexer)?;
            let conditional = Conditional {
                condition,
                body
            };
            conditionals.push(conditional);
        } else {
            otherwise = Some(closure(lexer)?);
            break;
        }
    }

    Ok(IfStatement {
        conditionals,
        otherwise
    })
}