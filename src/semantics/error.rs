use std::fmt::Display;
use crate::error::create_error_diagram::{create_error_diagram, get_line, get_next_line, get_prev_line};
use crate::lexer::lexer::Lexer;
use crate::interfaces::token_type::TokenType;
use crate::lexer::tokens::Token;
use crate::parser::parse_tree::{Operator, Type};

pub struct SemanticError {
    error_type: ErrorType,
}

pub enum ErrorType {
    UnsupportedOperation(Type, Type, Operator),
    UnmatchedTypes(String, Type, Type)
}

/// ErrorType Display implementations. Converts the enum name to a String
impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ErrorType::*;

        let (str, code) = match self {
            UnsupportedOperation(left, right, op) => {
                let string = format!(
                    "unsupported operation '{}' between types '{:?}' and '{:?}'", op.symbol(), left , right
                );
                (string, "e35015")
            },
            UnmatchedTypes(var_name, expected_type, actual_type) => {
                let string = format!(
                    "'{}' is of type '{:?}' but given type '{:?}'", var_name, expected_type, actual_type
                );
                (string, "e35015")
            },
        };

        let begin = format!("error[{}]:", code);
        let _ = f.write_str(&format!("{} {}", begin, str));

        Ok(())
    }
}

impl SemanticError {
    pub fn new(error_type: ErrorType) -> Self {
        Self {
            error_type,
        }
    }
}

/// Display implementations for LexerError. Contains a diagram with a pointer
impl Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

        let info = "For more information about this error try (NOT YET IMPLEMENTED)!";

        let _ =f.write_str(&format!("\n{}\n{}", self.error_type, info));
        Ok(())
    }
}