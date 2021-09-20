use std::{cell::UnsafeCell, collections::{HashMap, HashSet}, hash::Hash, mem::{self, take}, ops::Deref, rc::{Rc, Weak}};
use indexmap::IndexSet;
use parsable::{DataLocation, Parsable};
use crate::{items::{Identifier, LotusFile, TopLevelBlock}, program::{ENTRY_POINT_FUNC_NAME, HEADER_FUNCTIONS, HEADER_FUNC_TYPES, HEADER_GLOBALS, HEADER_IMPORTS, HEADER_MEMORIES, INIT_GLOBALS_FUNC_NAME, ItemGenerator, VI, Wat}, utils::Link, vasm, wat};
use super::{ActualTypeInfo, BuiltinInterface, BuiltinType, Error, ErrorList, FunctionBlueprint, FunctionInstanceContent, FunctionInstanceHeader, FunctionInstanceParameters, GeneratedItemIndex, GlobalItemIndex, GlobalVarBlueprint, GlobalVarInstance, Id, InterfaceBlueprint, Scope, ScopeKind, Type, TypeBlueprint, TypeInstanceContent, TypeInstanceHeader, TypeInstanceParameters, VariableInfo, VariableKind, Vasm};

#[derive(Default, Debug)]
pub struct ProgramContext {
    pub errors: ErrorList,

    pub types: GlobalItemIndex<TypeBlueprint>,
    pub interfaces: GlobalItemIndex<InterfaceBlueprint>,
    pub functions: GlobalItemIndex<FunctionBlueprint>,
    pub global_vars: GlobalItemIndex<GlobalVarBlueprint>,

    pub current_function: Option<Link<FunctionBlueprint>>,
    pub current_type: Option<Link<TypeBlueprint>>,
    pub current_interface: Option<Link<InterfaceBlueprint>>,
    pub scopes: Vec<Scope>,
    pub depth: u32,
    pub return_found: bool,

    // pub instances: GeneratedItems,
    pub type_instances: GeneratedItemIndex<TypeInstanceHeader, TypeInstanceContent>,
    pub function_instances: GeneratedItemIndex<FunctionInstanceHeader, FunctionInstanceContent>,
    pub global_var_instances: Vec<GlobalVarInstance>,
    pub entry_function: Option<Rc<FunctionInstanceHeader>>
}

impl ProgramContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_builtin_type(&self, builtin_type: BuiltinType, parameters: Vec<Type>) -> Type {
        let type_name = builtin_type.get_name();
        let type_blueprint = self.types.get_by_name(type_name).unwrap_or_else(|| panic!("undefined builtin type `{}`", type_name));
        let mut info = ActualTypeInfo {
            type_wrapped: type_blueprint,
            parameters: vec![],
        };

        for ty in parameters {
            info.parameters.push(ty);
        }

