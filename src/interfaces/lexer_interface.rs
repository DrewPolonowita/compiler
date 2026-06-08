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
use crate::lexer::lexer::Lexer;
use crate::lexer::token_type::TokenType;
use crate::lexer::tokens::Token;
use crate::parser::error::{ErrorType, ParserError};

/// Consumes a `Token` and checks to see it matches the expected `TokenType`. If so the expected
/// `Token` is returned otherwise a `CompilerError` is returned
/// # Arguments
/// - `lexer`: A mutable reference to a `Lexer` with the program being parsed
/// - `expected_type`: A reference to a `TokenType` which is the expected output of the `Lexer`
/// # Returns
/// A `Result` enum, with an `Ok` value `Token` or an `Err` value `CompilerError`
pub fn consume_type(lexer: &mut Lexer, expected_type: &TokenType) -> Result<Token, CompilerError> {
    match lexer.next() {
        Some(token) => {
            let token = token?;

            if token.is_member(expected_type) {
                Ok(token)

            } else {
                lexer.step_back();
                let _ = map_token_type_with_token_to_error(lexer, expected_type, &token)?;
                unreachable!();
            }
        },
        None => {
            lexer.step_back();

            let _ = map_token_type_to_error(lexer, &expected_type)?;
            unreachable!();
        },
    }
}

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

/// Gets the next `Token` without consuming and checks to see if it matches the `TokenType`.
/// If the `Token` matches then it's returned. Otherwise, a `CompilerError` is returned
/// # Arguments
/// - `lexer` A mutable reference to a lexer with the program being parsed
/// - `expected_token` A reference to a `TokenType` to match the given token
/// # Returns
/// A result enum with the next `Token` or `CompilerError` if the `Token` doesn't match or exist
pub fn peek_check(lexer: &mut Lexer, expected_token: &TokenType) -> Result<Token, CompilerError> {
    match peek(lexer)? {
        Some(token) => {
            if token.is_member(expected_token) {
                Ok(token)

            } else {
                let _ = map_token_type_with_token_to_error(lexer, expected_token, &token)?;
                unreachable!();
            }
        },
        None => {
            let _ = map_token_type_to_error(lexer, &expected_token)?;
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
pub fn is_peek_match_type(lexer: &mut Lexer, expected_type: &TokenType) -> Result<bool, CompilerError> {
    match peek(lexer)? {
        Some(token) => {
            if token.is_member(expected_type) {
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