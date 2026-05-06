use crate::lexer::tokens::Token;
use crate::lexer::lexer::Lexer;

#[derive(Debug)]
#[allow(dead_code)]
pub enum ParseTree {
    Program(Box<ParseTree>),
    Statements(Vec<ParseTree>),
    Statement(Box<ParseTree>),
    Assignment(Box<ParseTree>, Box<ParseTree>, Box<ParseTree>),
    Expression(Vec<ParseTree>, Vec<Token>),
    Term(Vec<ParseTree>, Vec<Token>),
    Factor(Box<ParseTree>),
    Println(Box<ParseTree>),

    Type(Token),

    Arithmetic(Token),
    Number(Token),
    String(Token),
    Identifier(Token),
}

impl ParseTree {
    pub fn parse(mut lexer: Lexer) -> ParseTree {
        program(&mut lexer)
    }
}

fn program(lexer: &mut Lexer) -> ParseTree {
    ParseTree::Program(Box::new(statements(lexer)))
}

fn statements(lexer: &mut Lexer) -> ParseTree {
    let mut all_statements = Vec::from([statement(lexer)]);

    while let Some(next_token) = lexer.peek() {
        if matches!(next_token, Token::LineEnd) {
            lexer.next();
            if !lexer.is_empty() {
                all_statements.push(statement(lexer));
            }
        }
    }

    ParseTree::Statements(all_statements)
}

fn statement(lexer: &mut Lexer) -> ParseTree {
    let Some(next_token) = lexer.peek() else {
        todo!();
    };

    if matches!(next_token, Token::Let) {
        ParseTree::Statement(Box::new(assignment(lexer)))
    } else if matches!(next_token, Token::Println) {
        ParseTree::Statement(Box::new(println(lexer)))
    } else {
        todo!();
    }
}

fn assignment(lexer: &mut Lexer) -> ParseTree {
    let Some(next_token) = lexer.next() else {todo!()};
    if !matches!(next_token, Token::Let) {todo!()};

    let typ = argtype(lexer);

    let Some(next_token) = lexer.next() else {todo!()};
    if !matches!(next_token, Token::Identifier(_)) {todo!()};

    let id = next_token;

    let Some(next_token) = lexer.next() else {todo!()};
    if !matches!(next_token, Token::Equals) {todo!()};

    ParseTree::Assignment(Box::new(ParseTree::Identifier(id)), Box::new(typ), Box::new(expression(lexer)))

}

fn argtype(lexer: &mut Lexer) -> ParseTree {
    let Some(next_token) = lexer.next() else {todo!()};

    use Token::*;

    match next_token {
        IntType | StringType | BoolType => ParseTree::Type(next_token),
        _ => todo!()
    }
}
fn println(lexer: &mut Lexer) -> ParseTree {
    let Some(next_token) = lexer.next() else {todo!()};
    if !matches!(next_token, Token::Println) {todo!()};

    let Some(next_token) = lexer.next() else {todo!()};
    if !matches!(next_token, Token::LParen) {todo!()};

    let tree = ParseTree::Println(Box::new(expression(lexer)));

    let Some(next_token) = lexer.next() else {todo!()};
    if !matches!(next_token, Token::RParen) {todo!()};

    tree
}

fn expression(lexer: &mut Lexer) -> ParseTree {
    let mut all_terms = Vec::from([term(lexer)]);
    let mut all_operands = Vec::new();

    while let Some(next_token) = lexer.peek() {
        if matches!(next_token, Token::Plus) || matches!(next_token, Token::Subtract) {
            let Some(operand) = lexer.next() else {todo!()};

            all_operands.push(operand);
            all_terms.push(term(lexer));
        } else {
            return ParseTree::Expression(all_terms, all_operands);
        }
    }

    ParseTree::Expression(all_terms, Vec::new())
}

fn term(lexer: &mut Lexer) -> ParseTree {
    let mut all_factors = Vec::from([factor(lexer)]);
    let mut all_operands = Vec::new();

    while let Some(next_token) = lexer.peek() {
        if matches!(next_token, Token::Times) || matches!(next_token, Token::Divide) {
            let Some(operand) = lexer.next() else {todo!()};
            all_operands.push(operand);
            all_factors.push(factor(lexer));
        } else {
            return ParseTree::Term(all_factors, all_operands);
        }
    }

    ParseTree::Term(all_factors, Vec::new())
}

fn factor(lexer: &mut Lexer) -> ParseTree {
    let Some(next_token) = lexer.next() else {todo!()};

    use Token::*;

    match next_token {
        Number(_) => ParseTree::Number(next_token),
        String(_) => ParseTree::String(next_token),
        Identifier(_) => ParseTree::Identifier(next_token),
        LParen => {
            lexer.next();
            let expr = expression(lexer);

            let Some(next_token) = lexer.next() else {todo!()};
            if !matches!(next_token, Token::RParen) {todo!()};

            expr
        },
        _ => todo!()
    }
}

fn arithmetic(lexer: &mut Lexer) -> ParseTree {
    let Some(next_token) = lexer.next() else {todo!()};

    use Token::*;

    match next_token {
        Plus => ParseTree::Arithmetic(next_token),
        Subtract => ParseTree::Arithmetic(next_token),
        Times => ParseTree::Arithmetic(next_token),
        Divide => ParseTree::Arithmetic(next_token),
        _ => todo!()
    }
}