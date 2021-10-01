use std::rc::Rc;
use crate::{items::Identifier};
use super::{ProgramContext, Type, Wat};

#[derive(Debug)]
pub struct VariableInfo {
    pub name: Identifier,
    pub ty: Type,
    pub kind: VariableKind,
    pub wasm_name: String
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum VariableKind {
    Global,
    Local,
    Argument
}

impl VariableInfo {
    pub fn new(name: Identifier, ty: Type, kind: VariableKind) -> Rc<Self> {
        let wasm_name = name.to_unique_string();
        let value = Self { name, ty, kind, wasm_name };

        Rc::new(value)
    }

    pub fn from_wasm_name(wasm_name: String, ty: Type, kind: VariableKind) -> Rc<Self> {
        Rc::new(Self {
            name: Identifier::unlocated(""),
            ty,
            kind,
            wasm_name,
        })
    }

    pub fn check_parameters(&self, context: &mut ProgramContext) {
        self.ty.check_parameters(&self.name, context);
    }

    pub fn get_to_stack(&self) -> Wat {
        match &self.kind {
            VariableKind::Global => Wat::get_global(&self.wasm_name),
            VariableKind::Local => Wat::get_local(&self.wasm_name),
            VariableKind::Argument => Wat::get_local(&self.wasm_name),
        }
    }

    pub fn set_from_stack(&self) -> Wat {
        match &self.kind {
            VariableKind::Global => Wat::set_global_from_stack(&self.wasm_name),
            VariableKind::Local => Wat::set_local_from_stack(&self.wasm_name),
            VariableKind::Argument => Wat::set_local_from_stack(&self.wasm_name),
        }
    }

    pub fn tee_from_stack(&self) -> Wat {
        match &self.kind {
            VariableKind::Global => Wat::tee_global_from_stack(&self.wasm_name),
            VariableKind::Local => Wat::tee_local_from_stack(&self.wasm_name),
            VariableKind::Argument => Wat::tee_local_from_stack(&self.wasm_name),
        }
    }

    pub fn get_wasm_name(&self) -> &str {
        self.wasm_name.as_str()
    }

    pub fn clone(self: &Rc<Self>) -> Rc<Self> {
        Rc::clone(self)
    }
}

impl Default for VariableInfo {
    fn default() -> Self {
        Rc::try_unwrap(Self::new(Identifier::default(), Type::Undefined, VariableKind::Local)).unwrap()
    }
}