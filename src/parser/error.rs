use std::fmt::Display;
use crate::error::create_error_diagram::{create_error_diagram, get_line, get_next_line};
use crate::lexer::lexer::Lexer;

pub struct ParserError {
    error_type: ErrorType,
    index: usize,
    program: String,
}

pub enum ErrorType {
    ArithmeticExpectedError,
    UnmatchedParenthesis
}

/// ErrorType Display implementations. Converts the enum name to a String
impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ErrorType::*;

        let (str, code) = match self {
            ArithmeticExpectedError => ("Expected arithmetic operator", "e35013"),
            UnmatchedParenthesis => ("Expected ')'", "e35013"),
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
        let next_line = get_next_line(self.program.as_str(), self.index + line.len() + 1 - column_no);
        let diagram = create_error_diagram(line, next_line, &line_no, &column_no);

        let info = "For more information about this error try (NOT YET IMPLEMENTED)!";

        let _ =f.write_str(&format!("\n{}\n{}\n{}", self.error_type, diagram, info));
        Ok(())
    }
}