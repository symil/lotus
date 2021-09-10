use std::{cell::UnsafeCell, collections::{HashMap, HashSet}, mem::{self, take}, ops::Deref};
use indexmap::IndexSet;
use parsable::{DataLocation, Parsable};
use crate::{generation::{GENERATED_METHOD_COUNT_PER_TYPE, HEADER_FUNCTIONS, HEADER_FUNC_TYPES, HEADER_GLOBALS, HEADER_IMPORTS, HEADER_MEMORIES, ToWat, ToWatVec, Wat}, items::{Identifier, LotusFile, TopLevelBlock}, wat};
use super::{ActualTypeInfo, BuiltinInterface, BuiltinType, Error, ErrorList, FunctionBlueprint, GeneratedMethods, GlobalVarBlueprint, GlobalVarInstance, Id, InterfaceBlueprint, ItemIndex, Scope, ScopeKind, StructInfo, Type, TypeBlueprint, TypeOld, VariableInfo, VariableKind, Wasm};

#[derive(Default, Debug)]
pub struct ProgramContext {
    pub errors: ErrorList,

    pub types: ItemIndex<TypeBlueprint>,
    pub interfaces: ItemIndex<InterfaceBlueprint>,
    pub functions: ItemIndex<FunctionBlueprint>,
    pub globals: ItemIndex<GlobalVarBlueprint>,

    pub builtin_types: HashMap<BuiltinType, (String, u64)>,
    pub builtin_interfaces: HashMap<BuiltinInterface, (String, u64, String)>,

    pub current_function: Option<u64>,
    pub current_type: Option<u64>,
    pub current_interface: Option<u64>,
    pub scopes: Vec<Scope>,
    pub depth: i32,
    pub return_found: bool,
}

impl ProgramContext {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_builtin_type_info(&self, builtin_type: BuiltinType) -> ActualTypeInfo {
        let (type_name, type_id) = self.builtin_types.get(&builtin_type).unwrap();

        ActualTypeInfo {
            name: type_name.clone(),
            type_id: type_id.clone(),
            parameters: vec![],
        }
    }

    pub fn bool_type(&self) -> Type {
        Type::Actual(self.get_builtin_type_info(BuiltinType::Bool))
    }

    pub fn int_type(&self) -> Type {
        Type::Actual(self.get_builtin_type_info(BuiltinType::Int))
    }

    pub fn float_type(&self) -> Type {
        Type::Actual(self.get_builtin_type_info(BuiltinType::Float))
    }

    pub fn array_type(&self, item_type: Type) -> Type {
        let mut info = self.get_builtin_type_info(BuiltinType::Array);

        info.parameters.push(item_type);

        Type::Actual(info)
    }

    pub fn get_builtin_interface(&self, interface: BuiltinInterface) -> (u64, &str) {
        let (interface_name, interface_id, method_name) = self.builtin_interfaces.get(&interface).unwrap();

        (*interface_id, method_name)
    }

    pub fn reset_local_scope(&mut self) {
        self.return_found = false;
        self.scopes = vec![];
    }

    pub fn push_scope(&mut self, kind: ScopeKind) {
        self.depth += kind.get_depth();
        self.scopes.push(Scope::new(kind, self.depth));
    }

    pub fn pop_scope(&mut self) {
        if let Some(scope) = self.scopes.pop() {
            self.depth -= scope.kind.get_depth();
        }
    }

    pub fn get_scope_depth(&self, kind: ScopeKind) -> Option<i32> {
        for scope in self.scopes.iter().rev() {
            if scope.kind == kind {
                return Some(self.depth - scope.depth);
            }
        }

        None
    }

    pub fn push_var(&mut self, name: &Identifier, ty: &Type, kind: VariableKind) -> VariableInfo {
        let var_info = VariableInfo::new(name.to_unique_string(), ty.clone(), kind);

        // global scope is handled differently
        if let Some(current_scope) = self.scopes.iter_mut().last() {
            current_scope.insert_var_info(name, var_info.clone());
        }

        var_info
    }

    pub fn get_var_info(&self, name: &Identifier) -> Option<VariableInfo> {
        for scope in self.scopes.iter().rev() {
            if let Some(var_info) = scope.get_var_info(name) {
                return Some(var_info.clone());
            }
        }

        match self.get_global_by_name(name) {
            Some(global) => Some(global.var_info.clone()),
            None => None,
        }
    }

    pub fn ckeck_var_unicity(&mut self, name: &Identifier) -> bool {
        let is_unique = self.get_var_info(name).is_none();

        if !is_unique {
            self.errors.add(name, format!("variable `{}` already exists in this scope", name));
        }

        is_unique
    }

    pub fn get_current_type(&self) -> Option<&TypeBlueprint> {
        if let Some(type_id) = &self.current_type {
            self.types.get_by_id(*type_id)
        } else {
            None
        }
    }

