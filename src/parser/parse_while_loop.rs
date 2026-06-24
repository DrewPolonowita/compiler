use crate::error::error::CompilerError;
use crate::interfaces::lexer_interface::consume_token;
use crate::lexer::lexer::Lexer;
use crate::lexer::tokens::Token;
use crate::parser::parse_expressions::expression;
use crate::parser::parse_tree::WhileLoop;
use crate::parser::parser::closure;

pub fn while_loop(lexer: &mut Lexer) -> Result<WhileLoop, CompilerError> {
    let _ = consume_token(lexer, &Token::While)?;
    let condition = expression(lexer)?;
    let body = closure(lexer)?;

    Ok(WhileLoop {
        condition,
        body,
    })
}