use std::{collections::HashMap, ops::Deref};
use parsable::Parsable;
use crate::{generation::WasmModule, items::Identifier};
use super::{ConstantAnnotation, Error, Type, FunctionAnnotation, StructAnnotation};

#[derive(Default)]
pub struct ProgramContext {
    pub errors: Vec<Error>,
    pub wasm: WasmModule,

    pub structs: HashMap<Identifier, StructAnnotation>,
    pub functions: HashMap<Identifier, FunctionAnnotation>,
    pub constants: HashMap<Identifier, ConstantAnnotation>,
    
    pub scopes: Vec<HashMap<Identifier, VarInfo>>,
    pub this_type: Option<VarInfo>,
    pub payload_type: Option<VarInfo>,
    pub visited_constants: Vec<Identifier>,
    pub inside_const_expr: bool,
    pub function_return_type: Option<Type>
}

impl ProgramContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn error<T : Parsable, S : Deref<Target=str>>(&mut self, data: &T, error: S) {
        self.errors.push(Error::located(data, error));
    }

    pub fn visit_constant(&mut self, constant_name: &Identifier) -> Option<&Identifier> {
        self.visited_constants.iter().find(|name| *name == constant_name)
    }

    pub fn get_return_type(&self) -> Option<Type> {
        self.function_return_type.clone()
    }

    pub fn get_this_type(&self) -> Option<Type> {
        self.this_type.and_then(|info| Some(info.expr_type.clone()))
    }

    pub fn get_payload_type(&self) -> Option<Type> {
        self.payload_type.and_then(|info| Some(info.expr_type.clone()))
    }

    pub fn set_this_type(&mut self, expr_type: Option<VarInfo>) {
        self.this_type = expr_type;
    }

    pub fn set_payload_type(&mut self, expr_type: Option<VarInfo>) {
        self.payload_type = expr_type;
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn get_var_ref(&mut self, name: &Identifier) -> Option<&mut VarInfo> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(expr_type) = scope.get_mut(name) {
                return Some(expr_type);
            }
        }

        None
    }

    pub fn var_exists(&mut self, name: &Identifier) -> bool {
        self.get_var_ref(name).is_some()
    }

    // pub fn set_var_type(&mut self, name: &Identifier, var_type: ExpressionType) {
    //     match self.get_var_ref(name) {
    //         Some(var_info) => var_info.expr_type = var_type,
    //         None => { },
    //     }
    // }

    pub fn get_var_info(&mut self, name: &Identifier) -> Option<VarInfo> {
        self.get_var_ref(name).cloned()
    }

    pub fn push_var(&mut self, name: Identifier, var_type: VarInfo) {
        self.scopes.last_mut().unwrap().insert(name.clone(), var_type);
    }

    pub fn get_method_signature(&self, struct_name: &Identifier, method_name: &Identifier) -> Option<(Vec<(Identifier, Type)>, Type)> {
        if let Some(struct_annotation) = self.structs.get(&struct_name) {
            if let Some(method_annotation) = struct_annotation.methods.get(method_name) {
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

#[derive(Debug, Clone)]
pub struct VarInfo {
    pub name: Identifier,
    pub expr_type: Type,
}

impl VarInfo {
    pub fn new(name: Identifier, expr_type: Type) -> Self {
        Self { name, expr_type }
    }
}