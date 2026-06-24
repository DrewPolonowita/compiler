use crate::error::error::CompilerError;
use crate::parser::parse_tree::{Expression, ExpressionEnum, Operator, Statements, Type};
use crate::parser::parse_tree::Operator::{Eq, Multiply, Plus, Subtract};
use crate::parser::parse_tree::Type::{Boolean, Integer};
use crate::semantics::error::ErrorType::UnsupportedOperation;
use crate::semantics::error::SemanticError;

impl ExpressionEnum {
    pub fn check_semantics(&self) -> Result<Type, CompilerError> {
        use ExpressionEnum::*;
        match self {
            Expression(expression) => expression.check_semantics(),
            UnaryExpression(_unary_expression) => todo!(),
            Statements(_statements) => todo!(),
            Identifier(_string) => todo!(),
            Integer(_) => Ok(Type::Integer),
            String(_string) => todo!(),
            Boolean(_) => Ok(Type::Boolean),
        }
    }
}


impl Expression {
    pub fn check_semantics(&self) -> Result<Type, CompilerError> {
        let left_type = self.left.check_semantics()?;
        let right_type = self.right.check_semantics()?;
        let operator = self.operator.clone();

        type_of(&left_type, &right_type, &operator)

    }
}











fn type_of(left: &Type, right: &Type, op: &Operator) -> Result<Type, CompilerError> {
    use Type::*;
    use Operator::*;
    match left {
        Integer => {
            match right {
                Integer => {
                    match op {
                        Plus => Ok(Integer),
                        Subtract => Ok(Integer),
                        Multiply => Ok(Integer),

                        Eq => Ok(Boolean),
                        Neq => Ok(Boolean),
                        Gt => Ok(Boolean),
                        Lt => Ok(Boolean),
                        Geq => Ok(Boolean),
                        Leq => Ok(Boolean),

                        _ => {
                            Err(CompilerError::from(
                                SemanticError::new(UnsupportedOperation(Integer, Integer, op.clone())))
                            )
                        }
                    }
                },
                Boolean => {
                    match op {
                        _ => {
                            Err(CompilerError::from(
                                SemanticError::new(UnsupportedOperation(Integer, Boolean, op.clone())))
                            )
                        }
                    }
                },
                _ => todo!()
            }
        },
        Bool => {
            match right {
                Integer => {
                    match op {
                        _ => {
                            Err(CompilerError::from(
                                SemanticError::new(UnsupportedOperation(Integer, Integer, op.clone())))
                            )
                        }
                    }
                },
                Boolean => {
                    match op {
                        And => Ok(Boolean),
                        Or => Ok(Boolean),

                        Eq => Ok(Boolean),
                        Neq => Ok(Boolean),

                        _ => {
                            Err(CompilerError::from(
                                SemanticError::new(UnsupportedOperation(Integer, Boolean, op.clone())))
                            )
                        }
                    }
                },
                _ => todo!()
            }
        },
        _ => todo!()
    }
}