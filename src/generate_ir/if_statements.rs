use crate::generate_ir::generate_ir::{LabelMaker, Temp};
use crate::generate_ir::handle_call_stack::VariableStack;
use crate::generate_ir::ir::{CmpSingleAddress, IRLine, Goto, Label, Comparison, LabelBlock};
use crate::parser::parse_tree::IfStatement;

impl IfStatement {
    pub fn generate_ir(
        &self, label_generator: &mut LabelMaker<Label>, temp_generator: &mut LabelMaker<Temp>, program: &mut Vec<IRLine>,
        stack: &mut VariableStack
    ) {

        let last_label = label_generator.next();

        let mut iter = self.conditionals.iter().peekable();
        let mut next_label = (&last_label).clone();

        while let Some(conditional) = iter.next() {

            if matches!(iter.peek(), Some(_)) || matches!(self.otherwise, Some(_)) {
                next_label = label_generator.next();
            } else {
                next_label = (&last_label).clone();
            }

            let condition_label = conditional.condition.generate_ir(label_generator, temp_generator, program, stack);

            program.push(IRLine::CmpSingleAddress(CmpSingleAddress::new(
                Comparison::False, condition_label, next_label.clone()
            )));

            conditional.body.generate_ir(label_generator, temp_generator, program, stack);

            if matches!(iter.peek(), Some(_)) || matches!(self.otherwise, Some(_)) {
                program.push(IRLine::Goto(Goto::new(last_label.clone())))
            }
        }

        if let Some(otherwise) = &self.otherwise {
            program.push(IRLine::Label(LabelBlock::new(next_label.clone())));
            otherwise.generate_ir(label_generator, temp_generator, program, stack);
        }

        program.push(IRLine::Label(LabelBlock::new(last_label.clone())));

        
    }
}