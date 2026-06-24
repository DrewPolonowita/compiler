use crate::error::error::CompilerError;

use crate::interfaces::lexer_interface::*;

use crate::lexer::tokens::Token;
use crate::lexer::lexer::Lexer;
use crate::parser::parse_expressions::expression;
use crate::parser::parse_functions::function;
use crate::parser::parse_if_statements::if_statement;
use crate::parser::parse_tree::*;
use crate::parser::parse_while_loop::while_loop;

impl ParseTree {
    pub fn parse(mut lexer: Lexer) -> Result<ParseTree, CompilerError> {
        program(&mut lexer)
    }
}

fn program(lexer: &mut Lexer) -> Result<ParseTree, CompilerError> {
    Ok(ParseTree {
        statements: statements(lexer)?,
    })
}

pub(crate) fn statements(lexer: &mut Lexer) -> Result<Statements, CompilerError> {
    let mut statement_list = Vec::new();
    statement_list.push(statement(lexer)?);

    while is_peek_match_token(lexer, &Token::LineEnd)? {
        let _ = next(lexer)?;

        if is_empty(lexer) || is_peek_match_token(lexer, &Token::RCurly)? {
            break;
        }
        statement_list.push(statement(lexer)?);
    }

    if !is_empty(lexer) && !is_peek_match_token(lexer, &Token::RCurly)? {
        let _ = consume_token(lexer, &Token::LineEnd)?;
        unreachable!();
    }

    Ok(Statements {
        statements: statement_list,
    })
}

fn statement(lexer: &mut Lexer) -> Result<Statement, CompilerError> {
    if is_peek_match_token(lexer, &Token::If)? {
        Ok(Statement::IfStatement(if_statement(lexer)?))
    } else if is_peek_match_token(lexer, &Token::While)? {
        Ok(Statement::WhileLoop(while_loop(lexer)?))
    } else if is_peek_match_token(lexer, &Token::For)? {
        todo!()
    } else if is_peek_match_token(lexer, &Token::Let)? {
        Ok(Statement::Assignment(assignment(lexer)?))
    } else if is_peek_match_token(lexer, &Token::Fn)? {
        Ok(Statement::Function(function(lexer)?))
    } else if let Some(Ok(next_next_token)) = lexer.peek_second() && matches!(next_next_token, Token::Equals) {
        Ok(Statement::Reassignment(reassignment(lexer)?))
    } else if is_peek_match_token(lexer, &Token::Return)? {
        todo!()
    } else {
        Ok(Statement::Expression(expression(lexer)?))
    }
}


fn assignment(lexer: &mut Lexer) -> Result<Assignment, CompilerError> {
    let _ = consume_token(lexer, &Token::Let)?;
    let return_type = consume_type(lexer)?;
    let identifier = consume_identifier(lexer)?;
    let _ = consume_token(lexer, &Token::Equals)?;
    let value = expression(lexer)?;
    Ok(Assignment {
        identifier,
        return_type,
        expression: value
    })
}

fn reassignment(lexer: &mut Lexer) -> Result<Reassignment, CompilerError> {
    let identifier = consume_identifier(lexer)?;
    let _ = consume_token(lexer, &Token::Equals)?;
    let value = expression(lexer)?;
    Ok(Reassignment {
        identifier,
        expression: value
    })
}

pub fn closure(lexer: &mut Lexer) -> Result<Statements, CompilerError> {
    let _ = consume_token(lexer, &Token::LCurly)?;
    let stmts = crate::parser::parser::statements(lexer);
    let _ = consume_token(lexer, &Token::RCurly)?;
    stmts
}