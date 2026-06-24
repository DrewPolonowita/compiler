use crate::generate_ir::handle_call_stack::VariableStack;
pub(crate) use crate::generate_ir::ir::{IRLine, Label, LabelMaker, SingleAddress, Temp};
use crate::parser::parse_tree::*;

impl ParseTree {
    pub fn generate_ir(&self) -> Vec<IRLine> {
        let mut label_generator: LabelMaker<Label> = LabelMaker::new();
        let mut temp_geneartor: LabelMaker<Temp> = LabelMaker::new();
        let mut stack = VariableStack::new();

        let mut program: Vec<IRLine> = Vec::new();
        self.statements.generate_ir(&mut label_generator, &mut temp_geneartor, &mut program, &mut stack);

        program
    }
}

impl Statements {
    pub fn generate_ir(
        &self, label_generator: &mut LabelMaker<Label>, temp_generator: &mut LabelMaker<Temp>,
        program: &mut Vec<IRLine>, stack: &mut VariableStack
    ) {

        stack.stack();

        for stmt in &self.statements {
            stmt.generate_ir(label_generator, temp_generator, program, stack);
        }

        stack.pop();
    }
}

impl Statement {
    pub fn generate_ir(
        &self, label_generator: &mut LabelMaker<Label>, temp_generator: &mut LabelMaker<Temp>,
        program: &mut Vec<IRLine>, stack: &mut VariableStack
    ) {

        use Statement::*;
        match self {
            Expression(expression) => {
                expression.generate_ir(label_generator, temp_generator, program, stack);
            },
            Function(_function) => {},
            Closure(_statements) => {},
            IfStatement(if_statement) => {
                if_statement.generate_ir(label_generator, temp_generator, program, stack);
            },
            Assignment(assignment) => {
                assignment.generate_ir(label_generator, temp_generator, program, stack);
            },
            Reassignment(reassignment) => {
                reassignment.generate_ir(label_generator, temp_generator, program, stack);
            },
            WhileLoop(while_loop) => {
                while_loop.generate_ir(label_generator, temp_generator, program, stack);
            }
        };
    }
}

impl Assignment {
    pub fn generate_ir(
        &self, label_generator: &mut LabelMaker<Label>, temp_generator: &mut LabelMaker<Temp>,
        program: &mut Vec<IRLine>, stack: &mut VariableStack
    ) -> Temp {
        let label = self.expression.generate_ir(label_generator, temp_generator, program, stack);


        let var = stack.create(&self.identifier, temp_generator);
        program.push(IRLine::SingleAddress(SingleAddress::new(
            var.clone(), label
        )));

        var
    }
}

impl Reassignment {
    pub fn generate_ir(
        &self, label_generator: &mut LabelMaker<Label>, temp_generator: &mut LabelMaker<Temp>,
        program: &mut Vec<IRLine>, stack: &mut VariableStack
    ) -> Temp {
        let label = self.expression.generate_ir(label_generator, temp_generator, program, stack);

        let var = stack.get(&self.identifier);
        program.push(IRLine::SingleAddress(SingleAddress::new(
            var.clone(), label
        )));

        var
    }
}