        Type::Actual(info)
    }

    pub fn get_this_type(&self) -> Type {
        if let Some(type_wrapped) = &self.current_type {
            Type::Actual(type_wrapped.get_info())
        } else if let Some(interface_wrapped) = &self.current_interface {
            Type::This(interface_wrapped.clone())
        } else {
            Type::Void
        }
    }

    pub fn bool_type(&self) -> Type {
        self.get_builtin_type(BuiltinType::Bool, vec![])
    }

    pub fn int_type(&self) -> Type {
        self.get_builtin_type(BuiltinType::Int, vec![])
    }

    pub fn get_builtin_interface(&self, interface: BuiltinInterface) -> Link<InterfaceBlueprint> {
        let interface_name = interface.get_name();
        let interface_blueprint = self.interfaces.get_by_name(interface_name).unwrap_or_else(|| panic!("undefined builtin interface `{}`", interface_name));

        interface_blueprint
    }

    pub fn get_current_function(&self) -> Option<Link<FunctionBlueprint>> {
        self.current_function.as_ref().and_then(|f| Some(f.clone()))
    }

    pub fn get_current_type(&self) -> Option<Link<TypeBlueprint>> {
        self.current_type.as_ref().and_then(|f| Some(f.clone()))
    }

    pub fn get_current_interface(&self) -> Option<Link<InterfaceBlueprint>> {
        self.current_interface.as_ref().and_then(|f| Some(f.clone()))
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

    pub fn get_scope_depth(&self, kind: ScopeKind) -> Option<u32> {
        for scope in self.scopes.iter().rev() {
            if scope.kind == kind {
                return Some(self.depth - scope.depth);
            }
        }

        None
    }

    pub fn push_var(&mut self, var_info: &Rc<VariableInfo>) {
        // global scope is handled differently
        if let Some(current_scope) = self.scopes.iter_mut().last() {
            current_scope.insert_var_info(&var_info);
        }
    }

    pub fn get_var_info(&self, name: &Identifier) -> Option<Rc<VariableInfo>> {
        for scope in self.scopes.iter().rev() {
            if let Some(var_info) = scope.get_var_info(name.as_str()) {
                return Some(var_info.clone());
            }
        }

        match self.global_vars.get_by_identifier(name) {
            Some(global) => Some(global.borrow().var_info.clone()),
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

    pub fn call_builtin_interface<L, F>(&mut self, location: &L, interface: BuiltinInterface, target_type: &Type, argument_types: &[&Type], make_error_prefix: F) -> Option<Vasm>
        where
            L : Deref<Target=DataLocation>,
            F : Fn() -> String
    {
        let interface_wrapped = self.get_builtin_interface(interface);

        interface_wrapped.with_ref(|interface_unwrapped| {
            let (_, method_wrapped) = interface_unwrapped.methods.first().unwrap();

            method_wrapped.with_ref(|function_unwrapped| {
                let method_name = function_unwrapped.name.as_str();
                let mut ok = true;

                if !target_type.match_interface(&interface_wrapped) {
                    self.errors.add(location, format!("type `{}` does not implement method `{}`", target_type, method_name));
                    ok = false;
                }

                for (expected_arg, actual_arg_type) in function_unwrapped.arguments.iter().zip(argument_types.iter()) {
                    let expected_type = expected_arg.ty.replace_generics(target_type, &[]);

                    if !actual_arg_type.is_assignable_to(&expected_type) {
                        let prefix = make_error_prefix();
                        self.errors.add(location, format!("{}: expected `{}`, got `{}`", prefix, &expected_arg.ty, actual_arg_type));
                        ok = false;
                    }
                }

                let ty = function_unwrapped.return_value.as_ref().and_then(|ret| Some(ret.ty.replace_generics(target_type, &[]))).unwrap_or(Type::Void);
                let method_instruction = VI::call_method(target_type, method_wrapped.clone(), &[], vasm![]);
                let result = Vasm::new(ty, vec![], vec![method_instruction]);

                match ok {
                    true => Some(result),
                    false => None
                }
            })
        })
    }

    pub fn call_builtin_interface_no_arg<L>(&mut self, location: &L, interface: BuiltinInterface, target_type: &Type) -> Option<Vasm>
        where
            L : Deref<Target=DataLocation>
    {
        self.call_builtin_interface(location, interface, target_type, &[], || String::new())
    }

    pub fn process_files(&mut self, files: Vec<LotusFile>) {
        let mut interfaces = vec![];
        let mut types = vec![];
        let mut functions = vec![];
        let mut global_vars = vec![];

        for file in files {
            for block in file.blocks {
                match block {
                    TopLevelBlock::InterfaceDeclaration(interface_declaration) => interfaces.push(interface_declaration),
                    TopLevelBlock::TypeDeclaration(struct_declaration) => types.push(struct_declaration),
                    TopLevelBlock::FunctionDeclaration(function_declaration) => functions.push(function_declaration),
                    TopLevelBlock::GlobalDeclaration(mut global_declaration) => global_vars.push(global_declaration),
                }
            }
        }

        for interface_declaration in &interfaces {
            interface_declaration.process_name(self);
        }

        for type_declaration in &types {
            type_declaration.process_name(self);
        }

        for interface_declaration in &interfaces {
            interface_declaration.process_associated_types(self);
        }

        for type_declaration in &types {
            type_declaration.process_associated_types(self);
        }

        for interface_declaration in &interfaces {
            interface_declaration.process_methods(self);
        }

        for type_declaration in &types {
            type_declaration.process_parent(self);
        }

        for type_declaration in &types {
            type_declaration.process_inheritance_chain(self);
        }

        for type_declaration in &types {
            type_declaration.process_fields(self);
        }
        
        for type_declaration in &types {
            type_declaration.process_fields_inheritance(self);
        }

        for type_declaration in &types {
            type_declaration.process_method_signatures(self);
        }

        for type_declaration in &types {
            type_declaration.process_methods_inheritance(self);
        }

        for function_declaration in &functions {
            function_declaration.process_signature(self);
        }

        for global_var_declaration in &global_vars {
            global_var_declaration.process(self);
        }

        for type_declaration in &types {
            type_declaration.process_methods_bodies(self);
        }

        for function_declaration in &functions {
            function_declaration.process_body(self);
        }
    }

    pub fn generate_instances(&mut self) {
        for global_var in self.global_vars.get_all() {
            let global_var_instance = global_var.with_ref(|global_var_unwrapped| {
                global_var_unwrapped.generate_instance(self)
            });

            self.global_var_instances.push(global_var_instance);
        }

        let main_identifier = Identifier::unlocated("main");

        if let Some(function_wrapped) = self.functions.get_by_identifier(&main_identifier) {
            let parameters = FunctionInstanceParameters {
                function_blueprint: function_wrapped.clone(),
                this_type: None,
                function_parameters: vec![],
            };

            let (function_instance_header, exists) = self.function_instances.get_header(&parameters);
            let function_instance_content = parameters.generate_content(&function_instance_header, self);

            self.function_instances.set_content(&parameters, function_instance_content);
            self.entry_function = Some(function_instance_header);
        } else { 
            self.errors.add_unlocated(format!("missing required function `main`"));
        }
    }

    pub fn generate_wat(mut self) -> Result<String, Vec<Error>> {
        if !self.errors.is_empty() {
            return Err(self.errors.consume());
        }

        let mut content = wat!["module"];
        let mut globals_declaration = vec![];
        let mut globals_initialization = vec![];

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

        for global_var in self.global_var_instances {
            globals_declaration.push(Wat::declare_global(&global_var.wasm_name, global_var.wasm_type));
            globals_initialization.extend(global_var.init_value);
            globals_initialization.push(Wat::set_global_from_stack(&global_var.wasm_name));
        }

        // let func_table_size = self.structs.len() * GENERATED_METHOD_COUNT_PER_TYPE;
        // content.push(wat!["table", func_table_size, "funcref"]);

        // let mut generated_methods_table_populate = vec![];
        // let mut generated_methods_declarations = vec![];

        // for struct_annotation in self.structs.id_to_item.values() {
        //     let generated_methods = GeneratedMethods::new(struct_annotation);
        //     let (retain_name, retain_declaration) = generated_methods.retain;
        //     let table_offset = struct_annotation.get_id() * GENERATED_METHOD_COUNT_PER_TYPE;

        //     generated_methods_table_populate.push(wat!["elem", Wat::const_i32(table_offset), Wat::var_name(&retain_name)]);
        //     generated_methods_declarations.push(retain_declaration);
        // }

        // content.extend(generated_methods_table_populate);

        // for (var_name, var_type) in HEADER_GLOBALS {
        //     content.push(Wat::declare_global(var_name, var_type));
        // }

        // let mut init_globals_body = vec![];

        // for (name, args, ret, locals, body) in HEADER_FUNCTIONS {
        //     content.push(Wat::declare_function(name, None, args.to_vec(), ret.clone(), locals.to_vec(), body()))
        // }

        let mut entry_function_body = vec![Wat::call(INIT_GLOBALS_FUNC_NAME, vec![])];
        entry_function_body.extend(self.entry_function.unwrap().wasm_call.clone());

        content.extend(globals_declaration);
        content.push(Wat::declare_function(INIT_GLOBALS_FUNC_NAME, None, vec![], None, vec![], globals_initialization));
        content.push(Wat::declare_function(ENTRY_POINT_FUNC_NAME, Some("_start"), vec![], None, vec![], entry_function_body));

        for (_, function_instance_content) in self.function_instances.consume() {
            if let Some(wasm_declaration) = function_instance_content.wasm_declaration {
                content.push(wasm_declaration);
            }
        }

        // for function in self.functions.id_to_item.into_values() {
        //     content.push(function.wat);
        // }

        // for struct_annotation in self.structs.id_to_item.into_values() {
        //     for method in struct_annotation.regular_methods.into_values() {
        //         content.push(method.wat);
        //     }
            
        //     for method in struct_annotation.static_methods.into_values() {
        //         content.push(method.wat);
        //     }
        // }

        // content.extend(generated_methods_declarations);
        
        Ok(content.to_string(0))
    }
}
