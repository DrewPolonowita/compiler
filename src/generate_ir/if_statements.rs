use crate::generate_ir::generate_ir::LabelMaker;
use crate::generate_ir::ir::{CmpSingleAddress, IRLine, Goto, Label, Comparison};
use crate::parser::parse_tree::IfStatement;

impl IfStatement {
    pub fn generate_ir(
        &self, label_generator: &mut LabelMaker, temp_generator: &mut LabelMaker, program: &mut Vec<IRLine>
    ) -> String {

        let last_label = label_generator.next();

        let mut iter = self.conditionals.iter().peekable();
        let mut next_label = (&last_label).to_string();

        while let Some(conditional) = iter.next() {

            if matches!(iter.peek(), Some(_)) || matches!(self.otherwise, Some(_)) {
                next_label = label_generator.next();
            } else {
                next_label = (&last_label).to_string();
            }

            let condition_label = conditional.condition.generate_ir(label_generator, temp_generator, program);

            program.push(IRLine::CmpSingleAddress(CmpSingleAddress::new(
                Comparison::False, condition_label, next_label.to_string()
            )));

            conditional.body.generate_ir(label_generator, temp_generator, program);

            if matches!(iter.peek(), Some(_)) || matches!(self.otherwise, Some(_)) {
                program.push(IRLine::Goto(Goto::new(last_label.to_string())))
            }
        }

        if let Some(otherwise) = &self.otherwise {
            program.push(IRLine::Label(Label::new(next_label.to_string())));
            otherwise.generate_ir(label_generator, temp_generator, program);
        }

        program.push(IRLine::Label(Label::new(last_label.to_string())));

        last_label
    }
}