use crate::generate_ir::handle_call_stack::VariableStack;
use crate::generate_ir::ir::{LabelMaker, Temp, Label};
use crate::generate_ir::ir::{CmpSingleAddress, Comparison, Goto, IRLine, LabelBlock};
use crate::parser::parse_tree::WhileLoop;

impl WhileLoop {
    pub fn generate_ir(
        &self, label_generator: &mut LabelMaker<Label>, temp_generator: &mut LabelMaker<Temp>, program: &mut Vec<IRLine>,
        stack: &mut VariableStack
    ) {
        let l0 = label_generator.next();
        let l1 = label_generator.next();

        program.push(IRLine::Label(LabelBlock::new(l0.clone())));
        let condition = self.condition.generate_ir(label_generator, temp_generator, program, stack);

        program.push(IRLine::CmpSingleAddress(CmpSingleAddress::new(
            Comparison::False, condition, l1.clone()
        )));
        program.push(IRLine::Goto(Goto::new(l0)));
        program.push(IRLine::Label(LabelBlock::new(l1)));
    }
}