use std::rc::Rc;
use parsable::DataLocation;
use super::{ProgramContext, ToVasm, Type, TypeIndex, VariableInfo, VirtualInstruction, Wat};

pub type Vasm = VirtualAssembly;

#[derive(Debug, Clone)]
pub struct VirtualAssembly {
    pub ty: Type,
    pub variables: Vec<VariableInfo>,
    pub instructions: Vec<VirtualInstruction>,
}

impl VirtualAssembly {
    pub fn new<T : ToVasm>(ty: Type, variables: Vec<VariableInfo>, instructions: T) -> Self {
        Self {
            ty,
            variables,
            instructions: instructions.to_vasm().instructions,
        }
    }

    pub fn void() -> Self {
        Self {
            ty: Type::Void,
            variables: vec![],
            instructions: vec![],
        }
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



    pub fn collect_variables(&self, list: &mut Vec<VariableInfo>) {
        list.extend(self.variables.clone());

        for instruction in &self.instructions {
            instruction.collect_variables(list);
        }
    }

    pub fn replace_placeholder(&mut self, location: &DataLocation, replacement: &Rc<Vasm>) {
        for instruction in &mut self.instructions {
            instruction.replace_placeholder(location, replacement);
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