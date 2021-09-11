use std::rc::Rc;
use super::{Type, VariableInfo, VirtualInstruction};

#[derive(Debug)]
pub struct Vasm {
    pub ty: Type,
    pub variables: Vec<Rc<VariableInfo>>,
    pub instructions: Vec<VirtualInstruction>,
}

impl Vasm {
    pub fn new(ty: Type, variables: Vec<Rc<VariableInfo>>, content: Vec<VirtualInstruction>) -> Self {
        Self { ty, variables, instructions: content }
    }

    pub fn empty() -> Self {
        Self::new(Type::Void, vec![], vec![])
    }

    pub fn merge(source: Vec<Self>) -> Self {
        let mut ty = Type::Void;
        let mut variables = vec![];
        let mut instructions = vec![];

        for virtual_block in source {
            ty = virtual_block.ty;
            variables.extend(virtual_block.variables);
            instructions.extend(virtual_block.instructions);
        }

        Self::new(ty, variables, instructions)
    }

    pub fn collect_variables(&self, mut list: Vec<Rc<VariableInfo>>) -> Vec<Rc<VariableInfo>> {
        list.extend(self.variables.clone());
        list
    }
}