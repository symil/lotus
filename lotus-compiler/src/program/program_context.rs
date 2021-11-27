use std::{cell::UnsafeCell, collections::{HashMap, HashSet}, hash::Hash, mem::{self, take}, ops::Deref, rc::{Rc, Weak}};
use indexmap::IndexSet;
use enum_iterator::IntoEnumIterator;
use colored::*;
use parsable::{DataLocation, Parsable};
use crate::{items::{Identifier, LotusFile, TopLevelBlock, TypeDeclaration}, program::{AssociatedTypeContent, DUMMY_FUNC_NAME, END_INIT_TYPE_METHOD_NAME, ENTRY_POINT_FUNC_NAME, EXPORTED_FUNCTIONS, HEADER_FUNCTIONS, HEADER_FUNC_TYPES, HEADER_GLOBALS, HEADER_IMPORTS, HEADER_MEMORIES, INIT_GLOBALS_FUNC_NAME, INIT_TYPES_FUNC_NAME, INIT_TYPE_METHOD_NAME, ItemGenerator, RETAIN_GLOBALS_FUNC_NAME, VI, Wat, typedef_blueprint}, utils::{Link, sort_dependancy_graph}, vasm, wat};
use super::{ActualTypeContent, BuiltinInterface, BuiltinType, ClosureDetails, CompilationError, CompilationErrorList, DEFAULT_INTERFACES, FunctionBlueprint, FunctionInstanceContent, FunctionInstanceHeader, FunctionInstanceParameters, FunctionInstanceWasmType, GeneratedItemIndex, GlobalItemIndex, GlobalVarBlueprint, GlobalVarInstance, Id, InterfaceBlueprint, InterfaceList, MainType, ResolvedSignature, Scope, ScopeKind, THIS_VAR_NAME, Type, TypeBlueprint, TypeInstanceContent, TypeInstanceHeader, TypeInstanceParameters, TypedefBlueprint, VariableInfo, VariableKind, Vasm};

#[derive(Default, Debug)]
pub struct ProgramContext {
    pub errors: CompilationErrorList,

    pub default_interfaces: InterfaceList,

    pub types: GlobalItemIndex<TypeBlueprint>,
    pub typedefs: GlobalItemIndex<TypedefBlueprint>,
    pub interfaces: GlobalItemIndex<InterfaceBlueprint>,
    pub functions: GlobalItemIndex<FunctionBlueprint>,
    pub global_vars: GlobalItemIndex<GlobalVarBlueprint>,

    builtin_types: HashMap<BuiltinType, Type>,
    main_types: HashMap<MainType, Type>,

    pub autogen_type: Option<Link<TypeBlueprint>>,
    scopes: Vec<Scope>,
    function_level: u32,

    pub iter_fields_counter: Option<usize>,
    pub iter_variants_counter: Option<usize>,
    pub iter_ancestors_counter: Option<usize>,

    function_table: Vec<Option<Rc<FunctionInstanceHeader>>>,
    function_wasm_types: HashMap<FunctionInstanceWasmType, String>,
    type_instances: HashMap<u64, (Rc<TypeInstanceHeader>, Option<TypeInstanceContent>)>,
    function_instances: HashMap<u64, (Rc<FunctionInstanceHeader>, Option<FunctionInstanceContent>)>,
    global_var_instances: Vec<GlobalVarInstance>,

    main_function: Option<Rc<FunctionInstanceHeader>>,
    start_client_function: Option<Rc<FunctionInstanceHeader>>,
    update_client_function: Option<Rc<FunctionInstanceHeader>>,
    start_server_function: Option<Rc<FunctionInstanceHeader>>,
    update_server_function: Option<Rc<FunctionInstanceHeader>>,
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

