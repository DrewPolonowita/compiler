//! # Lexer Interface
//!
//! This script in intended to act as an interface to the lexer. So that if the `Lexer` is changed
//! only this interface needs to be updated. The interface is also designed such that minimum
//! verbosity is required during the parsing phase of compilation.
//!
//! # Public Methods
//!
//! fn consume_type(lexer: &mut Lexer, expected_type: &TokenType) -> Result<Token, CompilerError>
//! fn consume_token(lexer: &mut Lexer, expected_token: &Token) -> Result<Token, CompilerError>
//! fn next(lexer: &mut Lexer) -> Result<Option<Token>, CompilerError>
//! fn is_empty(lexer: &Lexer) -> bool
//! fn peek(lexer: &Lexer) -> Result<Option<Token>, CompilerError>
//! fn peek_check(lexer: &mut Lexer, expected_token: &TokenType) -> Result<Token, CompilerError>

use crate::error::error::CompilerError;
use crate::interfaces::token_type::{Factor, TokenType};
use crate::lexer::lexer::Lexer;
use crate::lexer::tokens::Token;
use crate::parser::error::{ErrorType, ParserError};
use crate::parser::parse_tree::{Operator, Type};

/// Consumes a `Token` and checks to see it matches the expected `Token`. If so the expected
/// `Token` is returned otherwise a `CompilerError` is returned
/// # Arguments
/// - `lexer`: A mutable reference to a `Lexer` with the program being parsed
/// - `expected_token`: A reference to a `Token` which is the expected output of the `Lexer`
/// # Returns
/// A `Result` enum, with an `Ok` value `Token` or an `Err` value `CompilerError`
pub fn consume_token(lexer: &mut Lexer, expected_token: &Token) -> Result<Token, CompilerError> {
    match lexer.next() {
        Some(token) => {
            let token = token?;

            if token == *expected_token {
                Ok(token)

            } else {
                lexer.step_back();
                let _ = map_token_with_token_to_error(lexer, expected_token, &token)?;
                unreachable!();
            }
        },
        None => {
            lexer.step_back();

            let _ = map_token_to_error(lexer, &expected_token)?;
            unreachable!();
        },
    }
}

/// Gets the next `Token` while consuming it
/// # Arguments
/// - `lexer` A mutable reference to a lexer with the program being parsed
/// # Returns
/// The next `Token` from the `Lexer`
pub fn next(lexer: &mut Lexer) -> Result<Option<Token>, CompilerError> {
    match lexer.next() {
        Some(token) => {
            let token = token?;
            Ok(Some(token))
        },
        None => {
            Ok(None)
        }
    }
}

/// Returns true if the `Lexer` is empty
/// # Arguments
/// - `lexer` A mutable reference to a lexer with the program being parsed
/// # Returns
/// True if the `Lexer` is empty, meaning the `Lexer` has no more `Tokens`
pub fn is_empty(lexer: &Lexer) -> bool {
    lexer.is_empty()
}

/// Returns the next `Token` without consuming it
/// # Arguments
/// - `lexer` A mutable reference to a lexer with the program being parsed
/// # Returns
/// A result enum with an `Option<Token>` or a `CompilerError`
pub fn peek(lexer: &Lexer) -> Result<Option<Token>, CompilerError> {
    match lexer.peek() {
        Some(token) => {
            let token = token?;
            Ok(Some(token))
        },
        None => {
            Ok(None)
        }
    }
}

pub fn consume_factor(lexer: &mut Lexer) -> Result<Factor, CompilerError> {
    let factor_token = check_factor(lexer);
    next(lexer)?;
    factor_token
}

pub fn check_factor(lexer: &mut Lexer) -> Result<Factor, CompilerError> {
    match lexer.peek() {
        Some(token) => {
            let token = token?;

            match Factor::option_from(&token) {
                Some(token) => Ok(token),
                None => {
                    let _ = map_token_type_with_token_to_error(lexer, &TokenType::Factor, &token)?;
                    unreachable!();
                }
            }
        },
        None => {

            let _ = map_token_type_to_error(lexer, &TokenType::Factor)?;
            unreachable!();
        },
    }
}


pub fn consume_type(lexer: &mut Lexer) -> Result<Type, CompilerError> {
    let type_token = check_type(lexer);
    next(lexer)?;
    type_token
}

pub fn check_type(lexer: &mut Lexer) -> Result<Type, CompilerError> {
    match lexer.peek() {
        Some(token) => {
            let token = token?;

            match Type::option_from(&token) {
                Some(token) => Ok(token),
                None => {
                    let _ = map_token_type_with_token_to_error(lexer, &TokenType::Type, &token)?;
                    unreachable!();
                }
            }
        },
        None => {

            let _ = map_token_type_to_error(lexer, &TokenType::Type)?;
            unreachable!();
        },
    }
}


