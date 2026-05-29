use std::fmt::{Display, Formatter};
use crate::lexer::token_type::TokenType::{Expression, LParen, RParen};
use crate::lexer::tokens::Token;
use crate::lexer::tokens::Token::{Comma, Equals, LineEnd, RCurly};

#[derive(Clone, PartialOrd, PartialEq)]
#[allow(dead_code)]
#[derive(Debug)]
pub enum TokenType {
    // Keywords
    Println,
    Let,

    Separator,
    Colon,

    // Character tokens
    LParen,
    RParen,
    LineEnd,
    Equals,

    Operator,

    // Types
    Type,
    Identifier,
    Expression,
    Keyword,

    EOF
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TokenType::Println => "println",
            TokenType::Let => "let",
            TokenType::LineEnd => ";",
            TokenType::Equals => "=",
            TokenType::LParen => "(",
            TokenType::RParen => ")",

            TokenType::Separator => ",",
            TokenType::Colon => ":",


            TokenType::Operator => "operator",
            TokenType::Type => "type",
            TokenType::Expression => "expression",
            TokenType::Identifier => "identifier",
            TokenType::Keyword => "keyword",
            TokenType::EOF => "eof",
        };

        f.write_str(str)
    }
}
impl From<&Token> for TokenType {
    fn from(token: &Token) -> Self {
        use Token::*;
        match token {
            LineEnd => TokenType::LineEnd,
            Equals => TokenType::Equals,
            Comma => TokenType::Separator,
            Colon => TokenType::Colon,

            Println => TokenType::Expression,
            Let => TokenType::Expression,
            Fn => TokenType::Expression,

            Plus => TokenType::Operator,
            Subtract => TokenType::Operator,
            Times => TokenType::Operator,
            Divide => TokenType::Operator,

            LParen => TokenType::Expression,
            RParen => TokenType::Expression,
            RCurly => TokenType::Expression,
            LCurly => TokenType::Expression,

            Number_ => TokenType::Expression,
            Number(_) => TokenType::Expression,
            String_ => TokenType::Expression,
            String(_) => TokenType::Expression,

            Identifier_ => TokenType::Identifier,
            Identifier(_) => TokenType::Identifier,

            IntType => TokenType::Type,
            StringType => TokenType::Type,
            BoolType => TokenType::Type,

            EOF => TokenType::EOF,
        }
    }
}

impl TokenType {
    pub fn is_member(&self, other: &TokenType) -> bool {
        use TokenType::*;
        match self {
            Identifier => {
                match other {
                    Expression => true,
                    _ => self == other
                }
            }
            _ => self == other,
        }
    }
}

impl Token {
    pub fn is_member(&self, other: &TokenType) -> bool {
        let token_type = TokenType::from(self);
        token_type.is_member(other)
    }
}