use crate::interfaces::token_type::Factor;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::Token::{BoolType, IntType, LParen, Not, StringType};

#[derive(Debug)]
pub struct ParseTree {
    pub statements: Statements,
}

#[derive(Debug)]
pub struct Statements {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Expression(ExpressionEnum),
    Function(Function),
    Closure(Statements),
    IfStatement(IfStatement),
    Assignment(Assignment),
}

/* ---------- Assignment ---------- */

#[derive(Debug)]
pub struct Assignment {
    pub identifier: String,
    pub return_type: Type,
    pub expression: ExpressionEnum,
}

/* ---------- Conditionals ---------- */

#[derive(Debug)]
pub struct IfStatement {
    pub conditionals: Vec<Conditional>,
    pub otherwise: Option<Statements>,
}

#[derive(Debug)]
pub struct Conditional {
    pub condition: ExpressionEnum,
    pub body: Statements
}

/* ---------- Functions ---------- */

#[derive(Debug)]
pub struct Function {
    pub identifier: String,
    pub return_type: Type,
    pub arguments: Vec<TypedArgument>,
    pub body: Statements
}

#[derive(Debug)]
pub struct TypedArgument {
    pub identifier: String,
    pub arg_type: Type
}

#[derive(Debug)]
pub enum Type {
    Void,
    String,
    Integer,
    Boolean
}

impl Type {
    pub fn option_from(token: &Token) -> Option<Self> {

        use Token::*;
        match token {
            StringType => Some(Type::String),
            IntType => Some(Type::Integer),
            BoolType => Some(Type::Boolean),
            _ => None
        }
    }
}

/* ---------- EXPRESSIONS ---------- */

#[derive(Debug)]
pub enum ExpressionEnum {
    Expression(Expression),
    UnaryExpression(UnaryExpression),
    Statements(Statements),
    Identifier(String),
    Integer(String),
    String(String),
    Boolean(bool),
}

#[derive(Debug)]
pub struct Expression {
    pub left: Box<ExpressionEnum>,
    pub operator: Operator,
    pub right: Box<ExpressionEnum>
}

#[derive(Debug)]
pub enum UnaryExpression {
    Negate(Box<ExpressionEnum>),
    Not(Box<ExpressionEnum>)
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Subtract,
    Multiply,
    Divide,
    And,
    Or
}

impl Operator {
    pub fn option_from(token: &Token) -> Option<Self> {

        use Token::*;
        match token {
            Plus => Some(Operator::Plus),
            Subtract => Some(Operator::Subtract),
            Times => Some(Operator::Multiply),
            Divide => Some(Operator::Divide),
            And => Some(Operator::And),
            Or => Some(Operator::Or),
            _ => None
        }
    }
}