pub fn consume_identifier(lexer: &mut Lexer) -> Result<String, CompilerError> {
    let identifier = check_identifier(lexer);
    next(lexer)?;
    identifier
}

pub fn check_identifier(lexer: &mut Lexer) -> Result<String, CompilerError> {
    match lexer.peek() {
        Some(token) => {
            let token = token?;

            match token {
                Token::Identifier(identifier) => {
                    Ok(identifier)
                },
                _ => {
                    let _ = map_token_type_with_token_to_error(lexer, &TokenType::Identifier, &token)?;
                    unreachable!();
                }
            }
        },
        None => {

            let _ = map_token_type_to_error(lexer, &TokenType::Identifier)?;
            unreachable!();
        },
    }
}


pub fn consume_operator(lexer: &mut Lexer) -> Result<Operator, CompilerError> {
    let identifier = check_operator(lexer);
    next(lexer)?;
    identifier
}

pub fn check_operator(lexer: &mut Lexer) -> Result<Operator, CompilerError> {
    match lexer.peek() {
        Some(token) => {
            let token = token?;

            match Operator::option_from(&token) {
                Some(token) => Ok(token),
                None => {
                    let _ = map_token_type_with_token_to_error(lexer, &TokenType::Operator, &token)?;
                    unreachable!();
                }
            }
        },
        None => {

            let _ = map_token_type_to_error(lexer, &TokenType::Operator)?;
            unreachable!();
        },
    }
}


/// Checks if the next `Token` is in the `TokenType` without consuming the token.
/// # Arguments
/// - `lexer` A mutable reference to a lexer with the program being parsed
/// - `expected_token` A reference to a `TokenType` to match the given token
/// # Returns
/// A result enum with `Ok(true)` if the value is the `TokenType`. Otherwise, `Ok(false)`. If an error
/// is encountered then `CompilerError` is returned
// pub fn is_peek_match_type(lexer: &mut Lexer, expected_type: &TokenType) -> Result<bool, CompilerError> {
//     match peek(lexer)? {
//         Some(token) => {
//             if token.is_member(expected_type) {
//                 Ok(true)
//             } else {
//                 Ok(false)
//             }
//         },
//         None => {
//             Ok(false)
//         },
//     }
// }

/// Checks if the next `Token` is in the `Token` without consuming the token.
/// # Arguments
/// - `lexer` A mutable reference to a lexer with the program being parsed
/// - `expected_token` A reference to a `TokenType` to match the given token
/// # Returns
/// A result enum with `Ok(true)` if the value is the `Token`. Otherwise, `Ok(false)`. If an error
/// is encountered then `CompilerError` is returned
pub fn is_peek_match_token(lexer: &mut Lexer, expected_token: &Token) -> Result<bool, CompilerError> {
    match peek(lexer)? {
        Some(token) => {
            if token == *expected_token {
                Ok(true)
            } else {
                Ok(false)
            }
        },
        None => {
            Ok(false)
        },
    }
}

pub fn is_peek_in(lexer: &mut Lexer, expected_tokens: &[Token]) -> Result<bool, CompilerError> { //
    match peek(lexer)? {
        Some(token) => {
            for expected_token in expected_tokens {
                if token == *expected_token {
                    return Ok(true)
                }
            }

            Ok(false)
        },
        None => {
            Ok(false)
        },
    }
}

// Maps an expected token type into an error when there is no more tokens
fn map_token_type_to_error(lexer: &mut Lexer, token_type: &TokenType) -> Result<(), CompilerError> {
    let parser_error = ParserError::new(
        ErrorType::ExpectedType(token_type.clone()), lexer,
    );

    Err(CompilerError::from(parser_error))
}

// Maps an expected token type into an error when the given token doesn't match the token type
fn map_token_type_with_token_to_error(lexer: &mut Lexer, token_type: &TokenType, token: &Token) -> Result<Token, CompilerError> {
    let parser_error = ParserError::new(
        ErrorType::UnexpectedType(token_type.clone(), token.clone()), lexer,
    );

    Err(CompilerError::from(parser_error))
}

// Maps an expected token into an error when there is no more tokens
fn map_token_to_error(lexer: &mut Lexer, expected_token: &Token) -> Result<(), CompilerError> {
    let parser_error = ParserError::new(
        ErrorType::ExpectedToken(expected_token.clone()), lexer,
    );

    Err(CompilerError::from(parser_error))
}

// Maps an expected token into an error when the given token doesn't match the token
fn map_token_with_token_to_error(lexer: &mut Lexer, expected_token: &Token, token: &Token) -> Result<Token, CompilerError> {
    let parser_error = ParserError::new(
        ErrorType::UnexpectedToken(expected_token.clone(), token.clone()), lexer,
    );

    Err(CompilerError::from(parser_error))
}