use std::fmt::Display;
use crate::error::create_error_diagram::{create_error_diagram, get_line, get_next_line};

#[derive(Clone, Debug)]
pub struct LexerError {
    error_type: ErrorType,
    index: usize,
    program: String

}
#[derive(Clone, Debug)]
pub enum ErrorType {
    UnexpectedCharacter,
    UnmatchedStringLiteral,
}

/// Returns a LexerError from the program and the index the Lexer stopped at. This is
/// done by running tests and extracting the line to create an error message
pub fn get_error(program: &str, index: usize) -> LexerError {
    let error_type = class_error(&program.chars().nth(index).unwrap());

    LexerError {
        error_type,
        index,
        program: program.to_string()
    }
}

/// Classes an error enum with the error
fn class_error(ch: &char) -> ErrorType{
    match ch {
        '\"' => ErrorType::UnmatchedStringLiteral,
        _ => ErrorType::UnexpectedCharacter
    }
}

/// ErrorType Display implementations. Converts the enum name to a String
impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (str, code) = match self {
            ErrorType::UnexpectedCharacter => ("Unexpected Character", "e35013"),
            ErrorType::UnmatchedStringLiteral => ("Unmatched String Literal", "e35014"),
        };

        let begin = format!("error[{}]:", code);

        let _ = f.write_str(&format!("{} {}", begin, str));

        Ok(())
    }
}
/// Display implementations for LexerError. Contains a diagram with a pointer
impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

        let (line, line_no, column_no) = get_line(self.program.as_str(), self.index);
        let next_line = get_next_line(self.program.as_str(), self.index + line.len() + 1 - column_no);
        let diagram = create_error_diagram(line, next_line, &line_no, &column_no);

        let info = "For more information about this error try (NOT YET IMPLEMENTED)!";

        let _ =f.write_str(&format!("\n{}\n{}\n{}", self.error_type, diagram, info));
        Ok(())
    }
}