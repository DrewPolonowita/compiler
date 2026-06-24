use std::fmt::Display;
use crate::lexer::error::LexerError;
use crate::parser::error::ParserError;
use crate::semantics::error::SemanticError;

pub enum CompilerError {
    LexerError(LexerError),
    ParserError(ParserError),
    SemanticError(SemanticError)
}

impl From<LexerError> for CompilerError {
    fn from(err: LexerError) -> Self {
        Self::LexerError(err)
    }
}

impl From<ParserError> for CompilerError {
    fn from(err: ParserError) -> Self {
        Self::ParserError(err)
    }
}

impl From<SemanticError> for CompilerError {
    fn from(err: SemanticError) -> Self {
        Self::SemanticError(err)
    }
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilerError::LexerError(err) => Display::fmt(&err, f),
            CompilerError::ParserError(err) => Display::fmt(&err, f),
            CompilerError::SemanticError(err) => Display::fmt(&err, f)
        }
    }
}