        for builtin_type in BuiltinType::into_enum_iter() {
            let type_name = builtin_type.get_name();
            let type_wrapped = self.types.get_by_name(type_name).unwrap_or_else(|| panic!("undefined builtin type `{}`", type_name));
            let ty = Type::Actual(ActualTypeContent {
                type_blueprint: type_wrapped,
                parameters: vec![],
                location: DataLocation::default(),
            });

            self.builtin_types.insert(builtin_type, ty);
        }

        for main_type in MainType::into_enum_iter() {
            let type_name = main_type.get_name();
            let default_name = main_type.get_default_name();
            let type_wrapped = self.types.get_by_name(type_name).unwrap_or_else(|| self.types.get_by_name(default_name).unwrap());
            let ty = Type::Actual(ActualTypeContent {
                type_blueprint: type_wrapped,
                parameters: vec![],
                location: DataLocation::default(),
            });

            self.main_types.insert(main_type, ty);
        }
    }

    pub fn get_builtin_type(&self, builtin_type: BuiltinType, parameters: Vec<Type>) -> Type {
        let mut ty = self.builtin_types.get(&builtin_type).unwrap().clone();

        ty.push_parameters(parameters);

        ty
    }

    pub fn get_main_type(&self, main_type: MainType) -> Type {
        self.main_types.get(&main_type).unwrap().clone()
    }

    pub fn get_this_type(&self) -> Type {
        if let Some(type_wrapped) = self.get_current_type() {
            type_wrapped.borrow().self_type.clone()
        } else if let Some(interface_wrapped) = self.get_current_interface() {
            Type::This(interface_wrapped.clone())
        } else {
            Type::Undefined
        }
    }

    pub fn void_type(&self) -> Type {
        self.get_builtin_type(BuiltinType::Void, vec![])
    }

    pub fn bool_type(&self) -> Type {
        self.get_builtin_type(BuiltinType::Bool, vec![])
    }

    pub fn int_type(&self) -> Type {
        self.get_builtin_type(BuiltinType::Int, vec![])
    }

    pub fn float_type(&self) -> Type {
        self.get_builtin_type(BuiltinType::Float, vec![])
    }

    pub fn function_type(&self) -> Type {
        self.get_builtin_type(BuiltinType::Function, vec![])
    }
    
    pub fn pointer_type(&self) -> Type {
        self.get_builtin_type(BuiltinType::Pointer, vec![self.int_type()])
    }

    pub fn display_size_type(&self) -> Type {
        self.get_builtin_type(BuiltinType::DisplaySize, vec![])
    }

    pub fn get_builtin_interface(&self, interface: BuiltinInterface) -> Link<InterfaceBlueprint> {
        let interface_name = interface.get_name();
        let interface_blueprint = self.interfaces.get_by_name(interface_name).unwrap_or_else(|| panic!("undefined builtin interface `{}`", interface_name));

        interface_blueprint
    }

    pub fn get_current_function(&self) -> Option<Link<FunctionBlueprint>> {
        for scope in self.scopes.iter().rev() {
            if let ScopeKind::Function(function_wrapped) = &scope.kind {
                return Some(function_wrapped.clone());
            }
        }

        None
    }

    pub fn get_current_type(&self) -> Option<Link<TypeBlueprint>> {
        for scope in self.scopes.iter().rev() {
            if let ScopeKind::Type(type_wrapped) = &scope.kind {
                return Some(type_wrapped.clone());
            }
        }

        None
    }

    pub fn get_current_interface(&self) -> Option<Link<InterfaceBlueprint>> {
        for scope in self.scopes.iter().rev() {
            if let ScopeKind::Interface(interface_wrapped) = &scope.kind {
                return Some(interface_wrapped.clone());
            }
        }

        None
    }

    pub fn get_type_parameter(&self, name: &str) -> Option<Type> {
        for scope in self.scopes.iter().rev() {
            match &scope.kind {
                ScopeKind::Type(type_wrapped) => match type_wrapped.borrow().parameters.get(name) {
                    Some(info) => return Some(Type::TypeParameter(info.clone())),
                    None => match type_wrapped.borrow().associated_types.get(name) {
                        Some(info) => return Some(info.ty.clone()),
                        None => {},
                    },
                },
                ScopeKind::Interface(interface_wrapped) => match interface_wrapped.borrow().associated_types.get(name) {
                    Some(info) => return Some(Type::Associated(AssociatedTypeContent {
                        root: Box::new(Type::This(interface_wrapped.clone())),
                        associated: info.clone(),
                    })),
                    None => {},
                },
                ScopeKind::Function(function_wrapped) => match function_wrapped.borrow().parameters.get(name) {
                    Some(info) => return Some(Type::FunctionParameter(info.clone())),
                    None => {},
                },
                ScopeKind::Loop => {},
                ScopeKind::Branch => {},
                ScopeKind::Block => {},
            }
        }

        None
    }

    pub fn get_current_function_return_type(&self) -> Option<Type> {
        match self.get_current_function() {
            Some(function_wrapped) => Some(function_wrapped.borrow().signature.return_type.clone()),
            None => None,
        }
    }

    pub fn push_scope(&mut self, kind: ScopeKind) {
        if kind.is_function() {
            self.function_level += 1;
        }
        
        self.scopes.push(Scope::new(kind.clone()));

        if let ScopeKind::Function(function_wrapped) = kind {
            self.declare_function_arguments(&function_wrapped);
        }
    }

    pub fn pop_scope(&mut self) {
        if let Some(scope) = self.scopes.pop() {
            if scope.kind.is_function() {
                self.function_level -= 1;
            }
        }
    }

    pub fn get_function_level(&self) -> u32 {
        self.function_level
    }

    pub fn get_scope_depth(&self, kind: ScopeKind) -> Option<u32> {
        let mut result = 0;

        for scope in self.scopes.iter().rev() {
            if scope.kind == kind {
                return Some(result);
            }

            result += scope.kind.get_depth();

            if scope.kind.is_function() {
                break;
            }
        }

        None
    }

    fn push_var(&mut self, var_info: &VariableInfo) {
        // global scope is handled differently
        if let Some(current_scope) = self.scopes.iter_mut().last() {
            current_scope.insert_var_info(&var_info);
        }
    }

    pub fn declare_local_variable(&mut self, name: Identifier, ty: Type) -> VariableInfo {
        let kind = match self.scopes.last() {
            Some(_) => VariableKind::Local,
            None => VariableKind::Global,
        };
        let var_info = VariableInfo::create(name, ty, kind, self.get_function_level());

        self.push_var(&var_info);

        var_info
    }

    fn declare_function_arguments(&mut self, function_wrapped: &Link<FunctionBlueprint>) {
        function_wrapped.with_mut(|mut function_unwrapped| {
            let mut variables = vec![];

            if let Some(this_type) = &function_unwrapped.signature.this_type {
                let var_info = VariableInfo::create(Identifier::unlocated(THIS_VAR_NAME), this_type.clone(), VariableKind::Argument, self.get_function_level());

                self.push_var(&var_info);
                variables.push(var_info);
            }

            for (arg_name, arg_type) in function_unwrapped.argument_names.iter().zip(function_unwrapped.signature.argument_types.iter()) {
                let var_info = VariableInfo::create(arg_name.clone(), arg_type.clone(), VariableKind::Argument, self.get_function_level());

                self.push_var(&var_info);
                variables.push(var_info);
            }

            function_unwrapped.argument_variables = variables;
        });
    }

    pub fn access_var(&self, name: &Identifier) -> Option<VariableInfo> {
        let mut closure_access = false;

        for scope in self.scopes.iter().rev() {
            if let Some(var_info) = scope.get_var_info(name.as_str()) {
                if closure_access {
                    var_info.mark_as_closure_arg();

                    if var_info.borrow().declaration_level != self.function_level {
                        self.get_current_function().unwrap().with_mut(|mut function_unwrapped| {
                            let mut closure_details = function_unwrapped.closure_details.get_or_insert_with(|| ClosureDetails {
                                variables: HashSet::new(),
                                declaration_level: self.function_level,
                                retain_function: None,
                            });

                            closure_details.variables.insert(var_info.clone());
                        });
                    }
                }

                return Some(var_info.clone());
            }

            if scope.kind.is_function() {
                closure_access = true;
            }
        }

        match self.global_vars.get_by_identifier(name) {
            Some(global) => Some(global.borrow().var_info.clone()),
            None => None,
        }
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

    pub fn get_function_instance_wasm_type_name(&mut self, signature: &ResolvedSignature) -> String {
        let mut function_wasm_type = FunctionInstanceWasmType {
            arg_types: vec![],
            return_types: vec![],
        };

        if let Some(ty) = &signature.this_type {
            if let Some(wasm_type) = ty.wasm_type {
                function_wasm_type.arg_types.push(wasm_type);
            }
        }

        for ty in &signature.argument_types {
            if let Some(wasm_type) = ty.wasm_type {
                function_wasm_type.arg_types.push(wasm_type);
            }
        }

        if let Some(wasm_type) = signature.return_type.wasm_type {
            function_wasm_type.return_types.push(wasm_type);
        }

        match self.function_wasm_types.get(&function_wasm_type) {
            Some(name) => name.clone(),
            None => {
                let name = format!("type_{}", self.function_wasm_types.len() + 1);

                self.function_wasm_types.insert(function_wasm_type, name.clone());

                name
            },
        }
    }

    fn get_exported_function_instance(&mut self, name: &str) -> Option<Rc<FunctionInstanceHeader>> {
        match self.functions.get_by_name(name) {
            Some(function_wrapped) => {
                let parameters = FunctionInstanceParameters {
                    function_blueprint: function_wrapped.clone(),
                    this_type: None,
                    function_parameters: vec![],
                };

                Some(self.get_function_instance(parameters))
            },
            None => None,
        }
    }

    pub fn reserve_next_function_index(&mut self) -> usize {
        let result = self.function_table.len();
        self.function_table.push(None);

        result
    }

    pub fn assign_function_to_index(&mut self, index: usize, function_instance: &Rc<FunctionInstanceHeader>) {
        self.function_table[index] = Some(function_instance.clone());
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

        for typedef_declaration in &typedefs {
            typedef_declaration.process(self);
        }

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

                    self.errors.add_generic(&first.name, format!("type dependancy cycle: {}", s));
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
            function_blueprint.borrow().check_type_parameters(self);
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
            self.errors.add_generic_unlocated(format!("missing required function `main`"));
        }
    }

    pub fn generate_wat(mut self) -> Result<String, Vec<CompilationError>> {
        if !self.errors.is_empty() {
            return Err(self.errors.consume());
        }

        let mut content = wat!["module"];
        let mut globals_declaration = vec![];
        let mut globals_initialization = vec![];
        let mut globals_retaining = vec![];
        let mut types_initialization = vec![];
        let mut exports = vec![];

        for global_var in self.global_vars.get_all() {
            let global_var_instance = global_var.with_ref(|global_var_unwrapped| {
                global_var_unwrapped.generate_instance(&mut self)
            });

            self.global_var_instances.push(global_var_instance);
        }

        for func_name in EXPORTED_FUNCTIONS {
            if let Some(function_instance) = self.get_exported_function_instance(func_name) {
                exports.push(wat!["export", Wat::string(func_name), wat!["func", Wat::var_name(&function_instance.wasm_name)]]);
            }
        }

        if let Some(function_wrapped) = self.functions.get_by_name("main") {
            let parameters = FunctionInstanceParameters {
                function_blueprint: function_wrapped.clone(),
                this_type: None,
                function_parameters: vec![],
            };

            let function_instance_header = self.get_function_instance(parameters);

            self.main_function = Some(function_instance_header);
        }

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

        content.push(wat!["table", self.function_table.len(), "funcref"]);

        let mut elems = wat!["elem", Wat::const_i32(0)];

        for function_instance in &self.function_table {
            let wasm_func_name = match function_instance {
                Some(header) => &header.wasm_name,
                None => DUMMY_FUNC_NAME,
            };

            elems.push(Wat::var_name(wasm_func_name));
        }

        content.push(elems);

        for (wasm_type, wasm_type_name) in self.function_wasm_types.iter() {
            content.push(Wat::declare_function_type(&wasm_type_name, &wasm_type.arg_types, &wasm_type.return_types));
        }

        let type_headers : Vec<Rc<TypeInstanceHeader>> = self.type_instances.values().map(|(header, _)| header.clone()).collect();

        for type_instance_header in &type_headers {
            type_instance_header.type_blueprint.with_ref(|type_unwrapped| {
                if let Some(func_ref) = type_unwrapped.static_methods.get(INIT_TYPE_METHOD_NAME) {
                    let parameters = FunctionInstanceParameters {
                        function_blueprint: func_ref.function.clone(),
                        this_type: Some(type_instance_header.clone()),
                        function_parameters: vec![],
                    };

                    let function_instance = self.get_function_instance(parameters);

                    types_initialization.extend_from_slice(&function_instance.wasm_call);
                }
            });
        }

        for type_instance_header in &type_headers {
            type_instance_header.type_blueprint.with_ref(|type_unwrapped| {
                if let Some(func_ref) = type_unwrapped.static_methods.get(END_INIT_TYPE_METHOD_NAME) {
                    let parameters = FunctionInstanceParameters {
                        function_blueprint: func_ref.function.clone(),
                        this_type: Some(type_instance_header.clone()),
                        function_parameters: vec![],
                    };

                    let function_instance = self.get_function_instance(parameters);

                    types_initialization.extend_from_slice(&function_instance.wasm_call);
                }
            });
        }

        for (wasm_name, wasm_type) in HEADER_GLOBALS {
            globals_declaration.push(Wat::declare_global(wasm_name, wasm_type));
        }

        let mut wasm_locals = vec![];

        for global_var in self.global_var_instances {
            globals_declaration.push(Wat::declare_global(&global_var.wasm_name, global_var.wasm_type));
            globals_initialization.extend(global_var.init_wat);
            globals_retaining.extend(global_var.retain_wat);
            wasm_locals.extend(global_var.wasm_locals);
        }

        let wasm_locals : Vec<(&str, &str)> = wasm_locals.iter().map(|(ty, name)| (name.as_str(), ty.clone())).collect();

        for (name, args, ret, locals, body) in HEADER_FUNCTIONS {
            content.push(Wat::declare_function(name, None, args.to_vec(), ret.to_vec(), locals.to_vec(), body()))
        }

        let mut initialize_function_body = vec![
            Wat::call(INIT_GLOBALS_FUNC_NAME, vec![]),
            Wat::call(INIT_TYPES_FUNC_NAME, vec![]),
        ];

        content.extend(globals_declaration);
        content.push(Wat::declare_function(INIT_GLOBALS_FUNC_NAME, None, vec![], vec![], wasm_locals, globals_initialization));
        content.push(Wat::declare_function::<&str>(RETAIN_GLOBALS_FUNC_NAME, None, vec![], vec![], vec![], globals_retaining));
        content.push(Wat::declare_function::<&str>(INIT_TYPES_FUNC_NAME, None, vec![], vec![], vec![], types_initialization));
        content.push(Wat::declare_function::<&str>("initialize", Some("initialize"), vec![], vec![], vec![], initialize_function_body));

        for (function_instance_header, function_instance_content) in self.function_instances.into_values() {
            if let Some(mut wasm_declaration) = function_instance_content.unwrap().wasm_declaration {
                content.push(wasm_declaration);
            }
        }

        content.extend(exports);
        
        Ok(content.to_string(0))
    }
}
