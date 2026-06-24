use crate::error::error::CompilerError;

use crate::interfaces::lexer_interface::*;
use crate::interfaces::token_type::*;

use crate::lexer::tokens::Token;
use crate::lexer::lexer::Lexer;
use crate::parser::parse_expressions::expression;
use crate::parser::parse_tree::*;


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

fn statements(lexer: &mut Lexer) -> Result<Statements, CompilerError> {
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
        todo!()
    } else if is_peek_match_token(lexer, &Token::For)? {
        todo!()
    } else if is_peek_match_token(lexer, &Token::Let)? {
        Ok(Statement::Assignment(assignment(lexer)?))
    } else if is_peek_match_token(lexer, &Token::Fn)? {
        Ok(Statement::Function(function(lexer)?))
    } else {
        Ok(Statement::Expression(expression(lexer)?))
    }
}

/* ---------- Assignment Statement ---------- */


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


/* ---------- If Statements ---------- */

fn if_statement(lexer: &mut Lexer) -> Result<IfStatement, CompilerError> {
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

/* ---------- FUNCTIONS ---------- */

fn function(lexer: &mut Lexer) -> Result<Function, CompilerError> {
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

fn closure(lexer: &mut Lexer) -> Result<Statements, CompilerError> {
    let _ = consume_token(lexer, &Token::LCurly)?;
    let stmts = statements(lexer);
    let _ = consume_token(lexer, &Token::RCurly)?;
    stmts
}