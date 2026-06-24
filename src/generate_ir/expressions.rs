use crate::generate_ir::generate_ir::LabelMaker;
use crate::generate_ir::ir::{CmpSingleAddress, CmpThreeAddress, IRLine, Goto, Label, SingleAddress, ThreeAddress, Comparison};
use crate::parser::parse_tree::{Expression, ExpressionEnum, Operator};

impl ExpressionEnum {
    pub fn generate_ir(
        &self, label_generator: &mut LabelMaker, temp_generator: &mut LabelMaker, program: &mut Vec<IRLine>
    ) -> String {

        use crate::parser::parse_tree::ExpressionEnum::*;
        match self {
            Expression(expression) => {
                expression.generate_ir(label_generator, temp_generator, program)
            },
            UnaryExpression(unary_expression) => todo!(),
            Statements(statements) => todo!(),
            Identifier(identifier) => identifier.to_string(),
            Integer(int) => int.to_string(),
            String(string) => string.to_string(),
            Boolean(bool) => {
                if bool == &true {
                    "1".to_string()
                } else {
                    "0".to_string()
                }
            },
        }
    }

    pub fn generate_ir_continuing_bool(
        &self, label_generator: &mut LabelMaker, temp_generator: &mut LabelMaker, program: &mut Vec<IRLine>,
        start: &str, parent_op: &Operator
    ) -> String {
        use crate::parser::parse_tree::ExpressionEnum::*;
        match self {
            Expression(expression) => {
                expression.generate_ir_continuing_bool(label_generator, temp_generator, program, start, parent_op)
            },
            UnaryExpression(unary_expression) => todo!(),
            Statements(statements) => todo!(),
            Identifier(identifier) => identifier.to_string(),
            Integer(int) => int.to_string(),
            String(string) => string.to_string(),
            Boolean(bool) => {
                if bool == &true {
                    "1".to_string()
                } else {
                    "0".to_string()
                }
            },
        }
    }
}

impl Expression {
    pub fn generate_ir(
        &self, label_generator: &mut LabelMaker, temp_generator: &mut LabelMaker, program: &mut Vec<IRLine>
    ) -> String {
        match &self.operator {
            Operator::And | Operator::Or => {
                let l0 = label_generator.next();
                let l1 = label_generator.next();

                self.generate_ir_continuing_bool(
                    label_generator, temp_generator, program, &l0, &self.operator
                );

                let t0 = temp_generator.next();

                program.push(IRLine::SingleAddress(SingleAddress::new(
                    t0.to_string(), if matches!(&self.operator, &Operator::And) {'1'.to_string()} else {'0'.to_string()}
                )));
                program.push(IRLine::Goto(Goto::new(l1.to_string())));
                program.push(IRLine::Label(Label::new(l0.to_string())));
                program.push(IRLine::SingleAddress(SingleAddress::new(
                    t0.to_string(), if matches!(&self.operator, &Operator::And) {'0'.to_string()} else {'1'.to_string()}
                )));
                program.push(IRLine::Label(Label::new(l1.to_string())));

                t0
            },
            Operator::Eq | Operator::Neq |
            Operator::Lt | Operator::Gt |
            Operator::Leq | Operator::Geq => {
                let left = self.left.generate_ir(label_generator, temp_generator, program);
                let right = self.right.generate_ir(label_generator, temp_generator, program);

                let l0 = label_generator.next();
                let l1 = label_generator.next();
                let t0 = temp_generator.next();

                add_cmp_instruction(&self.operator, program, &left, &right, &l0, &l1, &t0);

                t0
            },
            _ => {
                let left = self.left.generate_ir(label_generator, temp_generator, program);
                let right = self.right.generate_ir(label_generator, temp_generator, program);

                let temp = temp_generator.next();

                program.push(IRLine::ThreeAddress(ThreeAddress::new(
                    temp.to_string(), self.operator.clone(), left.to_string(), right.to_string()
                )));

                temp
            }
        }
    }

    pub fn generate_ir_continuing_bool(
        &self, label_generator: &mut LabelMaker, temp_generator: &mut LabelMaker, program: &mut Vec<IRLine>,
        l0: &str, parent_op: &Operator
    ) -> String {
        match &self.operator {
            Operator::And => {
                match parent_op {
                    Operator::And => {
                        evaluate_boolean_expression(label_generator, temp_generator, self, program, l0);
                        "".to_string()
                    },
                    _ => self.generate_ir(label_generator, temp_generator, program)
                }
            },
            Operator::Or => {
                match parent_op {
                    Operator::Or => {
                        evaluate_boolean_expression(label_generator, temp_generator, self, program, l0);
                        "".to_string()
                    },
                    _ => self.generate_ir(label_generator, temp_generator, program)
                }
            },
            _ => {
                self.generate_ir(label_generator, temp_generator, program)
            }
        }
    }

}


fn add_cmp_instruction(current_operator: &Operator, program: &mut Vec<IRLine>, left: &str, right: &str, l0: &str, l1: &str, t0: &str) {
    program.push(IRLine::CmpThreeAddress(CmpThreeAddress::new(
        Comparison::from(current_operator), left.to_string(), right.to_string(), l0.to_string()
    )));
    program.push(IRLine::SingleAddress(SingleAddress::new(t0.to_string(), '0'.to_string())));
    program.push(IRLine::Goto(Goto::new(l1.to_string())));
    program.push(IRLine::Label(Label::new(l0.to_string())));
    program.push(IRLine::SingleAddress(SingleAddress::new(t0.to_string(), '1'.to_string())));
    program.push(IRLine::Label(Label::new(l1.to_string())));
}

fn evaluate_boolean_expression(
    label_generator: &mut LabelMaker, temp_generator: &mut LabelMaker, expression: &Expression,
    program: &mut Vec<IRLine>, l0: &str
) {
    let left = expression.left.generate_ir_continuing_bool(label_generator, temp_generator, program, &l0, &expression.operator);

    add_logical_jump_to_program(&expression.left, &expression.operator, program, &left, l0);

    let right = expression.right.generate_ir_continuing_bool(label_generator, temp_generator, program, &l0, &expression.operator);

    add_logical_jump_to_program(&expression.right, &expression.operator, program, &right, l0);
}

fn add_logical_jump_to_program(
    tree: &ExpressionEnum, operator: &Operator, program: &mut Vec<IRLine>, label: &str, l0: &str
)  {
    if not_a_expression_tree_with_same_logical_operator(tree, operator) {
        program.push(IRLine::CmpSingleAddress(CmpSingleAddress::new(
            if matches!(operator, &Operator::And) {Comparison::False} else {Comparison::True},
            label.to_string(), l0.to_string()
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