use crate::generate_ir::generate_ir::LabelMaker;
use crate::generate_ir::handle_call_stack::VariableStack;
use crate::generate_ir::ir::{CmpSingleAddress, CmpThreeAddress, IRLine, Goto, Label, SingleAddress, ThreeAddress, Comparison, Temp, Value, LabelBlock};
use crate::parser::parse_tree::{Expression, ExpressionEnum, Operator};

impl ExpressionEnum {
    pub fn generate_ir(
        &self, label_generator: &mut LabelMaker<Label>, temp_generator: &mut LabelMaker<Temp>,
        program: &mut Vec<IRLine>, stack: &mut VariableStack
    ) -> Value {

        use crate::parser::parse_tree::ExpressionEnum::*;
        match self {
            Expression(expression) => {
                expression.generate_ir(label_generator, temp_generator, program, stack)
            },
            UnaryExpression(unary_expression) => todo!(),
            Statements(statements) => todo!(),
            Identifier(identifier) => Value::Temp(stack.get(&identifier, temp_generator)),
            Integer(int) => Value::Prim(int.to_string()),
            String(string) => unreachable!(),
            Boolean(bool) => {
                if bool == &true {
                    Value::Prim("1".to_string())
                } else {
                    Value::Prim("0".to_string())
                }
            },
        }
    }

    pub fn generate_ir_continuing_bool(
        &self, label_generator: &mut LabelMaker<Label>, temp_generator: &mut LabelMaker<Temp>, program: &mut Vec<IRLine>,
        start: &Label, parent_op: &Operator, stack: &mut VariableStack
    ) -> Value {
        use crate::parser::parse_tree::ExpressionEnum::*;
        match self {
            Expression(expression) => {
                expression.generate_ir_continuing_bool(label_generator, temp_generator, program, start, parent_op, stack)
            },
            _ => self.generate_ir(label_generator, temp_generator, program, stack)
        }
    }
}

impl Expression {
    pub fn generate_ir(
        &self, label_generator: &mut LabelMaker<Label>, temp_generator: &mut LabelMaker<Temp>, program: &mut Vec<IRLine>,
        stack: &mut VariableStack
    ) -> Value {
        match &self.operator {
            Operator::And | Operator::Or => {
                let l0 = label_generator.next();
                let l1 = label_generator.next();

                self.generate_ir_continuing_bool(
                    label_generator, temp_generator, program, &l0, &self.operator, stack
                );

                let t0 = temp_generator.next();


                program.push(IRLine::SingleAddress(SingleAddress::new(
                    t0.clone(), if matches!(&self.operator, &Operator::And) {
                        Value::Prim('1'.to_string())} else {Value::Prim('0'.to_string())
                    }
                )));
                program.push(IRLine::Goto(Goto::new(l1.clone())));
                program.push(IRLine::Label(LabelBlock::new(l0.clone())));
                program.push(IRLine::SingleAddress(SingleAddress::new(
                    t0.clone(), if matches!(&self.operator, &Operator::And) {
                        Value::Prim('0'.to_string())} else {Value::Prim('1'.to_string())
                    }
                )));
                program.push(IRLine::Label(LabelBlock::new(l1.clone())));

                Value::Temp(t0)
            },
            Operator::Eq | Operator::Neq |
            Operator::Lt | Operator::Gt |
            Operator::Leq | Operator::Geq => {
                let left = self.left.generate_ir(label_generator, temp_generator, program, stack);
                let right = self.right.generate_ir(label_generator, temp_generator, program, stack);

                let l0 = label_generator.next();
                let l1 = label_generator.next();
                let t0 = temp_generator.next();

                add_cmp_instruction(&self.operator, program, left, right, &l0, &l1, &t0);

                Value::Temp(t0)
            },
            _ => {
                let left = self.left.generate_ir(label_generator, temp_generator, program, stack);
                let right = self.right.generate_ir(label_generator, temp_generator, program, stack);

                let temp = temp_generator.next();

                program.push(IRLine::ThreeAddress(ThreeAddress::new(
                    temp.clone(), self.operator.clone(), left, right
                )));

                Value::Temp(temp)
            }
        }
    }

    pub fn generate_ir_continuing_bool(
        &self, label_generator: &mut LabelMaker<Label>, temp_generator: &mut LabelMaker<Temp>, program: &mut Vec<IRLine>,
        l0: &Label, parent_op: &Operator, stack: &mut VariableStack
    ) -> Value {
        match &self.operator {
            Operator::And => {
                match parent_op {
                    Operator::And => {
                        evaluate_boolean_expression(label_generator, temp_generator, self, program, l0, stack);
                        Value::Prim("".to_string())
                    },
                    _ => self.generate_ir(label_generator, temp_generator, program, stack)
                }
            },
            Operator::Or => {
                match parent_op {
                    Operator::Or => {
                        evaluate_boolean_expression(label_generator, temp_generator, self, program, l0, stack);
                        Value::Prim("".to_string())
                    },
                    _ => self.generate_ir(label_generator, temp_generator, program, stack)
                }
            },
            _ => {
                self.generate_ir(label_generator, temp_generator, program, stack)
            }
        }
    }

}


fn add_cmp_instruction(
    current_operator: &Operator, program: &mut Vec<IRLine>, left: Value, right: Value, l0: &Label, l1: &Label, t0: &Temp
) {
    program.push(IRLine::CmpThreeAddress(CmpThreeAddress::new(
        Comparison::from(current_operator), left, right, l0.clone()
    )));
    program.push(IRLine::SingleAddress(SingleAddress::new(t0.clone(), Value::Prim('0'.to_string()))));
    program.push(IRLine::Goto(Goto::new(l1.clone())));
    program.push(IRLine::Label(LabelBlock::new(l0.clone())));
    program.push(IRLine::SingleAddress(SingleAddress::new(t0.clone(), Value::Prim('1'.to_string()))));
    program.push(IRLine::Label(LabelBlock::new(l1.clone())));
}

fn evaluate_boolean_expression(
    label_generator: &mut LabelMaker<Label>, temp_generator: &mut LabelMaker<Temp>, expression: &Expression,
    program: &mut Vec<IRLine>, l0: &Label, stack: &mut VariableStack
) {
    let left = expression.left.generate_ir_continuing_bool(label_generator, temp_generator, program, &l0, &expression.operator, stack);

    add_logical_jump_to_program(&expression.left, &expression.operator, program, &left, l0);

    let right = expression.right.generate_ir_continuing_bool(label_generator, temp_generator, program, &l0, &expression.operator, stack);

    add_logical_jump_to_program(&expression.right, &expression.operator, program, &right, l0);
}

fn add_logical_jump_to_program(
    tree: &ExpressionEnum, operator: &Operator, program: &mut Vec<IRLine>, label: &Value, l0: &Label
)  {
    if not_a_expression_tree_with_same_logical_operator(tree, operator) {
        program.push(IRLine::CmpSingleAddress(CmpSingleAddress::new(
            if matches!(operator, &Operator::And) {Comparison::False} else {Comparison::True},
            label.clone(), l0.clone()
        )))
    }
}

fn not_a_expression_tree_with_same_logical_operator(tree: &ExpressionEnum, current_op: &Operator) -> bool {
    match tree {
        ExpressionEnum::Expression(expression_tree) => {
            match current_op {
                Operator::And => !matches!(expression_tree.operator, Operator::Or),
                Operator::Or => !matches!(expression_tree.operator, Operator::Or),
                _ => false
            }
        },
        _ => true
    }
}