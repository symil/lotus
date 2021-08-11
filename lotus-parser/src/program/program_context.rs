use std::{collections::HashMap, ops::Deref};
use parsable::{DataLocation, Parsable};
use crate::{generation::{ENTRY_POINT_FUNC_NAME, IMPORT_LIST, INIT_GLOBALS_FUNC_NAME, PAYLOAD_VAR_NAME, THIS_VAR_NAME, ToWat, ToWatVec, WasmModule, Wat}, items::Identifier, wat};
use super::{Error, FunctionAnnotation, GlobalAnnotation, StructAnnotation, Type, VariableScope, VecHashMap};

#[derive(Default)]
pub struct ProgramContext {
    pub errors: Vec<Error>,

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
    pub return_found: bool,
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
        self.return_found = false;
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
            return Some(VarInfo::new(global_annotation.wasm_name.clone(), global_annotation.ty.clone()));
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

    pub fn generate_wat(mut self) -> Result<String, Vec<Error>> {
        let main_identifier = Identifier::new("main");

        if !self.functions.contains_key(&main_identifier) {
            self.errors.push(Error::unlocated(format!("missing required function `main`")));
        }

        if !self.errors.is_empty() {
            return Err(self.errors);
        }

        let mut content = wat!["module"];

        for (namespace1, namespace2, func_name, arguments, return_type) in IMPORT_LIST {
            content.push(Wat::import_function(namespace1, namespace2, func_name, arguments.to_vec(), return_type.clone()));
        }

        content.push(wat!["memory", Wat::export("memory"), 1]);

        let mut init_globals_body = vec![];

        for mut global_list in self.globals.hashmap.into_values() {
            let global = global_list.remove(0);
            let wat = match global.ty {
                Type::Float => Wat::declare_global_f32(&global.wasm_name, 0.),
                _ => Wat::declare_global_i32(&global.wasm_name, 0),
            };

            content.push(wat);

            init_globals_body.extend(global.value);
            init_globals_body.push(Wat::set_global_from_stack(&global.wasm_name));
        }

        content.push(Wat::declare_function(INIT_GLOBALS_FUNC_NAME, None, vec![], None, vec![], init_globals_body));
        content.push(Wat::declare_function(ENTRY_POINT_FUNC_NAME, Some("_start"), vec![], None, vec![], vec![
            Wat::call(INIT_GLOBALS_FUNC_NAME, vec![]),
            Wat::call(self.functions.get(&main_identifier).unwrap().wasm_name.as_str(), vec![]),
        ]));

        for mut function_list in self.functions.hashmap.into_values() {
            let function = function_list.remove(0);

            content.push(function.wat);
        }
        
        Ok(content.to_string(0))
    }
}

impl VarInfo {
    pub fn new(wasm_name: String, ty: Type) -> Self {
        Self { wasm_name, ty }
    }
}