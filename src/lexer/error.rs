use std::alloc::System;
use std::fmt::Display;

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

/// Extracts the line, line no and col no from the program and the line the error occurred at
fn get_line(program: &str, index: usize) -> (&str, usize, usize) {
    let start = program[..index]
        .rfind('\n')
        .map(|i| i + 1)
        .unwrap_or(0);

    let end = program[index..]
        .find('\n')
        .map(|i| index + i)
        .unwrap_or(program.len());

    let line_no = program[..index].matches("\n").count() + 1;
    let column_no = index - start;

    (&program[start..end], line_no, column_no)
}

/// Gets the next line after the error line. Returns an option as the error line could be the final line
fn get_next_line(program: &str, index: usize) -> Option<&str> {
    if index >= program.len() {
        None
    } else {
        let (line, _, _) = get_line(program, index);
        Some(line)
    }
}

/// Creates a diagram with an arrow pointing at the line and column the error occurred in
fn create_error_diagram(line: &str, next_line: Option<&str>, line_no: &usize, column_no: &usize) -> String {

    let gap = line_no / 10 + 2;

    let error_line = format!("{}--> filepath:{}:{}", " ".repeat(gap-2), line_no, column_no);

    let first_line = format!("{}|", " ".repeat(gap));
    let second_line = format!("{}{}|     {}", line_no, " ".repeat(gap - line_no / 10 - 1), line);
    let third_line = format!("{}|  ___{}^", " ".repeat(gap), "_".repeat(*column_no));
    let fourth_line = match next_line {
        Some(next_line) => format!("{}{}| |   {}", line_no+1, " ".repeat(gap - (line_no+1) / 10 - 1), next_line),
        None => format!("{}| |", " ".repeat(gap))
    };

    format!("{}\n{}\n{}\n{}\n{}\n", error_line, first_line, second_line, third_line, fourth_line)
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