use std::fmt::Display;
use crate::error::create_error_diagram::{create_error_diagram, get_line, get_next_line, get_prev_line};
use crate::lexer::lexer::Lexer;
use crate::interfaces::token_type::TokenType;
use crate::lexer::tokens::Token;

pub struct ParserError {
    error_type: ErrorType,
    index: usize,
    program: String,
}

pub enum ErrorType {
    ArithmeticExpectedError,
    UnmatchedParenthesis,
    ExpectedType(
        TokenType, //expected
    ),
    UnexpectedType(
        TokenType, //expected
        Token, //got
    ),
    ExpectedToken(
        Token, //expected
    ),
    UnexpectedToken(
        Token, //expected
        Token, //got
    )
}

/// ErrorType Display implementations. Converts the enum name to a String
impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ErrorType::*;

        let (str, code) = match self {
            ArithmeticExpectedError => ("Expected arithmetic operator".to_string(), "e35013"),
            UnmatchedParenthesis => ("Expected ')'".to_string(), "e35014"),
            ExpectedType(expected) => {
                let string = format!("expected {}", expected);
                (string, "e35015")
            },
            UnexpectedType(expected, got) => {
                let string = format!("expected {}, found \"{}\"", expected, got);
                (string, "e35015")
            },
            ExpectedToken(expected) => {
                let string = format!("expected {}", expected);
                (string, "e35015")
            },
            UnexpectedToken(expected, got) => {
                let string = format!("expected {}, found \"{}\"", expected, got);
                (string, "e35015")
            },
        };

        let begin = format!("error[{}]:", code);
        let _ = f.write_str(&format!("{} {}", begin, str));

        Ok(())
    }
}

impl ParserError {
    pub fn new(error_type: ErrorType, lexer: &mut Lexer) -> Self {
        Self {
            error_type,
            index: lexer.get_current_index(),
            program: lexer.get_program().to_string(),
        }
    }
}

/// Display implementations for LexerError. Contains a diagram with a pointer
impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

        let (line, line_no, column_no) = get_line(self.program.as_str(), self.index);

        let prev_line = get_prev_line(self.program.as_str(), self.index as isize - column_no as isize - 1);
        let next_line = get_next_line(self.program.as_str(), self.index + line.len() + 1 - column_no);
        let diagram = create_error_diagram(line, next_line, prev_line, &line_no, &column_no);

        let info = "For more information about this error try (NOT YET IMPLEMENTED)!";

        let _ =f.write_str(&format!("\n{}\n{}\n{}", self.error_type, diagram, info));
        Ok(())
    }
}