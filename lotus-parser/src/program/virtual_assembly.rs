use std::rc::Rc;

use super::{ProgramContext, ToVasm, Type, TypeIndex, VariableInfo, VirtualInstruction, Wat};

#[derive(Debug)]
pub struct Vasm {
    pub ty: Type,
    pub variables: Vec<Rc<VariableInfo>>,
    pub instructions: Vec<VirtualInstruction>,
}

impl Vasm {
    pub fn new(ty: Type, variables: Vec<Rc<VariableInfo>>, instructions: Vec<VirtualInstruction>) -> Self {
        Self { ty, variables, instructions }
    }

    pub fn undefined(variables: Vec<Rc<VariableInfo>>, content: Vec<VirtualInstruction>) -> Self {
        Self::new(Type::Undefined, variables, content)
    }

    pub fn empty() -> Self {
        Self::new(Type::Undefined, vec![], vec![])
    }

    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }

    pub fn extend<T : ToVasm>(&mut self, value: T) {
        let other = value.to_vasm();

        self.ty = other.ty;
        self.variables.extend(other.variables);
        self.instructions.extend(other.instructions);
    }

    pub fn merge(source: Vec<Self>) -> Self {
        let mut ty = Type::Undefined;
        let mut variables = vec![];
        let mut instructions = vec![];

        for virtual_block in source {
            ty = virtual_block.ty;
            variables.extend(virtual_block.variables);
            instructions.extend(virtual_block.instructions);
        }

        Self::new(ty, variables, instructions)
    }

    pub fn collect_variables(&self, list: &mut Vec<Rc<VariableInfo>>) {
        list.extend(self.variables.clone());

        for instruction in &self.instructions {
            instruction.collect_variables(list);
        }
    }

    pub fn resolve(&self, type_index: &TypeIndex, context: &mut ProgramContext) -> Vec<Wat> {
        let mut content = vec![];

        for inst in &self.instructions {
            content.extend(inst.resolve(type_index, context));
        }

        content
    }

    pub fn resolve_without_context(&self) -> Vec<Wat> {
        let mut content = vec![];

        for inst in &self.instructions {
            content.extend(inst.resolve_without_context());
        }

        content
    }
}

impl Default for Vasm {
    fn default() -> Self {
        Self::empty()
    }
}