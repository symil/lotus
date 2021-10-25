use std::{cell::UnsafeCell, collections::{HashMap, HashSet}, hash::Hash, mem::{self, take}, ops::Deref, rc::{Rc, Weak}};
use indexmap::IndexSet;
use colored::*;
use parsable::{DataLocation, Parsable};
use crate::{items::{Identifier, LotusFile, TopLevelBlock, TypeDeclaration}, program::{DUMMY_FUNC_NAME, ENTRY_POINT_FUNC_NAME, HEADER_FUNCTIONS, HEADER_FUNC_TYPES, HEADER_GLOBALS, HEADER_IMPORTS, HEADER_MEMORIES, INIT_GLOBALS_FUNC_NAME, ItemGenerator, VI, Wat, typedef_blueprint}, utils::{Link, sort_dependancy_graph}, vasm, wat};
use super::{ActualTypeContent, BuiltinInterface, BuiltinType, DEFAULT_INTERFACES, Error, ErrorList, FunctionBlueprint, FunctionInstanceContent, FunctionInstanceHeader, FunctionInstanceParameters, FunctionInstanceWasmType, GeneratedItemIndex, GlobalItemIndex, GlobalVarBlueprint, GlobalVarInstance, Id, InterfaceBlueprint, InterfaceList, Scope, ScopeKind, Type, TypeBlueprint, TypeInstanceContent, TypeInstanceHeader, TypeInstanceParameters, TypedefBlueprint, VariableInfo, VariableKind, Vasm};

#[derive(Default, Debug)]
pub struct ProgramContext {
    pub errors: ErrorList,

    pub types: GlobalItemIndex<TypeBlueprint>,
    pub typedefs: GlobalItemIndex<TypedefBlueprint>,
    pub interfaces: GlobalItemIndex<InterfaceBlueprint>,
    pub functions: GlobalItemIndex<FunctionBlueprint>,
    pub global_vars: GlobalItemIndex<GlobalVarBlueprint>,

    pub default_interfaces: InterfaceList,

    pub current_function: Option<Link<FunctionBlueprint>>,
    pub current_type: Option<Link<TypeBlueprint>>,
    pub current_interface: Option<Link<InterfaceBlueprint>>,
    pub scopes: Vec<Scope>,
    pub depth: u32,
    pub return_found: bool,
    pub iter_fields_counter: Option<usize>,

    pub dynamic_method_table: Vec<Option<Rc<FunctionInstanceHeader>>>,
    pub dynamic_method_wasm_types: HashMap<FunctionInstanceWasmType, String>,
    pub placeholder_to_wasm_type: HashMap<String, String>,
    pub type_instances: HashMap<u64, (Rc<TypeInstanceHeader>, Option<TypeInstanceContent>)>,
    pub function_instances: HashMap<u64, (Rc<FunctionInstanceHeader>, Option<FunctionInstanceContent>)>,
    pub global_var_instances: Vec<GlobalVarInstance>,
    pub entry_function: Option<Rc<FunctionInstanceHeader>>,
}

