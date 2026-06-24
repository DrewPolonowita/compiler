use crate::error::error::CompilerError;
use crate::interfaces::lexer_interface::{consume_factor, consume_operator, consume_token, is_peek_in};
use crate::lexer::lexer::Lexer;
use crate::lexer::tokens::Token;
use crate::parser::parse_tree::{Expression, ExpressionEnum, UnaryExpression};


pub fn expression(lexer: &mut Lexer) -> Result<ExpressionEnum, CompilerError> {
    or_expression(lexer)
}

/* ---------- BOOLEAN EXPRESSIONS ---------- */

fn or_expression(lexer: &mut Lexer) -> Result<ExpressionEnum, CompilerError> {
    let mut value = and_expression(lexer)?;

    while is_peek_in(lexer, &[
        Token::Or
    ])? {
        let operator = consume_operator(lexer)?;

        value = ExpressionEnum::Expression(Expression {
            left: Box::new(value),
            operator,
            right: Box::new(and_expression(lexer)?),
        })
    }

    Ok(value)
}

fn and_expression(lexer: &mut Lexer) -> Result<ExpressionEnum, CompilerError> {
    let mut value = equality_expression(lexer)?;

    while is_peek_in(lexer, &[
        Token::And
    ])? {
        let operator = consume_operator(lexer)?;

        value = ExpressionEnum::Expression(Expression {
            left: Box::new(value),
            operator,
            right: Box::new(equality_expression(lexer)?),
        })
    }

    Ok(value)
}

/* ---------- EQUALITY / RELATIONAL EXPRESSIONS ---------- */

fn equality_expression(lexer: &mut Lexer) -> Result<ExpressionEnum, CompilerError> {
    let mut value = relational_expression(lexer)?;

    while is_peek_in(lexer, &[
        Token::Eq, Token::Neq
    ])? {
        let operator = consume_operator(lexer)?;

        value = ExpressionEnum::Expression(Expression {
            left: Box::new(value),
            operator,
            right: Box::new(relational_expression(lexer)?),
        })
    }

    Ok(value)
}

fn relational_expression(lexer: &mut Lexer) -> Result<ExpressionEnum, CompilerError> {
    let mut value = arithmetic_expression(lexer)?;

    while is_peek_in(lexer, &[
        Token::Lt, Token::Gt, Token::Leq, Token::Geq
    ])? {
        let operator = consume_operator(lexer)?;

        value = ExpressionEnum::Expression(Expression {
            left: Box::new(value),
            operator,
            right: Box::new(arithmetic_expression(lexer)?),
        })
    }

    Ok(value)
}

/* ---------- ARITHMETIC EXPRESSIONS ---------- */

fn arithmetic_expression(lexer: &mut Lexer) -> Result<ExpressionEnum, CompilerError> {
    let mut value = term(lexer)?;

    while is_peek_in(lexer, &[
        Token::Plus, Token::Subtract
    ])? {
        let operator = consume_operator(lexer)?;

        value = ExpressionEnum::Expression(Expression {
            left: Box::new(value),
            operator,
            right: Box::new(term(lexer)?),
        })
    }

    Ok(value)
}

fn term(lexer: &mut Lexer) -> Result<ExpressionEnum, CompilerError> {
    let mut value = factor(lexer)?;

    while is_peek_in(lexer, &[
        Token::Times, Token::Divide
    ])? {
        let operator = consume_operator(lexer)?;

        value = ExpressionEnum::Expression(Expression {
            left: Box::new(value),
            operator,
            right: Box::new(factor(lexer)?),
        })
    }

    Ok(value)
}

fn factor(lexer: &mut Lexer) -> Result<ExpressionEnum, CompilerError> {

    use crate::interfaces::token_type::Factor::*;
    match consume_factor(lexer)? {
        Integer(num) => {
            Ok(ExpressionEnum::Integer(num))
        },
        String(string) => {
            Ok(ExpressionEnum::String(string))
        },
        Identifier(id) => {
            Ok(ExpressionEnum::Identifier(id))
        },
        Not => {
            Ok(ExpressionEnum::UnaryExpression(
                UnaryExpression::Not(Box::new(factor(lexer)?))
            ))
        }
        LParen => {
            let expr = expression(lexer);
            let _ = consume_token(lexer, &Token::RParen)?;

            expr
        },
        True => {
            Ok(ExpressionEnum::Boolean(true))
        },
        False => {
            Ok(ExpressionEnum::Boolean(false))
        }
    }
}