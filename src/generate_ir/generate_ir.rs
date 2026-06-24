use crate::generate_ir::ir::{IRLine, SingleAddress};
use crate::interfaces::lexer_interface::next;
use crate::parser::parse_tree::*;

pub struct LabelMaker {
    curr_num: usize,
    symbol: String,
}

impl LabelMaker {
    pub fn next(&mut self) -> String {
        let label = format!("{}{}", self.symbol, self.curr_num);
        self.curr_num += 1;
        label
    }

    pub fn new(label: &str) -> Self {
        Self {
            curr_num: 0usize,
            symbol: label.to_string(),
        }
    }
}

impl ParseTree {
    pub fn generate_ir(&self) -> Vec<IRLine> {
        let mut label_generator = LabelMaker::new("l");
        let mut temp_geneartor = LabelMaker::new("t");

        let mut program: Vec<IRLine> = Vec::new();
        self.statements.generate_ir(&mut label_generator, &mut temp_geneartor, &mut program);

        program
    }
}

impl Statements {
    pub fn generate_ir(
        &self, label_generator: &mut LabelMaker, temp_generator: &mut LabelMaker, program: &mut Vec<IRLine>
    ) {

        for stmt in &self.statements {
            stmt.generate_ir(label_generator, temp_generator, program);
        }
    }
}

impl Statement {
    pub fn generate_ir(
        &self, label_generator: &mut LabelMaker, temp_generator: &mut LabelMaker, program: &mut Vec<IRLine>
    ) {

        use Statement::*;
        match self {
            Expression(expression) => {
                expression.generate_ir(label_generator, temp_generator, program);
            },
            Function(function) => {},
            Closure(statements) => {},
            IfStatement(if_statement) => {
                if_statement.generate_ir(label_generator, temp_generator, program);
            },
            Assignment(assignment) => {
                assignment.generate_ir(label_generator, temp_generator, program);
            },
        };
    }
}

impl Assignment {
    pub fn generate_ir(
        &self, label_generator: &mut LabelMaker, temp_generator: &mut LabelMaker, program: &mut Vec<IRLine>
    ) -> String {
        let label = self.expression.generate_ir(label_generator, temp_generator, program);

        program.push(IRLine::SingleAddress(SingleAddress::new(
            self.identifier.to_string(), label
        )));

        self.identifier.to_string()
    }
}