impl ProgramContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn index_builtin_entities(&mut self) {
        for builtin_interface in DEFAULT_INTERFACES {
            let interface = self.interfaces.get_by_name(builtin_interface.get_name()).unwrap();

            self.default_interfaces.push(interface);
        }
    }

    pub fn get_builtin_type(&self, builtin_type: BuiltinType, parameters: Vec<Type>) -> Type {
        let type_name = builtin_type.get_name();
        let type_blueprint = self.types.get_by_name(type_name).unwrap_or_else(|| panic!("undefined builtin type `{}`", type_name));
        let mut info = ActualTypeContent {
            type_blueprint,
            parameters: vec![],
        };

        for ty in parameters {
            info.parameters.push(ty);
        }

        Type::Actual(info)
    }

    pub fn get_this_type(&self) -> Type {
        if let Some(type_wrapped) = &self.current_type {
            type_wrapped.borrow().self_type.clone()
        } else if let Some(interface_wrapped) = &self.current_interface {
            Type::This(interface_wrapped.clone())
        } else {
            Type::Undefined
        }
    }

    pub fn bool_type(&self) -> Type {
        self.get_builtin_type(BuiltinType::Bool, vec![])
    }

    pub fn int_type(&self) -> Type {
        self.get_builtin_type(BuiltinType::Int, vec![])
    }
    
    pub fn pointer_type(&self) -> Type {
        self.get_builtin_type(BuiltinType::Pointer, vec![self.int_type()])
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

    pub fn get_type_instance(&mut self, parameters: TypeInstanceParameters) -> Rc<TypeInstanceHeader> {
        let id = parameters.get_id();

        if let Some((header, _)) = self.type_instances.get(&id) {
            return header.clone();
        }

        let header = TypeInstanceHeader::from_parameters(&parameters, self);

        self.type_instances.insert(id, (header.clone(), None));

        let content = TypeInstanceContent::from_parameters(&parameters, header.clone(), self);

        self.type_instances.get_mut(&id).unwrap().1.insert(content);

        header
    }

    pub fn get_function_instance(&mut self, parameters: FunctionInstanceParameters) -> Rc<FunctionInstanceHeader> {
        let id = parameters.get_id();

        if let Some((header, _)) = self.function_instances.get(&id) {
            return header.clone();
        }

        let header = FunctionInstanceHeader::from_parameters(&parameters, self);

        self.function_instances.insert(id, (header.clone(), None));

        let content = FunctionInstanceContent::from_parameters(&parameters, header.clone(), self);

        self.function_instances.get_mut(&id).unwrap().1.insert(content);

        header
    }

    pub fn get_function_instance_wasm_type_name(&mut self, wasm_type: FunctionInstanceWasmType) -> String {
        if let Some(name) = self.dynamic_method_wasm_types.get(&wasm_type) {
            return name.clone()
        }
        
        let name = format!("type_{}", self.dynamic_method_wasm_types.len() + 1);

        self.dynamic_method_wasm_types.insert(wasm_type, name.clone());

        name
    }

    pub fn process_files(&mut self, files: Vec<LotusFile>) {
        let mut interfaces = vec![];
        let mut types = vec![];
        let mut typedefs = vec![];
        let mut functions = vec![];
        let mut global_vars = vec![];

        for file in files {
            for block in file.blocks {
                match block {
                    TopLevelBlock::InterfaceDeclaration(interface_declaration) => interfaces.push(interface_declaration),
                    TopLevelBlock::TypeDeclaration(struct_declaration) => types.push(struct_declaration),
                    TopLevelBlock::TypedefDeclaration(typedef_declaration) => typedefs.push(typedef_declaration),
                    TopLevelBlock::FunctionDeclaration(function_declaration) => functions.push(function_declaration),
                    TopLevelBlock::GlobalDeclaration(mut global_declaration) => global_vars.push(global_declaration),
                }
            }
        }

        for interface_declaration in &interfaces {
            interface_declaration.process_name(self);
        }

        for (i, type_declaration) in types.iter().enumerate() {
            type_declaration.process_name(i, self);
        }

        self.index_builtin_entities();

        let mut links = vec![];
        for (i, type_declaration) in types.iter().enumerate() {
            let dependancies = type_declaration.compute_type_dependencies(self);

            // let mut s = format!("{}: ", &type_declaration.name);
            // for t in &dependancies {
            //     s.push_str(&format!("{}, ", t.borrow().name.as_str()));
            // }
            // println!("{}", s);

            let indexes = dependancies.into_iter().map(|ty| ty.borrow().declaration_index).collect();
            links.push(indexes);
        }

        match sort_dependancy_graph(links) {
            Ok(result) => {
                let mut wrapped : Vec<Option<TypeDeclaration>> = types.into_iter().map(|ty| Some(ty)).collect();
                types = vec![];

                for index in result {
                    types.push(take(&mut wrapped[index]).unwrap())
                }
            },
            Err(cycles) => {
                for cycle in cycles {
                    let first = &types[cycle[0]];
                    let mut s = format!("{}", &first.name.as_str().bold());

                    for index in &cycle[1..] {
                        s.push_str(&format!(" -> {}", types[*index].name.as_str().bold()));
                    }

                    self.errors.add(&first.name, format!("type dependancy cycle: {}", s));
                }

                return;
            },
        }

        // println!("====");
        // for t in &types {
        //     println!("{}", &t.name);
        // }

        for type_declaration in &types {
            type_declaration.process_parent(self);
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
        
        for typedef_declaration in &typedefs {
            typedef_declaration.process(self);
        }

        // types.sort_by_cached_key(|type_declaration| {
        //     type_declaration.process_inheritance_chain(self)
        // });

        for type_declaration in types.iter().rev() {
            type_declaration.compute_descendants(self);
        }

        for type_declaration in types.iter() {
            type_declaration.compute_ancestors(self);
        }

        for type_declaration in &types {
            type_declaration.process_fields(self);
        }

        for type_declaration in &types {
            type_declaration.process_method_signatures(self);
        }

        for type_declaration in &types {
            type_declaration.process_autogen_method_signatures(self);
        }

        for type_declaration in &types {
            type_declaration.process_dynamic_methods(self);
        }

        for function_declaration in &functions {
            function_declaration.process_signature(self);
        }

        for type_declaration in &types {
            type_declaration.process_fields_default_values(self);
        }

        // TYPE PARAMS CHECK START
        for type_blueprint in self.types.get_all() {
            type_blueprint.borrow().check_types_parameters(self);
        }

        for function_blueprint in self.functions.get_all() {
            function_blueprint.borrow().check_types_parameters(self);
        }

        for typedef_blueprint in self.typedefs.get_all() {
            typedef_blueprint.borrow().check_types_parameters(self);
        }
        // TYPE PARAMS CHECK END

        for global_var_declaration in &global_vars {
            global_var_declaration.process(self);
        }

        for type_declaration in &types {
            type_declaration.process_method_bodies(self);
        }

        for type_declaration in &types {
            type_declaration.process_autogen_method_bodies(self);
        }

        for function_declaration in &functions {
            function_declaration.process_body(self);
        }

        if self.functions.get_by_name("main").is_none() {
            self.errors.add_unlocated(format!("missing required function `main`"));
        }
    }

    pub fn generate_wat(mut self) -> Result<String, Vec<Error>> {
        if !self.errors.is_empty() {
            return Err(self.errors.consume());
        }

        for global_var in self.global_vars.get_all() {
            let global_var_instance = global_var.with_ref(|global_var_unwrapped| {
                global_var_unwrapped.generate_instance(&mut self)
            });

            self.global_var_instances.push(global_var_instance);
        }

        if let Some(function_wrapped) = self.functions.get_by_name("main") {
            let parameters = FunctionInstanceParameters {
                function_blueprint: function_wrapped.clone(),
                this_type: None,
                function_parameters: vec![],
            };

            let function_instance_header = self.get_function_instance(parameters);

            self.entry_function = Some(function_instance_header);
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

        for (type_name, arguments, results) in HEADER_FUNC_TYPES {
            content.push(Wat::declare_function_type(type_name, arguments, results));
        }

        content.push(wat!["table", self.dynamic_method_table.len(), "funcref"]);

        let mut elems = wat!["elem", Wat::const_i32(0)];

        for function_instance in &self.dynamic_method_table {
            let wasm_func_name = match function_instance {
                Some(header) => &header.wasm_name,
                None => DUMMY_FUNC_NAME,
            };

            elems.push(Wat::var_name(wasm_func_name));
        }

        content.push(elems);

        for (wasm_type, wasm_type_name) in self.dynamic_method_wasm_types.iter() {
            content.push(Wat::declare_function_type(&wasm_type_name, &wasm_type.arg_types, &wasm_type.return_types));
        }

        for global_var in self.global_var_instances {
            globals_declaration.push(Wat::declare_global(&global_var.wasm_name, global_var.wasm_type));
            globals_initialization.extend(global_var.init_value);
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

        for (name, args, ret, locals, body) in HEADER_FUNCTIONS {
            content.push(Wat::declare_function(name, None, args.to_vec(), ret.to_vec(), locals.to_vec(), body()))
        }

        let mut entry_function_body = vec![Wat::call(INIT_GLOBALS_FUNC_NAME, vec![])];
        entry_function_body.extend(self.entry_function.unwrap().wasm_call.clone());

        content.extend(globals_declaration);
        content.push(Wat::declare_function(INIT_GLOBALS_FUNC_NAME, None, vec![], vec![], vec![], globals_initialization));
        content.push(Wat::declare_function(ENTRY_POINT_FUNC_NAME, Some("_start"), vec![], vec![], vec![], entry_function_body));

        let placeholder_to_wasm_type = take(&mut self.placeholder_to_wasm_type);
        
        for (function_instance_header, function_instance_content) in self.function_instances.into_values() {
            if let Some(mut wasm_declaration) = function_instance_content.unwrap().wasm_declaration {
                wasm_declaration.replace_placeholder(&|placeholder| {
                    format!("${}", placeholder_to_wasm_type.get(placeholder).unwrap())
                });

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