    pub fn call_builtin_interface<L, F>(&mut self, location: &L, interface: BuiltinInterface, target_type: &Type, argument_types: &[&Type], make_error_prefix: F) -> Option<Wasm>
        where
            L : Deref<Target=DataLocation>,
            F : FnMut() -> String
    {
        let mut ok = true;
        let (interface_id, method_name) = self.get_builtin_interface(interface);
        let interface_blueprint = self.interfaces.get_by_id(interface_id).unwrap();
        let method_info = interface_blueprint.methods.get(method_name).unwrap().clone();

        if !target_type.match_interface(interface_id, self) {
            self.errors.add(location, format!("type `{}` does not implement method `{}`", target_type, method_name));
            ok = false;
        }

        for (expected_arg_type, actual_arg_type) in method_info.arguments.iter().zip(argument_types.iter()) {
            if !actual_arg_type.is_assignable_to(expected_arg_type, self) {
                let prefix = make_error_prefix();
                self.errors.add(location, format!("{}: expected `{}`, got `{}`", prefix, expected_arg_type, actual_arg_type));
                ok = false;
            }
        }

        let result = Wasm {
            ty: method_info.return_type.clone().unwrap_or(Type::Void),
            wat: vec![target_type.method_call_placeholder(method_name)],
            variables: vec![],
        };

        match ok {
            true => Some(result),
            false => None
        }
    }

    pub fn call_builtin_interface_no_arg<L>(&mut self, location: &L, interface: BuiltinInterface, target_type: &Type) -> Option<Wasm>
        where
            L : Deref<Target=DataLocation>
    {
        self.call_builtin_interface(location, interface, target_type, &[], || String::new())
    }

    // pub fn get_type_id(&mut self, ty: &Type) -> Id {
    //     match self.types_ids.get(ty) {
    //         Some(id) => *id,
    //         None => {
    //             let id = self.types_ids.len();
    //             self.types_ids.insert(ty.clone(), id);

    //             id
    //         }
    //     }
    // }

    pub fn process_files(&mut self, files: Vec<LotusFile>) {
        let mut structs = vec![];
        let mut functions = vec![];
        let mut globals = vec![];

        for file in files {
            for block in file.blocks {
                match block {
                    TopLevelBlock::StructDeclaration(mut struct_declaration) => {
                        structs.push(struct_declaration);
                    },
                    TopLevelBlock::FunctionDeclaration(mut function_declaration) => {
                        functions.push(function_declaration);
                    },
                    TopLevelBlock::GlobalDeclaration(mut global_declaration) => {
                        globals.push(global_declaration);
                    },
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
            struct_declaration.process_inheritance_chain(index, self);
        }

        for (index, struct_declaration) in structs.iter().enumerate() {
            struct_declaration.process_fields(index, self);
        }

        for (index, struct_declaration) in structs.iter().enumerate() {
            struct_declaration.process_fields_inheritance(index, self);
        }

        for (index, struct_declaration) in structs.iter().enumerate() {
            struct_declaration.process_method_signatures(index, self);
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

        for (index, struct_declaration) in structs.iter().enumerate() {
            struct_declaration.process_methods_bodies(index, self);
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

        for (file_namespace1, file_namespace2, func_name, arguments, return_type) in HEADER_IMPORTS {
            content.push(Wat::import_function(file_namespace1, file_namespace2, func_name, arguments.to_vec(), return_type.clone()));
        }

        for (export_name, page_count) in HEADER_MEMORIES {
            content.push(match export_name {
                Some(name) => wat!["memory", Wat::export("memory"), page_count],
                None => wat!["memory", page_count]
            });
        }

        for (type_name, arguments, result) in HEADER_FUNC_TYPES {
            content.push(Wat::declare_function_type(type_name, arguments, result.clone()));
        }

        let func_table_size = self.structs.len() * GENERATED_METHOD_COUNT_PER_TYPE;
        content.push(wat!["table", func_table_size, "funcref"]);

        let mut generated_methods_table_populate = vec![];
        let mut generated_methods_declarations = vec![];

        for struct_annotation in self.structs.id_to_item.values() {
            let generated_methods = GeneratedMethods::new(struct_annotation);
            let (retain_name, retain_declaration) = generated_methods.retain;
            let table_offset = struct_annotation.get_id() * GENERATED_METHOD_COUNT_PER_TYPE;

            generated_methods_table_populate.push(wat!["elem", Wat::const_i32(table_offset), Wat::var_name(&retain_name)]);
            generated_methods_declarations.push(retain_declaration);
        }

        content.extend(generated_methods_table_populate);

        for (var_name, var_type) in HEADER_GLOBALS {
            content.push(Wat::declare_global(var_name, var_type));
        }

        let mut init_globals_body = vec![];

        for global in get_globals_sorted(take(&mut self.globals)) {
            let wat = match global.var_info.ty {
                TypeOld::Float => Wat::declare_global_f32(&global.var_info.wasm_name, 0.),
                _ => Wat::declare_global_i32(&global.var_info.wasm_name, 0),
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

        for struct_annotation in self.structs.id_to_item.into_values() {
            for method in struct_annotation.regular_methods.into_values() {
                content.push(method.wat);
            }
            
            for method in struct_annotation.static_methods.into_values() {
                content.push(method.wat);
            }
        }

        content.extend(generated_methods_declarations);
        
        Ok(content.to_string(0))
    }
}
