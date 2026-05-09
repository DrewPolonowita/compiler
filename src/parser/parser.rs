use crate::error::error::CompilerError;
use crate::lexer::error::LexerError;
use crate::lexer::tokens::Token;
use crate::lexer::lexer::Lexer;

#[derive(Debug)]
#[allow(dead_code)]
pub enum ParseTree {
    Program(Box<ParseTree>),
    Statements(Vec<ParseTree>),
    Statement(Box<ParseTree>),
    Assignment(Box<ParseTree>, Box<ParseTree>, Box<ParseTree>),
    Expression(Vec<ParseTree>, Vec<ParseTree>),
    Term(Vec<ParseTree>, Vec<ParseTree>),
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
    let mut all_terms = Vec::from([term(lexer)?]);
    let mut all_operands = Vec::new();

    while let Some(next_token) = lexer.peek() {
        let next_token = next_token?;
        if matches!(next_token, Token::Plus) || matches!(next_token, Token::Subtract) {

            all_operands.push(arithmetic(lexer)?);
            all_terms.push(term(lexer)?);
        } else {
            return Ok(ParseTree::Expression(all_terms, all_operands));
        }
    }

    Ok(ParseTree::Expression(all_terms, Vec::new()))
}

fn term(lexer: &mut Lexer) -> Result<ParseTree, CompilerError> {
    let mut all_factors = Vec::from([factor(lexer)?]);
    let mut all_operands = Vec::new();

    while let Some(next_token) = lexer.peek() {
        let next_token = next_token?;

        if matches!(next_token, Token::Times) || matches!(next_token, Token::Divide) {
            all_operands.push(arithmetic(lexer)?);
            all_factors.push(factor(lexer)?);
        } else {
            return Ok(ParseTree::Term(all_factors, all_operands));
        }
    }

    Ok(ParseTree::Term(all_factors, Vec::new()))
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

            if !matches!(next_token, Token::RParen) {todo!()};

            expr
        },
        _ => todo!()
    }
}

fn arithmetic(lexer: &mut Lexer) -> Result<ParseTree, CompilerError> {
    let Some(next_token) = lexer.next() else {todo!()};
    let next_token = next_token?;

    use Token::*;
    match next_token {
        Plus => Ok(ParseTree::Arithmetic(next_token)),
        Subtract => Ok(ParseTree::Arithmetic(next_token)),
        Times => Ok(ParseTree::Arithmetic(next_token)),
        Divide => Ok(ParseTree::Arithmetic(next_token)),
        _ => todo!()
    }
}