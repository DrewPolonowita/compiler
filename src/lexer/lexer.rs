use regex::Regex;
use crate::lexer::error::{get_error, LexerError};
use crate::lexer::tokens::{Token, TOKENS};

pub struct Lexer {
    program: String,
    current_index: usize,
    prev_index: usize
}

impl Lexer {
    /// Creates a new instance of a lexer with some program to tokenize the input.
    /// The input program is tokenized lazily by implementing iterator.
    ///
    /// # Arguments
    ///
    /// * `program` - The program to be tokenized
    ///
    /// # Examples
    ///
    /// ```
    /// let lexer = Lexer::new("let str my_string = \"Hello, world!\";");
    /// assert_eq!(lexer.next(), Some(Ok(Token::Let)));
    /// assert_eq!(lexer.next(), Some(Ok(Token::StringType)));
    /// assert_eq!(lexer.next(), Some(Ok(Token::Identifier("my_string")));
    /// assert_eq!(lexer.next(), Some(Ok(Token::Equals)));
    /// assert_eq!(lexer.next(), Some(Ok(Token::String("Hello, world!")));
    /// assert_eq!(lexer.next(), Some(Ok(Token::LineEnd));
    /// ```
    pub fn new(program: String) -> Lexer {
        let lexer = Self {
            program,
            current_index: 0usize,
            prev_index: 0usize
        };
        lexer
    }

    /// Gets the number of whitespace chars from self.current_index to the next token.
    /// This is used to calculate an effective index
    fn get_no_whitespace_chars(&self) -> usize {
        let mut whitespace = 0;
        let program_bytes = self.program.as_bytes();

        while self.current_index + whitespace < self.program.len() {
            let b = program_bytes[self.current_index + whitespace];
            if b == b' ' || b == b'\n' || b == b'\t' || b == b'\r' {
                whitespace += 1;
            } else {
                break;
            }
        }
        whitespace
    }

    /// Adds the number of whitespace chars to the current index to get the index of the next token
    fn effective_index(&self) -> usize {
        self.current_index + self.get_no_whitespace_chars()
    }

    /// Returns the longest token from the effective index; if an error is met by there not being a valid
    /// next token then a LexerError is returned
    fn get_longest_matched_string(&self) -> Result<(Token, usize), LexerError> {
        let mut longest_token: Option<Token> = None;
        let mut longest_string_length: usize = 0;

        let text = &self.program[self.effective_index()..];

        for (token, pattern) in TOKENS {
            let re = Regex::new(pattern).unwrap();

            if let Some(matched_token) = re.find(text) {
                let matched_token = matched_token.as_str();

                if matched_token.len() > longest_string_length {
                    longest_string_length = matched_token.len();
                    longest_token = Some(token.remove___(matched_token));
                }
            }
        }

        if let Some(longest_token) = longest_token {
            Ok((longest_token, longest_string_length))
        } else {
            Err(get_error(&self.program, self.effective_index()))
        }
    }

    /// Returns the next token and increments the iterator to the next token
    fn next_token(&mut self) -> Result<Token, LexerError> {
        self.prev_index = self.current_index;

        if self.effective_index() >= self.program.len() {
            return Ok(Token::EOF);
        }
        let (token, length) = self.get_longest_matched_string()?;
        self.current_index = self.effective_index() + length;
        Ok(token)
    }

    /// Returns the next token without incrementing the iterator to the next token
    pub fn peek(&self) -> Option<Result<Token, LexerError>> {
        if self.effective_index() >= self.program.len() {
            return None;
        }

        match self.get_longest_matched_string() {
            Ok((token, _)) => Some(Ok(token)),
            Err(e) => Some(Err(e))
        }
    }

    /// Returns true if the iterator is at the end of the file
    pub fn is_empty(&self) -> bool {
        self.effective_index() >= self.program.len()
    }

    /// Returns the program string
    pub fn get_program(&self) -> &str {
        &self.program
    }

    /// Returns the current index
    pub fn get_current_index(&self) -> usize {
        self.effective_index()
    }

    /// Returns to the previous index. Only works once
    /// Does nothing if a step back has been done and next_token hasn't been called
    /// Does nothing if the first token hasn't been read.
    pub fn step_back(&mut self) {
        self.current_index = self.prev_index;
    }
}

/// Iterator implementation for Lexer so that it works with loops easier
impl Iterator for Lexer {
    type Item = Result<Token, LexerError>;
    fn next(&mut self) -> Option<Self::Item> {
        let next_token = self.next_token();

        // Token::EOF indicates the end of the file
        if matches!(next_token, Ok(Token::EOF)) {
            None
        } else {
            Some(next_token)
        }
    }
}