use std::fmt::Display;
use crate::lexer::tokens::Token::{And, LParen, Lt, Or, RParen};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Token {
    // Keywords
    Println,
    Let,
    Fn,
    If,
    Else,
    While,
    For,

    // Character tokens
    LineEnd,
    Colon,
    Comma,
    Arrow,

    LParen,
    RParen,
    LCurly,
    RCurly,

    Equals,
    Plus,
    Subtract,
    Times,
    Divide,
    Not,
    And,
    Or,

    Eq,
    Neq,
    Leq,
    Geq,
    Lt,
    Gt,

    // Types
    IntType,
    StringType,
    BoolType,

    Number_,
    Number(String),
    String_,
    String(String),
    Identifier_,
    Identifier(String),

    True,
    False,

    EOF
}

pub const TOKENS: [(Token, &str); 37] = [
    (Token::Println, "^println"),
    (Token::Let, "^let"),
    (Token::Fn, "^fn"),
    (Token::If, "^if"),
    (Token::Else, "^else"),
    (Token::While, "^while"),
    (Token::For, "^for"),

    (Token::LineEnd, "^;"),
    (Token::Colon, "^:"),
    (Token::Comma, "^,"),
    (Token::Arrow, "^->"),
    (Token::LParen, "^\\("),
    (Token::RParen, "^\\)"),
    (Token::LCurly, "^\\{"),
    (Token::RCurly, "^\\}"),

    (Token::Plus, "^\\+"),
    (Token::Subtract, "^-"),
    (Token::Times, "^\\*"),
    (Token::Divide, "^/"),

    (Token::Eq, "^=="),
    (Token::Neq, "^!="),
    (Token::Lt, "^<"),
    (Token::Gt, "^>"),
    (Token::Leq, "^<="),
    (Token::Geq, "^>="),


    (Token::Equals, "^="),

    (Token::Not, "^not"),
    (Token::And, "^and"),
    (Token::Or, "^or"),

    (Token::True, "^true"),
    (Token::False, "^false"),

    (Token::IntType, "^int"),
    (Token::StringType, "^str"),
    (Token::BoolType, "^bool"),

    (Token::Number_, "^\\d+"),
    (Token::String_, "^\"[^\"]+\""),
    (Token::Identifier_, "^[A-Za-z_]*"),
];

impl Token {
    pub fn remove___(self, token_value: &str) -> Self {
        use Token::*;

        match self {
            Number_ => {Number(token_value.to_string())},
            Identifier_ => {Identifier(token_value.to_string())},
            String_ => String(token_value.to_string()),
            _ => self
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Token::*;
        let str = match self {
            Println => "println",
            Let => "let",
            Fn => "fn",
            If => "if",
            Else => "else",
            For => "for",
            While => "while",

            LineEnd => ";",
            Comma => ",",
            Colon => ":",
            Arrow => "->",

            Equals => "=",

            Eq => "==",
            Neq => "!=",
            Lt => "<",
            Gt => ">",
            Leq => "<=",
            Geq => ">=",

            LParen => "(",
            RParen => ")",
            LCurly => "{",
            RCurly => "}",

            Plus => "+",
            Subtract => "-",
            Times => "*",
            Divide => "/",
            Not => "not",
            And => "and",
            Or => "or",

            True => "true",
            False => "false",

            IntType => "int",
            StringType => "string",
            BoolType => "bool",

            Number_ => "number",
            Number(num) => num.as_str(),
            String_ => "string",
            String(str) => str.as_str(),
            Identifier_ => "identifier",
            Identifier(id) => id.as_str(),

            EOF => "EOF"
        };

        f.write_str(str)
    }
}