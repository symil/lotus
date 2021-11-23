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
    pub declaration_level: u32,
    pub is_closure_arg: bool
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

    pub fn is_global(&self) -> bool {
        match self {
            VariableKind::Global => true,
            _ => false
        }
    }
}

impl VariableInfo {
    pub fn create(name: Identifier, ty: Type, kind: VariableKind, declaration_level: u32) -> Self {
        let wasm_name = name.to_unique_string();
        let is_closure_arg = false;
        let content = VariableInfoContent { name, ty, kind, declaration_level, wasm_name, is_closure_arg };

        Link::new(content)
    }

    pub fn tmp(name: &str, ty: Type) -> Self {
        let name = Identifier::unique(name);
        let wasm_name = name.to_string();
        let declaration_level = u32::MAX;
        let kind = VariableKind::Local;
        let is_closure_arg = false;
        let content = VariableInfoContent { name, ty, kind, declaration_level, wasm_name, is_closure_arg };

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

    pub fn resolve_wasm_type(&self, type_index: &TypeIndex, context: &mut ProgramContext) -> Option<&'static str> {
        self.with_ref(|var_info| -> Option<&str> {
            match var_info.ty.resolve(type_index, context).wasm_type {
                Some(wasm_type) => match var_info.is_closure_arg {
                    true => Some("i32"),
                    false => Some(wasm_type),
                },
                None => None,
            }
        })
    }

    pub fn set_type(&self, ty: Type) {
        self.with_mut(|mut var_info| {
            var_info.ty = ty;
        });
    }

    pub fn mark_as_closure_arg(&self) {
        self.with_mut(|mut var_info| {
            if !var_info.kind.is_global() {
                var_info.is_closure_arg = true;
            }
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
            declaration_level: u32::MAX,
            wasm_name: String::new(),
            is_closure_arg: false
        })
    }
}