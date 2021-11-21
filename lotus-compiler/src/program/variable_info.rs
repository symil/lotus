use std::{cell::Ref, rc::Rc};
use crate::{items::Identifier, utils::Link};
use super::{ProgramContext, Type, TypeIndex, TypeInstanceHeader, Wat};

pub type VariableInfo = Link<VariableInfoContent>;

#[derive(Debug)]
pub struct VariableInfoContent {
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

impl VariableKind {
    pub fn is_local(&self) -> bool {
        match self {
            VariableKind::Local => true,
            _ => false,
        }
    }
}

impl VariableInfo {
    pub fn from(name: Identifier, ty: Type, kind: VariableKind) -> Self {
        let wasm_name = name.to_unique_string();
        let value = VariableInfoContent { name, ty, kind, wasm_name };

        Link::new(value)
    }

    pub fn from_wasm_name(wasm_name: String, ty: Type, kind: VariableKind) -> Self {
        Link::new(VariableInfoContent {
            name: Identifier::unlocated(""),
            ty,
            kind,
            wasm_name,
        })
    }
}

impl VariableInfo {
    pub fn check_parameters(&self, context: &mut ProgramContext) {
        self.with_ref(|var_info| {
            var_info.ty.check_parameters(&var_info.name, context);
        })
    }

    pub fn get_to_stack(&self) -> Wat {
        self.with_ref(|var_info| {
            match &var_info.kind {
                VariableKind::Global => Wat::get_global(&var_info.wasm_name),
                VariableKind::Local => Wat::get_local(&var_info.wasm_name),
                VariableKind::Argument => Wat::get_local(&var_info.wasm_name),
            }
        })
    }

    pub fn set_from_stack(&self) -> Wat {
        self.with_ref(|var_info| {
            match &var_info.kind {
                VariableKind::Global => Wat::set_global_from_stack(&var_info.wasm_name),
                VariableKind::Local => Wat::set_local_from_stack(&var_info.wasm_name),
                VariableKind::Argument => Wat::set_local_from_stack(&var_info.wasm_name),
            }
        })
    }

    pub fn tee_from_stack(&self) -> Wat {
        self.with_ref(|var_info| {
            match &var_info.kind {
                VariableKind::Global => Wat::tee_global_from_stack(&var_info.wasm_name),
                VariableKind::Local => Wat::tee_local_from_stack(&var_info.wasm_name),
                VariableKind::Argument => Wat::tee_local_from_stack(&var_info.wasm_name),
            }
        })
    }

    pub fn get_wasm_name(&self) -> String {
        self.borrow().wasm_name.clone()
    }

    pub fn replace_type_parameters(&self, this_type: &Type, id: u64) -> VariableInfo {
        self.with_ref(|var_info| {
            Link::new(VariableInfoContent {
                name: var_info.name.clone(),
                ty: var_info.ty.replace_parameters(Some(this_type), &[]),
                kind: var_info.kind.clone(),
                wasm_name: format!("{}_{}", var_info.wasm_name.clone(), id),
            })
        })
    }

    pub fn ty(&self) -> Ref<Type> {
        Ref::map(self.borrow(), |var_info| &var_info.ty)
    }

    pub fn kind(&self) -> Ref<VariableKind> {
        Ref::map(self.borrow(), |var_info| &var_info.kind)
    }

    pub fn name(&self) -> Ref<Identifier> {
        Ref::map(self.borrow(), |var_info| &var_info.name)
    }
}

impl Default for VariableInfo {
    fn default() -> Self {
        Link::new(VariableInfoContent {
            name: Identifier::default(),
            ty: Type::Undefined,
            kind: VariableKind::Local,
            wasm_name: String::new(),
        })
    }
}