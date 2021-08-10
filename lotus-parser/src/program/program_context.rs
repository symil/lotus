use std::{collections::HashMap, ops::Deref};
use parsable::{DataLocation, Parsable};
use crate::{generation::{PAYLOAD_VAR_NAME, THIS_VAR_NAME, WasmModule}, items::Identifier};
use super::{Error, FunctionAnnotation, GlobalAnnotation, StructAnnotation, Type, VariableScope, VecHashMap};

#[derive(Default)]
pub struct ProgramContext {
    pub errors: Vec<Error>,
    pub wasm: WasmModule,

    pub world_struct_name: Option<Identifier>,
    pub user_struct_name: Option<Identifier>,
    pub structs: VecHashMap<Identifier, StructAnnotation>,
    pub functions: VecHashMap<Identifier, FunctionAnnotation>,
    pub globals: VecHashMap<Identifier, GlobalAnnotation>,

    pub local_variables: HashMap<Identifier, VarInfo>,
    pub this_var: Option<VarInfo>,
    pub payload_var: Option<VarInfo>,
    pub function_return_type: Option<Type>,
    pub function_depth: usize,
    pub current_scope: VariableScope
}

#[derive(Debug, Clone)]
pub struct VarInfo {
    pub wasm_name: String,
    pub ty: Type,
}

impl ProgramContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn error<S : Deref<Target=str>>(&mut self, location: &DataLocation, error: S) {
        self.errors.push(Error::located(location, error));
    }

    pub fn reset_local_scope(&mut self, variable_scope: VariableScope) {
        self.current_scope = variable_scope;
        self.function_return_type = None;
        self.this_var = None;
        self.payload_var = None;
        self.function_depth = 0;
        self.local_variables.clear();
    }

    pub fn set_function_return_type(&mut self, return_type: Option<Type>) {
        self.function_return_type = return_type;
    }

    pub fn set_this_type(&mut self, ty: Option<Type>) {
        self.this_var = ty.and_then(|t| Some(VarInfo::new(THIS_VAR_NAME.to_string(), t)));
    }

    pub fn set_payload_type(&mut self, ty: Option<Type>) {
        self.payload_var = ty.and_then(|t| Some(VarInfo::new(PAYLOAD_VAR_NAME.to_string(), t)));
    }

    pub fn push_local_var(&mut self, name: &Identifier, ty: &Type) {
        self.local_variables.insert(name.clone(), VarInfo::new(name.to_string(), ty.clone()));
    }

    pub fn get_var_info(&self, name: &Identifier) -> Option<VarInfo> {
        if let Some(local_var) = self.local_variables.get(name) {
            return Some(local_var.clone());
        }

        if let Some(global_annotation) = self.globals.get(name) {
            return Some(VarInfo::new(global_annotation.wasm_name.clone(), global_annotation.ty));
        }

        None
    }

    pub fn var_exists(&self, name: &Identifier) -> bool {
        self.get_var_info(name).is_some()
    }

    pub fn get_method_signature(&self, struct_name: &Identifier, method_name: &Identifier) -> Option<(Vec<(Identifier, Type)>, Type)> {
        if let Some(struct_annotation) = self.structs.get(&struct_name) {
            if let Some(method_annotation) = struct_annotation.user_methods.get(method_name) {
                Some((method_annotation.arguments.clone(), method_annotation.return_type.clone()))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_function_signatures(&self, function_name: &Identifier) -> Option<(Vec<(Identifier, Type)>, Type)> {
        if let Some(function_annotation) = self.functions.get(function_name) {
            Some((function_annotation.arguments.clone(), function_annotation.return_type.clone()))
        } else {
            None
        }
    }
}

impl VarInfo {
    pub fn new(wasm_name: String, ty: Type) -> Self {
        Self { wasm_name, ty }
    }
}