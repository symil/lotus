use std::{cell::Ref, rc::Rc};
use crate::{items::Identifier, utils::Link};
use super::{ProgramContext, Type, TypeIndex, TypeInstanceHeader, Wat};

pub type VariableInfo = Link<VariableInfoContent>;

#[derive(Debug)]
pub struct VariableInfoContent {
    pub name: Identifier,
    pub ty: Type,
    pub kind: VariableKind,
    pub wasm_name: String,
    pub declaration_level: Option<u32>,
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
    pub fn create(name: Identifier, ty: Type, kind: VariableKind, function_level: u32) -> Self {
        let wasm_name = name.to_unique_string();
        let declaration_level = Some(function_level);
        let content = VariableInfoContent { name, ty, kind, declaration_level, wasm_name };

        Link::new(content)
    }

    pub fn tmp(name: &str, ty: Type) -> Self {
        let name = Identifier::unique(name);
        let wasm_name = name.to_string();
        let declaration_level = None;
        let kind = VariableKind::Local;
        let content = VariableInfoContent { name, ty, kind, declaration_level, wasm_name };

        Link::new(content)
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

    pub fn get_wasm_name(&self) -> String {
        self.borrow().wasm_name.clone()
    }

    pub fn set_type(&self, ty: Type) {
        self.with_mut(|mut var_info| {
            var_info.ty = ty;
        });
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
}

impl Default for VariableInfo {
    fn default() -> Self {
        Link::new(VariableInfoContent {
            name: Identifier::default(),
            ty: Type::Undefined,
            kind: VariableKind::Local,
            declaration_level: None,
            wasm_name: String::new(),
        })
    }
}