use std::fmt::{Display, Formatter};
use crate::lexer::tokens::Token;

#[derive(Clone)]
pub enum TokenType {
    Expression,
    Type,
    Operator,
    Factor,
    Identifier
}

impl Display for &TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use TokenType::*;
        let display_text = match self {
            Expression => "expression",
            Type => "type",
            Operator => "operator",
            Factor => "factor",
            Identifier => "identifier"
        };

        write!(f, "{}", display_text)
    }
}


pub enum Factor {
    Integer(String),
    String(String),
    Identifier(String),
    Not,
    LParen,
    True,
    False
}

impl Factor {
    pub fn option_from(token: &Token) -> Option<Self> {

        use Token::*;
        match token {
            Number(num) => Some(Factor::Integer(num.to_string())),
            String(num) => Some(Factor::String(num.to_string())),
            Identifier(num) => Some(Factor::Identifier(num.to_string())),
            Not => Some(Factor::Not),
            LParen => Some(Factor::LParen),
            True => Some(Factor::True),
            False => Some(Factor::False),
            _ => None
        }
    }
}