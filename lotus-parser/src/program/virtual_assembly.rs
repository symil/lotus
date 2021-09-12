use std::rc::Rc;

use super::{ToVasm, Type, VariableInfo, VirtualInstruction, Wat};

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

    pub fn void(variables: Vec<Rc<VariableInfo>>, content: Vec<VirtualInstruction>) -> Self {
        Self::new(Type::Void, variables, content)
    }

    pub fn empty() -> Self {
        Self::new(Type::Void, vec![], vec![])
    }

    pub fn extend<T : ToVasm>(&mut self, value: T) {
        let other = value.to_vasm();

        self.ty = other.ty;
        self.variables.extend(other.variables);
        self.instructions.extend(other.instructions);
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

    pub fn collect_all_variables(&self) -> Vec<Rc<VariableInfo>> {
        let mut list = vec![];
        self.collect_variables(&mut list);
        list
    }

    pub fn collect_variables(&self, list: &mut Vec<Rc<VariableInfo>>) {
        list.extend(self.variables.clone());
    }

    pub fn resolve(&self) -> Vec<Wat> {
        todo!()
    }
}

impl Default for Vasm {
    fn default() -> Self {
        Self::empty()
    }
}