use crate::error::error::CompilerError;
use crate::interfaces::lexer_interface::{is_peek_in, consume_token, consume_type, is_empty, is_peek_match_token, is_peek_match_type, next, peek, peek_check};
use crate::parser::error::ParserError;
use crate::lexer::tokens::Token;
use crate::lexer::lexer::Lexer;
use crate::lexer::token_type::TokenType;
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
        todo!()
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
        todo!()
    } else if is_peek_match_token(lexer, &Token::Fn)? {
        Ok(Statement::Function(function(lexer)?))
    } else {
        Ok(Statement::Expression(expression(lexer)?))
    }
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

    let identifier = match consume_type(lexer, &TokenType::Identifier)? {
        Token::Identifier(identifier) => identifier,
        _ => return unreachable!(),
    };

    let _  = consume_token(lexer, &Token::LParen)?;
    let arguments = function_declaration_arguments(lexer)?;
    let _  = consume_token(lexer, &Token::RParen)?;

    let return_type = if is_peek_match_token(lexer, &Token::Arrow)? {
        let _  = consume_token(lexer, &Token::Arrow)?;
        Type::from(consume_type(lexer, &TokenType::Type)?)
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
        let identifier = match consume_type(lexer, &TokenType::Identifier)? {
            Token::Identifier(identifier) => identifier,
            _ => return unreachable!(),
        };
        let _ = consume_token(lexer, &Token::Colon)?;
        let arg_type = Type::from(consume_type(lexer, &TokenType::Type)?);
        arguments.push(TypedArgument {
            identifier,
            arg_type
        })
    }

    while !is_peek_match_token(lexer, &Token::RParen)? {
        let _ = consume_token(lexer, &Token::Comma)?;

        let identifier = match consume_type(lexer, &TokenType::Identifier)? {
            Token::Identifier(identifier) => identifier,
            _ => return unreachable!(),
        };
        let _ = consume_token(lexer, &Token::Colon)?;
        let arg_type = Type::from(consume_type(lexer, &TokenType::Type)?);
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



/* ---------- EXPRESSIONS ---------- */


fn expression(lexer: &mut Lexer) -> Result<ExpressionEnum, CompilerError> {
    let mut value = term(lexer)?;

    while is_peek_in(lexer, &[Token::Plus, Token::Subtract])? {
        let mut operator = Operator::Plus;
        if is_peek_match_token(lexer, &Token::Subtract)? {
            operator = Operator::Subtract;
        }

        let _ = next(lexer)?;

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

    while is_peek_in(lexer, &[Token::Times, Token::Divide])? {
        let mut operator = Operator::Multiply;
        if is_peek_match_token(lexer, &Token::Divide)? {
            operator = Operator::Divide;
        }

        let _ = next(lexer)?;

        value = ExpressionEnum::Expression(Expression {
            left: Box::new(value),
            operator,
            right: Box::new(factor(lexer)?),
        })
    }

    Ok(value)
}

fn factor(lexer: &mut Lexer) -> Result<ExpressionEnum, CompilerError> {
    let next_token = consume_type(lexer, &TokenType::Expression)?;

    use Token::*;
    match next_token {
        Number(num) => {
            Ok(ExpressionEnum::Integer(num.parse::<i64>().unwrap()))
        },
        String(string) => {
            Ok(ExpressionEnum::String(string))
        },
        Identifier(id) => {
            Ok(ExpressionEnum::Identifier(id))
        },
        LParen => {
            let expr = expression(lexer);
            let _ = consume_token(lexer, &Token::RParen)?;

            expr
        },
        _ => todo!()
    }
}