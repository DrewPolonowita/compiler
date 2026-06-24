use std::cmp::PartialEq;
use crate::error::error::CompilerError;
use crate::parser::parse_tree::{Assignment, ExpressionEnum, Function, IfStatement, ParseTree, Reassignment, Statement, Statements, Type, WhileLoop};
use crate::semantics::error::ErrorType::UnmatchedTypes;
use crate::semantics::error::SemanticError;

impl ParseTree {
    pub fn check_semantics(&self) -> Result<(), CompilerError> {

        self.statements.check_semantics()?;

        Ok(())
    }
}

impl Statements {
    pub fn check_semantics(&self) -> Result<(), CompilerError> {
        for statement in &self.statements {
            statement.check_semantics()?;
        }

        Ok(())
    }
}

impl Statement {
    pub fn check_semantics(&self) -> Result<Type, CompilerError> {
        use Statement::*;
        match self {
            Expression(expression_enum) => expression_enum.check_semantics(),
            Function(_function) => todo!(),
            Closure(_statements) => todo!(),
            IfStatement(_if_statement) => todo!(),
            Assignment(assignment) => assignment.check_semantics(),
            WhileLoop(_while_loop) => todo!(),
            Reassignment(_reassignment) => todo!(),
        }
    }
}

impl Assignment {
    pub fn check_semantics(&self) -> Result<Type, CompilerError> {
        let expected_type = self.return_type.clone();
        let expression_type = self.expression.check_semantics()?;

        if &expected_type != &expression_type {
            Err(CompilerError::from(SemanticError::new(
                UnmatchedTypes(self.identifier.to_string(), expected_type, expression_type)
            )))
        } else {
            Ok(Type::Void)
        }
    }
}