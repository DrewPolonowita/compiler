use crate::error::error::CompilerError;
use crate::parser::error::{ErrorType, ParserError};
use crate::lexer::tokens::Token;
use crate::lexer::lexer::Lexer;
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

    while let Some(next_token) = lexer.peek() {
        let next_token = next_token?;
        if matches!(next_token, Token::LineEnd) {
            lexer.next();
            if !lexer.is_empty() {
                all_statements.push(statement(lexer)?);
            }
        } else {
            return Ok(ParseTree::Statements(all_statements));
        }
    }

    Ok(ParseTree::Statements(all_statements))
}

fn statement(lexer: &mut Lexer) -> Result<ParseTree, CompilerError> {
    let Some(next_token) = lexer.peek() else {
        todo!();
    };
    let next_token = next_token?;

    if matches!(next_token, Token::Let) {
        Ok(ParseTree::Statement(Box::new(assignment(lexer)?)))
    } else if matches!(next_token, Token::Println) {
        Ok(ParseTree::Statement(Box::new(println(lexer)?)))
    } else {
        todo!();
    }
}

fn assignment(lexer: &mut Lexer) -> Result<ParseTree, CompilerError> {
    let Some(next_token) = lexer.next() else {todo!()};
    let next_token = next_token?;
    if !matches!(next_token, Token::Let) {todo!()};

    let typ = argtype(lexer)?;

    let Some(next_token) = lexer.next() else {todo!()};
    let next_token = next_token?;
    if !matches!(next_token, Token::Identifier(_)) {todo!()};

    let id = next_token;

    let Some(next_token) = lexer.next() else {todo!()};
    let next_token = next_token?;
    if !matches!(next_token, Token::Equals) {todo!()};

    Ok(ParseTree::Assignment(Box::new(ParseTree::Identifier(id)), Box::new(typ), Box::new(expression(lexer)?)))

}

fn argtype(lexer: &mut Lexer) -> Result<ParseTree, CompilerError> {
    let Some(next_token) = lexer.next() else {todo!()};
    let next_token = next_token?;

    use Token::*;
    match next_token {
        IntType | StringType | BoolType => Ok(ParseTree::Type(next_token)),
        _ => todo!()
    }
}
fn println(lexer: &mut Lexer) -> Result<ParseTree, CompilerError> {
    let Some(next_token) = lexer.next() else {todo!()};
    let next_token = next_token?;
    if !matches!(next_token, Token::Println) {todo!()};

    let Some(next_token) = lexer.next() else {todo!()};
    let next_token = next_token?;
    if !matches!(next_token, Token::LParen) {todo!()};

    let tree = ParseTree::Println(Box::new(expression(lexer)?));

    let Some(next_token) = lexer.next() else {todo!()};
    let next_token = next_token?;
    if !matches!(next_token, Token::RParen) {todo!()};

    Ok( tree)
}

fn expression(lexer: &mut Lexer) -> Result<ParseTree, CompilerError> {
    let mut expression = term(lexer)?;
    use Token::*;

    while let Some(next_token) = lexer.peek() {
        let next_token = next_token?;

        expression = match next_token {
            Times | Divide => {
                lexer.next();
                ParseTree::Expression(Box::new(expression), next_token, Box::new(term(lexer)?))
            },
            _ => break
        };
    }

    if let Some(next_token) = lexer.peek() {
        let next_token = next_token?;

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

    while let Some(next_token) = lexer.peek() {
        let next_token = next_token?;
        use Token::*;

        term = match next_token {
            Plus | Subtract => {
                lexer.next();
                ParseTree::Expression(Box::new(term), next_token, Box::new(factor(lexer)?))
            },
            _ => {
                break
            }
        };
    }

    Ok(term)
}

fn factor(lexer: &mut Lexer) -> Result<ParseTree, CompilerError> {
    let Some(next_token) = lexer.next() else {todo!()};
    let next_token = next_token?;
    use Token::*;

    match next_token {
        Number(_) => Ok(ParseTree::Number(next_token)),
        String(_) => Ok(ParseTree::String(next_token)),
        Identifier(_) => Ok(ParseTree::Identifier(next_token)),
        LParen => {
            lexer.next();
            let expr = expression(lexer);

            let Some(next_token) = lexer.next() else {todo!()};
            let next_token = next_token?;

            if !matches!(next_token, RParen) {
                todo!()
            };

            expr
        },
        _ => todo!()
    }
}