use std::{collections::HashMap, ops::Deref};
use parsable::{DataLocation, Parsable};
use crate::{generation::{ENTRY_POINT_FUNC_NAME, HEADER_FUNCTIONS, HEADER_GLOBALS, HEADER_IMPORTS, HEADER_MEMORIES, INIT_GLOBALS_FUNC_NAME, PAYLOAD_VAR_NAME, THIS_VAR_NAME, ToWat, ToWatVec, WasmModule, Wat}, items::{Identifier, LotusFile, TopLevelBlock}, wat};
use super::{Error, FunctionAnnotation, GlobalAnnotation, Id, ItemIndex, Scope, ScopeKind, StructAnnotation, StructInfo, Type, VariableInfo, VariableKind};

#[derive(Default, Debug)]
pub struct ProgramContext {
    pub errors: Vec<Error>,
    current_namespace_name: String,
    current_file_name: String,

    world_struct_name: Option<Identifier>,
    user_struct_name: Option<Identifier>,
    structs: ItemIndex<StructAnnotation>,
    functions: ItemIndex<FunctionAnnotation>,
    globals: ItemIndex<GlobalAnnotation>,

    pub scopes: Vec<Scope>,
    pub this_var: Option<VariableInfo>,
    pub payload_var: Option<VariableInfo>,
    pub function_return_type: Option<Type>,
    pub function_depth: usize,
    pub return_found: bool
}

impl ProgramContext {
    pub fn new() -> Self {
        let mut value = Self::default();

        value.push_scope(ScopeKind::Global);

        value
    }

    pub fn error<S : Deref<Target=str>>(&mut self, location: &DataLocation, error: S) {
        self.errors.push(Error::located(location, error));
    }

    pub fn get_current_namespace_name(&self) -> String {
        self.current_namespace_name.clone()
    }

    pub fn get_current_file_name(&self) -> String {
        self.current_file_name.clone()
    }

    pub fn reset_local_scope(&mut self) {
        self.function_return_type = None;
        self.this_var = None;
        self.payload_var = None;
        self.function_depth = 0;
        self.return_found = false;

        while self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn push_scope(&mut self, kind: ScopeKind) {
        self.scopes.push(Scope::new(kind));
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn set_function_return_type(&mut self, return_type: Option<Type>) {
        self.function_return_type = return_type;
    }

    pub fn set_this_type(&mut self, ty: Option<Type>) {
        self.this_var = ty.and_then(|t| Some(VariableInfo::new(THIS_VAR_NAME.to_string(), t, VariableKind::Argument)));
    }

    pub fn set_payload_type(&mut self, ty: Option<Type>) {
        self.payload_var = ty.and_then(|t| Some(VariableInfo::new(PAYLOAD_VAR_NAME.to_string(), t, VariableKind::Argument)));
    }

    pub fn push_var(&mut self, name: &Identifier, ty: &Type, scope: VariableKind) -> VariableInfo {
        let current_scope = self.scopes.iter_mut().last().unwrap();
        let var_info = VariableInfo::new(name.to_unique_string(), ty.clone(), scope);

        current_scope.insert_var_info(name, var_info.clone());

        var_info
    }

    pub fn get_var_info(&self, name: &Identifier) -> Option<VariableInfo> {
        for scope in self.scopes.iter().rev() {
            if let Some(var_info) = scope.get_var_info(name) {
                return Some(var_info.clone());
            }
        }

        None
    }

    pub fn ckeck_var_unicity(&mut self, name: &Identifier) -> bool {
        let is_unique = self.get_var_info(name).is_none();

        if !is_unique {
            self.error(name, format!("variable `{}` already exists in this scope", name));
        }

        is_unique
    }

    // Structures

    pub fn get_struct_by_name(&self, name: &Identifier) -> Option<&StructAnnotation> {
        self.structs.get_by_name(name, &self.current_file_name, &self.current_namespace_name)
    }

    pub fn get_struct_by_id(&mut self, id: Id) -> Option<&StructAnnotation> {
        self.structs.get_by_id(id)
    }

    pub fn get_struct_by_id_mut(&mut self, id: Id) -> Option<&mut StructAnnotation> {
        self.structs.get_mut_by_id(id)
    }

    pub fn add_struct(&mut self, value: StructAnnotation) {
        self.structs.insert(value);
    }

    // Functions

    pub fn get_function_by_name(&self, name: &Identifier) -> Option<&FunctionAnnotation> {
        self.functions.get_by_name(name, &self.current_file_name, &self.current_namespace_name)
    }

    pub fn get_function_by_id(&mut self, id: Id) -> Option<&FunctionAnnotation> {
        self.functions.get_by_id(id)
    }

    pub fn get_function_by_id_mut(&mut self, id: Id) -> Option<&mut FunctionAnnotation> {
        self.functions.get_mut_by_id(id)
    }

    pub fn add_function(&mut self, value: FunctionAnnotation) {
        self.functions.insert(value);
    }

    // Globals

    pub fn get_global_by_name(&self, name: &Identifier) -> Option<&GlobalAnnotation> {
        self.globals.get_by_name(name, &self.current_file_name, &self.current_namespace_name)
    }

    pub fn get_global_by_id(&mut self, id: Id) -> Option<&GlobalAnnotation> {
        self.globals.get_by_id(id)
    }

    pub fn get_global_by_id_mut(&mut self, id: Id) -> Option<&mut GlobalAnnotation> {
        self.globals.get_mut_by_id(id)
    }

    pub fn add_global(&mut self, value: GlobalAnnotation) {
        self.globals.insert(value);
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

        if self.get_function_by_name(&main_identifier).is_none() {
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

        let mut init_globals_body = vec![];

        for global in get_globals_sorted(self.globals) {
            let wat = match global.ty {
                Type::Float => Wat::declare_global_f32(&global.wasm_name, 0.),
                _ => Wat::declare_global_i32(&global.wasm_name, 0),
            };

            content.push(wat);

            init_globals_body.extend(global.value);
        }

        for (name, args, ret, locals, body) in HEADER_FUNCTIONS {
            content.push(Wat::declare_function(name, None, args.to_vec(), ret.clone(), locals.to_vec(), body()))
        }

        content.push(Wat::declare_function(INIT_GLOBALS_FUNC_NAME, None, vec![], None, vec![], init_globals_body));
        content.push(Wat::declare_function(ENTRY_POINT_FUNC_NAME, Some("_start"), vec![], None, vec![], vec![
            Wat::call(INIT_GLOBALS_FUNC_NAME, vec![]),
            Wat::call(self.get_function_by_name(&main_identifier).unwrap().wasm_name.as_str(), vec![]),
        ]));

        for function in self.functions.id_to_item.into_values() {
            content.push(function.wat);
        }
        
        Ok(content.to_string(0))
    }
}

fn get_globals_sorted(mut map: ItemIndex<GlobalAnnotation>) -> Vec<GlobalAnnotation> {
    let mut list = vec![];

    for global in map.id_to_item.into_values() {
        list.push(global);
    }

    list.sort_by_key(|global| global.metadata.id);

    list
}