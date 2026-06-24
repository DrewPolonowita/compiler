use std::collections::HashMap;
use crate::generate_ir::ir::{LabelMaker, Temp};

pub struct VariableStack {
    pub(crate) stack: Vec<HashMap<String, Temp>>
}

impl VariableStack {
    pub fn new() -> Self {
        Self {
            stack: Vec::new()
        }
    }

    pub fn get(&mut self, var: &str, temp_maker: &mut LabelMaker<Temp>) -> Temp {
        for layer in self.stack.iter().rev() {
            if layer.contains_key(var) {
                return layer.get(var).unwrap().clone();
            }
        };

        for layer in self.stack.iter() {
            for (item, _) in layer {
                println!("{}", item);
            }
        }

        println!("{}", var);

        unreachable!("THIS CODE IS UNREACHABLE DUE TO SEMANTIC ANALYSIS")
    }

    pub fn create(&mut self, var: &str, temp_maker: &mut LabelMaker<Temp>) -> Temp {
        let temp = temp_maker.next();
        let mut layer = self.stack.last_mut().unwrap();
        layer.insert(var.to_string(), temp.clone());

        temp
    }

    pub fn stack(&mut self) {
        self.stack.push(HashMap::new());
    }

    pub fn pop(&mut self) {self.stack.pop().unwrap();}
}