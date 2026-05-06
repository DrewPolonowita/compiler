use regex::Regex;
use crate::lexer::tokens::{Token, TOKENS};

pub struct Lexer {
    program: String,
    current_index: usize,
    next_token: Option<Token>
}

fn get_longest_matched_string(text : &str) -> (Option<Token>, usize) {
    let mut longest_token: Option<Token> = None;
    let mut longest_string_length: usize = 0;

    for (mut token, pattern) in TOKENS {
        let re = Regex::new(pattern).unwrap();

        if let Some(matched_token) = re.find(text) {
            let matched_token = matched_token.as_str();

            longest_token = match longest_token {

                Some(longest_token) => {
                    if matched_token.len() > longest_string_length {
                        longest_string_length = matched_token.len();
                        token = token.remove___(matched_token);
                        Some(token)
                    } else {
                        Some(longest_token)
                    }
                },
                None => {
                    longest_string_length = matched_token.len();
                    token = token.remove___(matched_token);
                    Some(token)
                }
            }
        }
    }

    (longest_token, longest_string_length)
}


impl Lexer {
    pub fn new(program: String) -> Lexer {
        let mut lexer = Self {
            program,
            current_index: 0usize,
            next_token: None
        };

        lexer.next_token();

        lexer
    }

    fn skip_whitespace(&mut self) {

        let re = Regex::new(r"^\s+").unwrap();
        if let Some(matched_whitespace) = re.find(&self.program[self.current_index..]) {
            self.current_index += matched_whitespace.end();
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        let old_token = self.next_token.take();

        self.skip_whitespace();
        let (token, length) = get_longest_matched_string(&self.program[self.current_index..]);

        self.current_index += length;
        self.next_token = token;

        old_token
    }

    pub fn peek(&self) -> &Option<Token> {
        return &self.next_token;
    }

    pub fn is_empty(&self) -> bool { return matches!(self.next_token, None); }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}