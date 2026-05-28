use crate::error::error::CompilerError;
use crate::interfaces::lexer_interface::{consume_token, consume_type, is_empty, next, peek, peek_check};
use crate::parser::error::ParserError;
use crate::lexer::tokens::Token;
use crate::lexer::lexer::Lexer;
use crate::lexer::token_type::TokenType;
use crate::parser::error::ErrorType::ArithmeticExpectedError;

#[derive(Debug)]
#[allow(dead_code)]
pub enum ParseTree {
    Program(Box<ParseTree>),
    Statements(Vec<ParseTree>),
    Statement(Box<ParseTree>),
    Assignment(Box<ParseTree>, Box<ParseTree>, Box<ParseTree>),
    Expression(Box<ParseTree>, Token, Box<ParseTree>),
    Term(Box<ParseTree>, Token, Box<ParseTree>),
    Factor(Box<ParseTree>),
    Println(Box<ParseTree>),

    Type(Token),

    Arithmetic(Token),
    Number(Token),
    String(Token),
    Identifier(Token),
}

impl ParseTree {
    pub fn parse(mut lexer: Lexer) -> Result<ParseTree, CompilerError> {
        program(&mut lexer)
    }
}

fn program(lexer: &mut Lexer) -> Result<ParseTree, CompilerError> {
    Ok(ParseTree::Program(Box::new(statements(lexer)?)))
}

fn statements(lexer: &mut Lexer) -> Result<ParseTree, CompilerError> {
    let mut all_statements = Vec::from([statement(lexer)?]);

    while let Some(next_token) = peek(lexer)? {
        if matches!(next_token, Token::LineEnd) {
            next(lexer)?;

            if !is_empty(lexer) {
                all_statements.push(statement(lexer)?);
            }
        } else {
            return Ok(ParseTree::Statements(all_statements));
        }
    }
    Ok(ParseTree::Statements(all_statements))
}

fn statement(lexer: &mut Lexer) -> Result<ParseTree, CompilerError> {
    let next_token = peek_check(lexer, &TokenType::Expression)?;

    if matches!(next_token, Token::Let) {
        Ok(ParseTree::Statement(Box::new(assignment(lexer)?)))
    } else if matches!(next_token, Token::Println) {
        Ok(ParseTree::Statement(Box::new(println(lexer)?)))
    } else {
        Ok(ParseTree::Statement(Box::new(expression(lexer)?)))
    }
}

fn assignment(lexer: &mut Lexer) -> Result<ParseTree, CompilerError> {
    let _ = consume_token(lexer, &Token::Let)?;
    let typ = arg_type(lexer)?;
    let next_token = consume_type(lexer, &TokenType::Identifier)?;
    let id = next_token;
    let _ = consume_token(lexer, &Token::Equals)?;
    Ok(ParseTree::Assignment(Box::new(ParseTree::Identifier(id)), Box::new(typ), Box::new(expression(lexer)?)))
}

fn arg_type(lexer: &mut Lexer) -> Result<ParseTree, CompilerError> {
    let next_token = consume_type(lexer, &TokenType::Type)?;
    Ok(ParseTree::Type(next_token))
}
fn println(lexer: &mut Lexer) -> Result<ParseTree, CompilerError> {
    let _ = consume_token(lexer, &Token::Println)?;
    let _ = consume_token(lexer, &Token::LParen)?;
    let tree = ParseTree::Println(Box::new(expression(lexer)?));
    let _ = consume_token(lexer, &Token::RParen)?;
    Ok(tree)
}

fn expression(lexer: &mut Lexer) -> Result<ParseTree, CompilerError> {
    let mut expression = term(lexer)?;
    use Token::*;

    while let Some(next_token) = peek(lexer)? {
        expression = match next_token {
            Plus | Subtract => {
                next(lexer)?;
                ParseTree::Expression(Box::new(expression), next_token, Box::new(term(lexer)?))
            },
            _ => break
        };
    }

    if let Some(next_token) = peek(lexer)? {
        match next_token {
            Identifier(_) | Number(_) => {
                Err(ParserError::new(
                    ArithmeticExpectedError, lexer,
                ))?
            },
            _ => {},
        }
    }

    Ok(expression)
}

fn term(lexer: &mut Lexer) -> Result<ParseTree, CompilerError> {
    let mut term = factor(lexer)?;
    use Token::*;

    while let Some(next_token) = peek(lexer)? {
        term = match next_token {
            Times | Divide => {
                next(lexer)?;
                ParseTree::Expression(Box::new(term), next_token, Box::new(factor(lexer)?))
            },
            _ => break
        };
    }

    Ok(term)
}

fn factor(lexer: &mut Lexer) -> Result<ParseTree, CompilerError> {
    let next_token = consume_type(lexer, &TokenType::Expression)?;
    use Token::*;

    match next_token {
        Number(_) => Ok(ParseTree::Number(next_token)),
        String(_) => Ok(ParseTree::String(next_token)),
        Identifier(_) => Ok(ParseTree::Identifier(next_token)),
        LParen => {
            let expr = expression(lexer);
            let _ = consume_token(lexer, &RParen)?;
            expr
        },
        _ => unreachable!()
    }
}