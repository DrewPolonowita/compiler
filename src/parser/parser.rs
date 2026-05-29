use crate::error::error::CompilerError;
use crate::interfaces::lexer_interface::{consume_token, consume_type, is_empty, is_peek_match_token, is_peek_match_type, next, peek, peek_check};
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

    Closure(Box<ParseTree>),
    Function(Token, Box<ParseTree>),

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

            if !is_empty(lexer) & !is_peek_match_token(lexer, &Token::RCurly)? {
                all_statements.push(statement(lexer)?);
            }
        } else {
            if !is_empty(lexer) {
                if !is_peek_match_token(lexer, &Token::LineEnd)? & !is_peek_match_token(lexer, &Token::RCurly)? {
                    consume_token(lexer, &Token::LineEnd)?;
                };
            }

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
    } else if matches!(next_token, Token::LCurly) {
        Ok(ParseTree::Statement(Box::new(closure(lexer)?)))
    } else if matches!(next_token, Token::Fn) {
        Ok(ParseTree::Statement(Box::new(function(lexer)?)))
    } else {
        Ok(ParseTree::Statement(Box::new(expres sion(lexer)?)))
    }
}

fn function(lexer: &mut Lexer) -> Result<ParseTree, CompilerError> {
    let _ = consume_token(lexer, &Token::Fn)?;
    let function_name = consume_type(lexer, &TokenType::Identifier)?;
    let _ = consume_token(lexer, &Token::LParen)?;
    let _ = consume_token(lexer, &Token::RParen)?;
    let stmts = closure(lexer)?;
    Ok(ParseTree::Function(function_name, Box::new(stmts)))
}

fn closure(lexer: &mut Lexer) -> Result<ParseTree, CompilerError> {
    let _ = consume_token(lexer, &Token::LCurly)?;
    let stmts = statements(lexer)?;
    let _ = consume_token(lexer, &Token::RCurly)?;

    Ok(stmts)
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

    if is_peek_match_type(lexer, &TokenType::Expression)? {
        let _ = peek_check(lexer, &TokenType::Operator)?;
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