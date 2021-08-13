use std::{collections::HashMap, ops::Deref};
use parsable::{DataLocation, Parsable};
use crate::{generation::{ENTRY_POINT_FUNC_NAME, HEADER_FUNCTIONS, HEADER_GLOBALS, HEADER_IMPORTS, HEADER_MEMORIES, INIT_GLOBALS_FUNC_NAME, PAYLOAD_VAR_NAME, THIS_VAR_NAME, ToWat, ToWatVec, WasmModule, Wat}, items::{Identifier, LotusFile, TopLevelBlock}, wat};
use super::{Error, FunctionAnnotation, GlobalAnnotation, StructAnnotation, Type, VariableScope, VecHashMap};

#[derive(Default, Debug)]
pub struct ProgramContext {
    pub errors: Vec<Error>,

    pub world_struct_name: Option<Identifier>,
    pub user_struct_name: Option<Identifier>,
    pub structs: VecHashMap<Identifier, StructAnnotation>,
    pub functions: VecHashMap<Identifier, FunctionAnnotation>,
    pub globals: VecHashMap<Identifier, GlobalAnnotation>,

    pub variables: HashMap<Identifier, VarInfo>,
    pub this_var: Option<VarInfo>,
    pub payload_var: Option<VarInfo>,
    pub function_return_type: Option<Type>,
    pub function_depth: usize,
    pub return_found: bool
}


#[derive(Debug, Clone)]
pub struct VarInfo {
    pub wasm_name: String,
    pub ty: Type,
    pub scope: VariableScope
}

impl ProgramContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn error<S : Deref<Target=str>>(&mut self, location: &DataLocation, error: S) {
        self.errors.push(Error::located(location, error));
    }

    pub fn reset_local_scope(&mut self) {
        self.function_return_type = None;
        self.this_var = None;
        self.payload_var = None;
        self.function_depth = 0;
        self.return_found = false;
        self.variables.retain(|_, var_info| var_info.scope == VariableScope::Global);
    }

    pub fn set_function_return_type(&mut self, return_type: Option<Type>) {
        self.function_return_type = return_type;
    }

    pub fn set_this_type(&mut self, ty: Option<Type>) {
        self.this_var = ty.and_then(|t| Some(VarInfo::new(THIS_VAR_NAME.to_string(), t, VariableScope::Argument)));
    }

    pub fn set_payload_type(&mut self, ty: Option<Type>) {
        self.payload_var = ty.and_then(|t| Some(VarInfo::new(PAYLOAD_VAR_NAME.to_string(), t, VariableScope::Argument)));
    }

    pub fn push_var(&mut self, name: &Identifier, ty: &Type, scope: VariableScope) {
        self.variables.insert(name.clone(), VarInfo::new(name.to_string(), ty.clone(), scope));
    }

    pub fn get_var_info(&self, name: &Identifier) -> Option<VarInfo> {
        self.variables.get(name).cloned()
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

    pub fn process_files(&mut self, files: Vec<LotusFile>) {
        let mut structs = vec![];
        let mut functions = vec![];
        let mut globals = vec![];

        for file in files {
            for block in file.blocks {
                match block {
                    TopLevelBlock::StructDeclaration(struct_declaration) => structs.push(struct_declaration),
                    TopLevelBlock::FunctionDeclaration(function_declaration) => functions.push(function_declaration),
                    TopLevelBlock::GlobalDeclaration(global_declaration) => globals.push(global_declaration),
                }
            }
        }

        for (index, struct_declaration) in structs.iter().enumerate() {
            struct_declaration.process_name(index, self);
        }

        for (index, struct_declaration) in structs.iter().enumerate() {
            struct_declaration.process_parent(index, self);
        }

        for (index, struct_declaration) in structs.iter().enumerate() {
            struct_declaration.process_inheritence(index, self);
        }

        for (index, struct_declaration) in structs.iter().enumerate() {
            struct_declaration.process_self_fields(index, self);
        }

        for (index, struct_declaration) in structs.iter().enumerate() {
            struct_declaration.process_all_fields(index, self);
        }

        for (index, struct_declaration) in structs.iter().enumerate() {
            struct_declaration.process_methods_signatures(index, self);
        }

        for (index, function_declaration) in functions.iter().enumerate() {
            function_declaration.process_signature(index, self);
        }

        for (index, global_declaration) in globals.iter().enumerate() {
            global_declaration.process(index, self);
        }

        for (index, function_declaration) in functions.iter().enumerate() {
            function_declaration.process_body(index, self);
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

        for (namespace1, namespace2, func_name, arguments, return_type) in HEADER_IMPORTS {
            content.push(Wat::import_function(namespace1, namespace2, func_name, arguments.to_vec(), return_type.clone()));
        }

        for (export_name, page_count) in HEADER_MEMORIES {
            content.push(match export_name {
                Some(name) => wat!["memory", Wat::export("memory"), page_count],
                None => wat!["memory", page_count]
            });
        }

        for (var_name, var_type) in HEADER_GLOBALS {
            content.push(Wat::declare_global(var_name, var_type));
        }

        for (name, args, ret, locals, body) in HEADER_FUNCTIONS {
            content.push(Wat::declare_function(name, None, args.to_vec(), ret.clone(), locals.to_vec(), body()))
        }

        let mut init_globals_body = vec![];

        for mut global_list in self.globals.hashmap.into_values() {
            let global = global_list.remove(0);
            let wat = match global.ty {
                Type::Float => Wat::declare_global_f32(&global.wasm_name, 0.),
                _ => Wat::declare_global_i32(&global.wasm_name, 0),
            };

            content.push(wat);

            init_globals_body.extend(global.value);
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
    pub fn new(wasm_name: String, ty: Type, scope: VariableScope) -> Self {
        Self { wasm_name, ty, scope }
    }
}