use std::rc::Rc;
use super::{ProgramContext, ToVasm, Type, TypeIndex, VariableInfo, VirtualInstruction, Wat};

#[derive(Debug, Clone)]
pub struct Vasm {
    pub ty: Type,
    pub variables: Vec<VariableInfo>,
    pub instructions: Vec<VirtualInstruction>,
}

impl Vasm {
    pub fn new(ty: Type, variables: Vec<VariableInfo>, instructions: Vec<VirtualInstruction>) -> Self {
        Self { ty, variables, instructions }
    }

    pub fn void() -> Self {
        Self::new(Type::Void, vec![], vec![])
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

    pub fn merge_with_type(ty: Type, source: Vec<Self>) -> Self {
        let mut result = Self::merge(source);
        result.ty = ty;

        result
    }

    pub fn replace_type_parameters(&self, this_type: &Type, id: u64) -> Self {
        Self {
            ty: self.ty.replace_parameters(Some(this_type), &[]),
            variables: self.variables.iter().map(|var_info| var_info.replace_type_parameters(this_type, id)).collect(),
            instructions: self.instructions.iter().map(|inst| inst.replace_type_parameters(this_type, id)).collect()
        }
    }

    pub fn collect_variables(&self, list: &mut Vec<VariableInfo>) {
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
        Self::void()
    }
}