#[derive(Debug)]
#[allow(dead_code)]
pub enum Token {
    // Keywords
    Println,
    Let,

    // Character tokens
    LineEnd,
    LParen,
    RParen,
    Equals,
    Plus,
    Subtract,
    Times,
    Divide,

    // Types
    IntType,
    StringType,
    BoolType,

    Number_,
    Number(String),
    Identifier_,
    Identifier(String),
}

pub const TOKENS: [(Token, &str); 15] = [
    (Token::Println, "^println"),
    (Token::Let, "^let"),

    (Token::LineEnd, "^;"),
    (Token::LParen, "^\\("),
    (Token::RParen, "^\\)"),
    (Token::Equals, "^="),
    (Token::Plus, "^\\+"),
    (Token::Subtract, "^-"),
    (Token::Times, "^\\*"),
    (Token::Divide, "^/"),

    (Token::IntType, "^int"),
    (Token::StringType, "^str"),
    (Token::BoolType, "^bool"),

    (Token::Number_, "^\\d+"),
    (Token::Identifier_, "^[A-Za-z_]+"),
];

impl Token {
    pub fn remove___(self, token_value: &str) -> Self {
        use Token::*;

        match self {
            Number_ => {Number(token_value.to_string())},
            Identifier_ => {Identifier(token_value.to_string())},
            _ => self
        }
    }
}