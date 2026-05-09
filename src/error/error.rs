use std::fmt::Display;
use crate::lexer::error::LexerError;
use crate::parser::error::ParserError;

pub enum CompilerError {
    LexerError(LexerError),
    ParserError(ParserError),
}

impl From<LexerError> for CompilerError {
    fn from(err: LexerError) -> Self {
        CompilerError::LexerError(err)
    }
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilerError::LexerError(err) => Display::fmt(&err, f),
            CompilerError::ParserError(err) => Display::fmt(&err, f),
        }
    }
}