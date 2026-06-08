use crate::lexer::tokens::Token;
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

impl From<Token> for Type {
    fn from(token: Token) -> Self {
        use Token::*;
        match token {
            StringType => Type::String,
            IntType => Type::Integer,
            BoolType => Type::Boolean,
            _ => unreachable!()
        }
    }
}

/* ---------- EXPRESSIONS ---------- */

#[derive(Debug)]
pub enum ExpressionEnum {
    Expression(Expression),
    Statements(Statements),
    Identifier(String),
    Integer(i64),
    String(String),
}

#[derive(Debug)]
pub struct Expression {
    pub left: Box<ExpressionEnum>,
    pub operator: Operator,
    pub right: Box<ExpressionEnum>
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Subtract,
    Multiply,
    Divide,
}