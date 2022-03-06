use core::panic;
use std::{collections::{HashMap, HashSet}, hash::{Hasher}, mem::{take}, rc::{Rc}, fs::{DirBuilder, File}, path::Path, io::Write};
use indexmap::{IndexMap, IndexSet};
use enum_iterator::IntoEnumIterator;
use colored::*;
use parsable::{ItemLocation, Parsable, ParseOptions, ParseError};
use crate::{items::{ParsedEventCallbackQualifierKeyword, Identifier, ParsedSourceFile, ParsedTopLevelBlock, ParsedTypeDeclaration, init_string_literal, init_color_literal}, program::{AssociatedTypeContent, DUMMY_FUNC_NAME, END_INIT_TYPE_METHOD_NAME, ENTRY_POINT_FUNC_NAME, EVENT_CALLBACKS_GLOBAL_NAME, EXPORTED_FUNCTIONS, FunctionCall, HEADER_FUNCTIONS, HEADER_FUNC_TYPES, HEADER_GLOBALS, HEADER_IMPORTS, HEADER_MEMORIES, INIT_EVENTS_FUNC_NAME, INIT_GLOBALS_FUNC_NAME, INIT_TYPES_FUNC_NAME, INIT_TYPE_METHOD_NAME, INSERT_EVENT_CALLBACK_FUNC_NAME, ItemGenerator, NamedFunctionCallDetails, RETAIN_GLOBALS_FUNC_NAME, TypeIndex, Wat, typedef_blueprint}, utils::{Link, sort_dependancy_graph, read_directory_recursively, compute_hash, FileSystemCache, PerfTimer}, wat, language_server::{CompletionItemProvider, RenameProvider, HoverProvider, SignatureHelpProvider, CompletionItemGenerator, VariableCompletionDetails, FieldCompletionDetails, MatchItemCompletionDetails, TypeCompletionDetails, EventCompletionDetails, DefinitionProvider, CodeActionsProvider, InterfaceCompletionDetails}};
use super::{ActualTypeContent, BuiltinInterface, BuiltinType, ClosureDetails, CompilationError, CompilationErrorList, DEFAULT_INTERFACES, FunctionBlueprint, FunctionInstanceContent, FunctionInstanceHeader, FunctionInstanceParameters, FunctionInstanceWasmType, GeneratedItemIndex, GlobalItemIndex, GlobalVarBlueprint, GlobalVarInstance, Id, InterfaceBlueprint, InterfaceList, MainType, ResolvedSignature, Scope, ScopeKind, SELF_VAR_NAME, Type, TypeBlueprint, TypeInstanceContent, TypeInstanceHeader, TypeInstanceParameters, TypedefBlueprint, VariableInfo, VariableKind, Vasm, SORT_EVENT_CALLBACK_FUNC_NAME, GlobalItem, SourceDirectoryDetails, SOURCE_FILE_EXTENSION, SourceFileDetails, COMMENT_START_TOKEN, insert_in_vec_hashmap, EVENT_VAR_NAME, EVENT_OUTPUT_VAR_NAME, TypeContent, CursorLocation, FunctionBody, ProgramContextOptions, Cursor, ProgramContextMode, LiteralItemManager, RETAIN_METHOD_NAME, ANONYMOUS_FUNCTION_NAME, MainTypeIndex};

pub struct ProgramContext {
    pub options: ProgramContextOptions,
    pub cursor_location: Option<ItemLocation>,
    pub source_file_list: Vec<SourceFileDetails>,
    pub parsed_source_files: Vec<Rc<ParsedSourceFile>>,
    pub errors: CompilationErrorList,

    pub default_interfaces: InterfaceList,

    pub types: GlobalItemIndex<TypeBlueprint>,
    pub typedefs: GlobalItemIndex<TypedefBlueprint>,
    pub interfaces: GlobalItemIndex<InterfaceBlueprint>,
    pub functions: GlobalItemIndex<FunctionBlueprint>,
    pub global_vars: GlobalItemIndex<GlobalVarBlueprint>,
    pub string_literals: LiteralItemManager,
    pub color_literals: LiteralItemManager,

    pub code_actions_provider: CodeActionsProvider,
    pub completion_provider: CompletionItemProvider,
    pub definition_provider: DefinitionProvider,
    pub hover_provider: HoverProvider,
    pub rename_provider: RenameProvider,
    pub signature_help_provider: SignatureHelpProvider,

    builtin_types: HashMap<BuiltinType, Link<TypeBlueprint>>,
    pub main_types: MainTypeIndex,

    pub autogen_type: Option<Link<TypeBlueprint>>,
    scopes: Vec<Scope>,
    function_level: u32,

    pub iter_fields_counter: Option<usize>,
    pub iter_variants_counter: Option<usize>,
    pub iter_ancestors_counter: Option<usize>,

    function_table: Vec<Option<Rc<FunctionInstanceHeader>>>,
    function_wasm_types: HashMap<FunctionInstanceWasmType, String>,
    type_instances: IndexMap<u64, (Rc<TypeInstanceHeader>, Option<TypeInstanceContent>)>,
    function_instances: HashMap<u64, (Rc<FunctionInstanceHeader>, Option<FunctionInstanceContent>)>,
    global_var_instances: Vec<GlobalVarInstance>,

    main_function: Option<Rc<FunctionInstanceHeader>>,
    start_client_function: Option<Rc<FunctionInstanceHeader>>,
    update_client_function: Option<Rc<FunctionInstanceHeader>>,
    start_server_function: Option<Rc<FunctionInstanceHeader>>,
    update_server_function: Option<Rc<FunctionInstanceHeader>>,

    output_wat: Wat,
    output_file: String
}

impl ProgramContext {
    pub fn new(options: ProgramContextOptions) -> Self {
        let cursor = Cursor::new(&options.cursor_location);

        Self {
            options: options.clone(),
            cursor_location: None,
            source_file_list: vec![],
            parsed_source_files: vec![],
            errors: Default::default(),
            default_interfaces: Default::default(),
            types: Default::default(),
            typedefs: Default::default(),
            interfaces: Default::default(),
            functions: Default::default(),
            global_vars: Default::default(),
            string_literals: LiteralItemManager::new("string"),
            color_literals: LiteralItemManager::new("color"),
            code_actions_provider: CodeActionsProvider::new(&cursor),
            completion_provider: CompletionItemProvider::new(&cursor),
            definition_provider: DefinitionProvider::new(&cursor),
            hover_provider: HoverProvider::new(&cursor),
            rename_provider: RenameProvider::new(&cursor),
            signature_help_provider: SignatureHelpProvider::new(&cursor),
            builtin_types: Default::default(),
            main_types: MainTypeIndex::new(),
            autogen_type: Default::default(),
            scopes: Default::default(),
            function_level: Default::default(),
            iter_fields_counter: Default::default(),
            iter_variants_counter: Default::default(),
            iter_ancestors_counter: Default::default(),
            function_table: Default::default(),
            function_wasm_types: Default::default(),
            type_instances: Default::default(),
            function_instances: Default::default(),
            global_var_instances: Default::default(),
            main_function: Default::default(),
            start_client_function: Default::default(),
            update_client_function: Default::default(),
            start_server_function: Default::default(),
            update_server_function: Default::default(),
            output_wat: Default::default(),
            output_file: Default::default(),
        }
    }

    pub fn is_new(&self) -> bool {
        self.source_file_list.is_empty()
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn take_errors(&mut self) -> Option<&[CompilationError]> {
        match self.errors.is_empty() {
            true => None,
            false => Some(self.errors.get_all()),
        }
    }

    fn index_builtin_types(&mut self) {
        for builtin_interface in DEFAULT_INTERFACES {
            let interface = self.interfaces.get_by_name(builtin_interface.get_name()).unwrap();

            self.default_interfaces.push(interface);
        }

        for builtin_type in BuiltinType::into_enum_iter() {
            let type_name = builtin_type.get_name();

            if let Some(type_wrapped) = self.types.get_by_name(type_name) {
                self.builtin_types.insert(builtin_type, type_wrapped);
            } else {
                panic!("undefined builtin type `{}`", type_name);
            }
        }

        self.string_literals.set_item_type(self.get_builtin_type(BuiltinType::String, vec![]));
        self.color_literals.set_item_type(self.get_builtin_type(BuiltinType::Color, vec![]));
    }

    pub fn get_builtin_type(&self, builtin_type: BuiltinType, parameters: Vec<Type>) -> Type {
        let type_blueprint = self.builtin_types.get(&builtin_type).unwrap();

        Type::actual(type_blueprint, parameters, &ItemLocation::default())
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

    pub fn get_main_type(&self, main_type: MainType) -> Type {
        self.main_types.get(main_type)
            .unwrap_or_else(|| panic!("main type \"{:?}\" is not defined", main_type))
            // .unwrap_or_else(|| self.get_builtin_type(BuiltinType::Object, vec![]))
    }

    pub fn get_this_type(&self) -> Type {
        if let Some(type_wrapped) = self.get_current_type() {
            type_wrapped.borrow().self_type.clone()
        } else if let Some(interface_wrapped) = self.get_current_interface() {
            Type::this(&interface_wrapped)
        } else {
            Type::undefined()
        }
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

    pub fn get_named_current_function(&self) -> Option<Link<FunctionBlueprint>> {
        for scope in self.scopes.iter().rev() {
            if let ScopeKind::Function(function_wrapped) = &scope.kind {
                if function_wrapped.borrow().name.as_str() != ANONYMOUS_FUNCTION_NAME {
                    return Some(function_wrapped.clone());
                }
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
                    Some(info) => return Some(Type::type_parameter(&info)),
                    None => {}
                },
                ScopeKind::Interface(interface_wrapped) => match interface_wrapped.borrow().associated_types.get(name) {
                    Some(info) => return Some(Type::associated(Type::this(interface_wrapped), &info)),
                    None => {},
                },
                ScopeKind::Function(function_wrapped) => match function_wrapped.borrow().parameters.get(name) {
                    Some(info) => return Some(Type::function_parameter(&info)),
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

        match kind {
            ScopeKind::Type(type_wrapped) => {
            },
            ScopeKind::Interface(interface_wrapped) => {
            },
            ScopeKind::Function(function_wrapped) => {
                self.declare_function_arguments(&function_wrapped);

            },
            _ => {}
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

        self.rename_provider.add_occurence(&var_info.name(), &var_info.name());
        self.hover_provider.set_type(&var_info.name(), &var_info.ty());
    }

    pub fn add_variable_completion_area(&mut self, location: &ItemLocation, insert_arguments: bool, expected_type: Option<&Type>) {
        let mut index = take(&mut self.completion_provider);

        index.add_completion(location, || {
            let mut available_variables = vec![];

            for scope in self.scopes.iter().rev() {
                for var_info_list in scope.variables.values() {
                    available_variables.push(var_info_list.last().unwrap().clone());
                }
            }

            let available_globals = self.global_vars.get_all_from_location(location);
            let available_functions = self.functions.get_all_from_location(location);
            let available_types = self.types.get_all_from_location(location);
            let available_typedefs = self.typedefs.get_all_from_location(location);
            let self_type = self.get_current_type();
            
            CompletionItemGenerator::Variable(VariableCompletionDetails {
                available_variables,
                available_globals,
                available_functions,
                available_types,
                available_typedefs,
                self_type,
                insert_arguments,
                expected_type: expected_type.cloned()
            })
        });

        self.completion_provider = index;
    }

    pub fn add_match_item_completion_area(&mut self, location: &ItemLocation, matched_type: &Type) {
        let mut index = take(&mut self.completion_provider);

        index.add_completion(location, || {
            let mut available_types = vec![];

            for type_wrapped in self.types.get_all_from_location(location) {
                available_types.push(type_wrapped.borrow().self_type.clone());
            }

            for typedef_wrapped in self.typedefs.get_all_from_location(location) {
                available_types.push(typedef_wrapped.borrow().target.clone());
            }

            CompletionItemGenerator::MatchItem(MatchItemCompletionDetails {
                matched_type: matched_type.clone(),
                available_types,
            })
        });

        self.completion_provider = index;
    }

    pub fn add_type_completion_area(&mut self, location: &ItemLocation) {
        let mut index = take(&mut self.completion_provider);

        index.add_completion(location, || {
            let mut available_types = vec![];

            for type_wrapped in self.types.get_all_from_location(location) {
                available_types.push(type_wrapped.borrow().self_type.clone());
            }

            for typedef_wrapped in self.typedefs.get_all_from_location(location) {
                available_types.push(typedef_wrapped.borrow().target.clone());
            }

            let self_type = self.get_current_type().map(|type_wrapped| type_wrapped.borrow().self_type.clone());

            CompletionItemGenerator::Type(TypeCompletionDetails {
                available_types,
                self_type,
            })
        });

        self.completion_provider = index
    }

    pub fn add_event_completion_area(&mut self, location: &ItemLocation, insert_brackets: bool) {
        let mut index = take(&mut self.completion_provider);

        index.add_completion(location, || {
            let mut available_events = vec![];

            for type_wrapped in self.types.get_all_from_location(location) {
                if type_wrapped.borrow().self_type.inherits_from(BuiltinType::Event.get_name()) {
                    available_events.push(type_wrapped.borrow().self_type.clone());
                }
            }

            CompletionItemGenerator::Event(EventCompletionDetails {
                available_events,
                insert_brackets,
            })
        });

        self.completion_provider = index;
    }

    pub fn add_interface_completion_area(&mut self, location: &ItemLocation) {
        let mut index = take(&mut self.completion_provider);

        index.add_completion(location, || {
            let mut available_interfaces = vec![];

            for type_wrapped in self.interfaces.get_all_from_location(location) {
                available_interfaces.push(type_wrapped.clone());
            }

            CompletionItemGenerator::Interface(InterfaceCompletionDetails {
                available_interfaces,
            })
        });

        self.completion_provider = index;
    }

    pub fn declare_local_variable(&mut self, name: Identifier, ty: Type) -> VariableInfo {
        let kind = match self.scopes.last() {
            Some(_) => VariableKind::Local,
            None => VariableKind::Global,
        };
        let suffix = self.iter_fields_counter.clone();
        let var_info = VariableInfo::create(name, ty, kind, self.get_function_level(), suffix);

        self.push_var(&var_info);

        var_info
    }

    fn declare_function_arguments(&mut self, function_wrapped: &Link<FunctionBlueprint>) {
        function_wrapped.with_mut(|mut function_unwrapped| {
            let mut variables = vec![];

            if let Some(this_type) = &function_unwrapped.signature.this_type {
                // let location = match this_type.content() {
                //     TypeContent::This(interface_blueprint) => interface_blueprint.borrow().name.location.clone(),
                //     TypeContent::Actual(content) => content.type_blueprint.borrow().name.location.clone(),
                //     _ => function_unwrapped.name.location.clone()
                // };

                let var_info = VariableInfo::create(Identifier::new(SELF_VAR_NAME, None), this_type.clone(), VariableKind::Argument, self.get_function_level(), None);

                self.push_var(&var_info);
                variables.push(var_info);
            }

            for arg_info in &function_unwrapped.arguments {
                let var_info = VariableInfo::create(arg_info.name.clone(), arg_info.ty.clone(), VariableKind::Argument, self.get_function_level(), None);

                self.push_var(&var_info);
                variables.push(var_info);
            }

            function_unwrapped.argument_variables = variables;
        });
    }

    pub fn access_var(&mut self, name: &Identifier) -> Option<VariableInfo> {
        let mut closure_access = false;
        let mut result = None;

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

                result = Some(var_info.clone());
                break;
            }

            if scope.kind.is_function() {
                closure_access = true;
            }
        }

        if result.is_none() {
            if let Some(global) = self.global_vars.get_by_identifier(name) {
                result = Some(global.borrow().var_info.clone());
            }
        }

        if let Some(var_info) = &result {
            match var_info.borrow().name.as_str() {
                SELF_VAR_NAME | EVENT_VAR_NAME => {
                    self.definition_provider.set_definition(name, &var_info.ty().get_type_blueprint().borrow().name);
                },
                _ => {
                    self.rename_provider.add_occurence(name, &var_info.name());
                    self.definition_provider.set_definition(name, &var_info.name());
                }
            };

            self.hover_provider.set_type(name, &var_info.ty());
        }

        result
    }

    pub fn get_type_instance(&mut self, parameters: TypeInstanceParameters) -> Rc<TypeInstanceHeader> {
        let id = parameters.get_id();

        if let Some((header, _)) = self.type_instances.get(&id) {
            return header.clone();
        }

        let header = TypeInstanceHeader::from_parameters(&parameters, self);

        self.type_instances.insert(id, (header.clone(), None));

        let content = TypeInstanceContent::from_parameters(&parameters, header.clone(), self);

        self.type_instances.get_mut(&id).unwrap().1 = Some(content);

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

        self.function_instances.get_mut(&id).unwrap().1 = Some(content);

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

    fn get_all_type_instances(&self) -> Vec<Rc<TypeInstanceHeader>> {
        self.type_instances.values().map(|(header, _)| header.clone()).collect()
    }

    pub fn vasm(&self) -> Vasm {
        Vasm::new(self.options.mode == ProgramContextMode::Compiler)
    }

    pub fn parse_source_files(&mut self, directories: &[SourceDirectoryDetails], provided_cache: Option<&mut FileSystemCache<ParsedSourceFile, ParseError>>) {
        for details in directories {
            let mut path_list = vec![];

            for file_path in read_directory_recursively(&details.path) {
                if let Some(extension) = file_path.extension() {
                    if extension == SOURCE_FILE_EXTENSION {
                        path_list.push(file_path.to_str().unwrap().to_string());
                    }
                }
            }

            path_list.sort();

            for file_path in path_list {
                self.source_file_list.push(SourceFileDetails {
                    file_path,
                    root_directory_path: details.path.clone(),
                });
            }
        }

        let mut empty_cache = FileSystemCache::new();
        let cache = provided_cache.unwrap_or(&mut empty_cache);

        let mut processed_count = 0;

        for source_file_details in &self.source_file_list {
            let root_directory_path = &source_file_details.root_directory_path;
            let file_path = &source_file_details.file_path;
            let parse_function = |file_content : String| {
                let parse_options = ParseOptions {
                    file_path: Some(file_path.clone()),
                    package_root_path: Some(root_directory_path.clone()),
                    comment_start: Some(COMMENT_START_TOKEN),
                };

                processed_count += 1;

                ParsedSourceFile::parse(file_content, parse_options)
            };

            let result = cache.read_file(file_path, parse_function);

            match result {
                Ok(lotus_file) => {
                    self.parsed_source_files.push(lotus_file);
                },
                Err(parse_error) => {
                    self.errors.parse_error(&parse_error);
                },
            }
        }

        // println!("files processed: {}", processed_count);
    }

    pub fn process_source_files(&mut self) {
        let mut timer = PerfTimer::new();
        let mut interfaces = vec![];
        let mut types = vec![];
        let mut typedefs = vec![];
        let mut functions = vec![];
        let mut global_vars = vec![];
        let mut main_type_declarations = vec![];

        let parsed_source_files = take(&mut self.parsed_source_files);

        for file in &parsed_source_files {
            for block in &file.blocks {
                match block {
                    ParsedTopLevelBlock::MainTypeDeclaration(main_type_declaration) => main_type_declarations.push(main_type_declaration),
                    ParsedTopLevelBlock::InterfaceDeclaration(interface_declaration) => interfaces.push(interface_declaration),
                    ParsedTopLevelBlock::TypeDeclaration(struct_declaration) => types.push(struct_declaration),
                    ParsedTopLevelBlock::TypedefDeclaration(typedef_declaration) => typedefs.push(typedef_declaration),
                    ParsedTopLevelBlock::FunctionDeclaration(function_declaration) => functions.push(function_declaration),
                    ParsedTopLevelBlock::GlobalDeclaration(global_declaration) => global_vars.push(global_declaration),
                    ParsedTopLevelBlock::InProgress(block) => block.process(self),
                }
            }
        }

        timer.trigger("interface names");
        for interface_declaration in &interfaces {
            interface_declaration.process_name(self);
        }

        timer.trigger("type names");
        for (i, type_declaration) in types.iter().enumerate() {
            type_declaration.process_name(i, self);
        }

        timer.trigger("index builtin types");
        self.index_builtin_types();

        for typedef_declaration in &typedefs {
            typedef_declaration.process(self);
        }

        timer.trigger("type dependencies");
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
                let mut wrapped : Vec<Option<&ParsedTypeDeclaration>> = types.into_iter().map(|ty| Some(ty)).collect();
                types = vec![];

                for index in result {
                    types.push(take(&mut wrapped[index]).unwrap())
                }

                // for ty in &types {
                //     println!("{}", &ty.name);
                // }
            },
            Err(cycles) => {
                for cycle in cycles {
                    let first = &types[cycle[0]];
                    let mut s = format!("{}", &first.name.as_str().bold());

                    for index in &cycle[1..] {
                        s.push_str(&format!(" -> {}", types[*index].name.as_str().bold()));
                    }

                    self.errors.generic(&first.name, format!("type dependancy cycle: {}", s));
                }

                return;
            },
        }

        // println!("====");
        // for t in &types {
        //     println!("{}", &t.name);
        // }

        timer.trigger("type parents");
        for type_declaration in &types {
            type_declaration.process_parent(self);
        }

        timer.trigger("interface associated types");
        for interface_declaration in &interfaces {
            interface_declaration.process_associated_types(self);
        }

        timer.trigger("type associated types");
        for type_declaration in &types {
            type_declaration.process_associated_types(self);
        }

        timer.trigger("interface methods");
        for interface_declaration in &interfaces {
            interface_declaration.process_method_signatures(self);
        }

        // types.sort_by_cached_key(|type_declaration| {
        //     type_declaration.process_inheritance_chain(self)
        // });

        timer.trigger("type descendants");
        for type_declaration in types.iter().rev() {
            type_declaration.compute_descendants(self);
        }

        timer.trigger("type ancestors");
        for type_declaration in types.iter() {
            type_declaration.compute_ancestors(self);
        }

        timer.trigger("main type declarations");
        for main_type_declaration in &main_type_declarations {
            main_type_declaration.process(self);
        }

        timer.trigger("type fields");
        for type_declaration in &types {
            type_declaration.process_fields(self);
        }

        timer.trigger("type method signatures");
        for type_declaration in &types {
            type_declaration.process_method_signatures(self);
        }

        timer.trigger("type autogen method signatures");
        for type_declaration in &types {
            type_declaration.process_autogen_method_signatures(self);
        }

        timer.trigger("type dynamic methods");
        for type_declaration in &types {
            type_declaration.process_dynamic_methods(self);
        }

        timer.trigger("function signatures");
        for function_declaration in &functions {
            function_declaration.process_signature(self);
        }

        timer.trigger("parameters check");
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

        timer.trigger("global variables");
        for global_var_declaration in &global_vars {
            global_var_declaration.process(self);
        }

        timer.trigger("type field default values");
        for type_declaration in &types {
            type_declaration.process_fields_default_values(self);
        }

        timer.trigger("interface method default arguments");
        for interface_declaration in &interfaces {
            interface_declaration.process_method_default_arguments(self);
        }

        timer.trigger("type method default arguments");
        for type_declaration in &types {
            type_declaration.process_method_default_arguments(self);
        }

        timer.trigger("type method autogen default arguments");
        for type_declaration in &types {
            type_declaration.process_autogen_method_default_arguments(self);
        }

        timer.trigger("function default arguments");
        for function_declaration in &functions {
            function_declaration.process_default_arguments(self);
        }

        timer.trigger("type method bodies");
        for type_declaration in &types {
            type_declaration.process_method_bodies(self);
        }

        timer.trigger("type autogen method bodies");
        for type_declaration in &types {
            type_declaration.process_autogen_method_bodies(self);
        }

        timer.trigger("function bodies");
        for function_declaration in &functions {
            function_declaration.process_body(self);
        }

        timer.trigger("type event callbacks");
        for type_declaration in &types {
            type_declaration.process_event_callbacks(self);
        }

        // timer.display();

        self.parsed_source_files = parsed_source_files;
    }

    pub fn resolve_wat(&mut self) {
        let mut content = wat!["module"];
        let mut globals_declaration = vec![];
        let mut essential_globals_initialization = vec![];
        let mut globals_initialization = vec![];
        let mut globals_retaining = vec![];
        let mut types_initialization = vec![];
        let mut events_initialization = vec![];
        let mut exports = vec![];

        for global_var in self.global_vars.get_all() {
            let global_var_instance = global_var.with_ref(|global_var_unwrapped| {
                global_var_unwrapped.generate_instance(self)
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

        let mut prev_type_instance_count = 0;

        loop {
            let type_instances = self.get_all_type_instances();
            let current_type_instance_count = type_instances.len();

            if current_type_instance_count == prev_type_instance_count {
                break;
            }

            let new_type_instances = &type_instances[prev_type_instance_count..];

            prev_type_instance_count = current_type_instance_count;

            let event_callbacks_var_info = self.global_vars.get_by_name(EVENT_CALLBACKS_GLOBAL_NAME).unwrap().borrow().var_info.clone();
            
            for type_instance in new_type_instances {
                let insert_function = self.functions.get_by_name(INSERT_EVENT_CALLBACK_FUNC_NAME).unwrap();
                let type_id = type_instance.get_type_id();

                type_instance.type_blueprint.with_ref(|type_unwrapped| {
                    // if !type_unwrapped.event_callbacks.is_empty() {
                    //     dbg!(type_instance.name.as_str());
                    // }

                    for (event_type_wrapped, event_callbacks) in type_unwrapped.event_callbacks.iter() {
                        let event_type_instance = self.get_type_instance(TypeInstanceParameters {
                            type_blueprint: event_type_wrapped.clone(),
                            type_parameters: vec![],
                        });
                        let event_type_id = event_type_instance.get_type_id();

                        // if !event_callbacks.is_empty() {
                        //     dbg!(event_type_instance.name.as_str());
                        //     dbg!(event_callbacks.len());
                        // }

                        for callback in event_callbacks.iter() {
                            let vasm = self.vasm()
                                .call_function_named(None, &insert_function, &[], vec![
                                    self.vasm().int(event_type_id),
                                    callback.borrow().get_event_callback_details().unwrap().priority.clone(),
                                    self.vasm().int(type_id),
                                    self.vasm().function_index(callback, &[])
                                ]);

                            let type_index = TypeIndex {
                                current_type_instance: Some(type_instance.clone()),
                                current_function_parameters: vec![],
                            };

                            events_initialization.extend(vasm.resolve(&type_index, self));
                        }
                    }
                });
            }
        }

        let empty_type_index = TypeIndex::empty();
        let sort_function = self.functions.get_by_name(SORT_EVENT_CALLBACK_FUNC_NAME).unwrap();

        events_initialization.extend(self.vasm()
            .call_function_named(None, &sort_function, &[], vec![])
            .resolve(&empty_type_index, self));

        for (file_namespace1, file_namespace2, func_name, arguments, return_type) in HEADER_IMPORTS {
            content.push(Wat::import_function(file_namespace1, file_namespace2, func_name, arguments.to_vec(), return_type.clone()));
        }

        for function_wrapped in self.functions.get_all() {
            function_wrapped.with_ref(|function_unwrapped| {
                if let FunctionBody::Import(first_namespace, second_namespace) = &function_unwrapped.body {
                    let function_instance = self.get_function_instance(FunctionInstanceParameters {
                        function_blueprint: function_wrapped.clone(),
                        this_type: None,
                        function_parameters: vec![],
                    });

                    let mut args = vec![];
                    let mut ret = function_unwrapped.signature.return_type.resolve(&empty_type_index, self).wasm_type;

                    for ty in &function_unwrapped.signature.argument_types {
                        if let Some(wasm_type) = ty.resolve(&empty_type_index, self).wasm_type {
                            args.push(wasm_type);
                        }
                    }

                    content.push(Wat::import_function(first_namespace, second_namespace, &function_instance.wasm_name, args, ret));
                }
            });
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

        let mut elems = wat!["elem", Wat::const_i32(0i32)];

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

        // TODO: merge literals handling
        for (string, var_info) in self.string_literals.get_all() {
            globals_declaration.push(Wat::declare_global(&var_info.wasm_name(), "i32"));
            globals_initialization.extend(self.vasm()
                .set_var(&var_info, None, init_string_literal(&string, self))
                .resolve(&empty_type_index, self)
            );
            globals_retaining.extend(self.vasm()
                .call_static_method(&var_info.ty(), RETAIN_METHOD_NAME, &[], vec![self.vasm().get_var(&var_info, None)], self)
                .resolve(&empty_type_index, self)
            );
        }

        for (string, var_info) in self.color_literals.get_all() {
            globals_declaration.push(Wat::declare_global(&var_info.wasm_name(), "i32"));
            globals_initialization.extend(self.vasm()
                .set_var(&var_info, None, init_color_literal(&string, self))
                .resolve(&empty_type_index, self)
            );
            globals_retaining.extend(self.vasm()
                .call_static_method(&var_info.ty(), RETAIN_METHOD_NAME, &[], vec![self.vasm().get_var(&var_info, None)], self)
                .resolve(&empty_type_index, self)
            );
        }

        let mut wasm_locals = vec![];

        for global_var in take(&mut self.global_var_instances) {
            globals_declaration.push(Wat::declare_global(&global_var.wasm_name, global_var.wasm_type));
            match global_var.is_essential {
                true => essential_globals_initialization.extend(global_var.init_wat),
                false => globals_initialization.extend(global_var.init_wat),
            };
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
            Wat::call(INIT_EVENTS_FUNC_NAME, vec![]),
        ];

        globals_initialization.splice(0..0, essential_globals_initialization);

        content.extend(globals_declaration);
        content.push(Wat::declare_function(INIT_GLOBALS_FUNC_NAME, None, vec![], vec![], wasm_locals, globals_initialization));
        content.push(Wat::declare_function::<&str>(RETAIN_GLOBALS_FUNC_NAME, None, vec![], vec![], vec![], globals_retaining));
        content.push(Wat::declare_function::<&str>(INIT_TYPES_FUNC_NAME, None, vec![], vec![], vec![], types_initialization));
        content.push(Wat::declare_function::<&str>(INIT_EVENTS_FUNC_NAME, None, vec![], vec![], vec![], events_initialization));
        content.push(Wat::declare_function::<&str>("initialize", Some("initialize"), vec![], vec![], vec![], initialize_function_body));

        for (function_instance_header, function_instance_content) in take(&mut self.function_instances).into_values() {
            if let Some(mut wasm_declaration) = function_instance_content.unwrap().wasm_declaration {
                content.push(wasm_declaration);
            }
        }

        content.extend(exports);

        self.output_wat = content;
    }

    pub fn generate_output_file(&mut self) {
        self.output_file = self.output_wat.to_string(0);
    }

    pub fn write_output_file(&self, output_file_path: &str) {
        let path = Path::new(output_file_path);

        if let Some(parent_dir) = path.to_path_buf().parent() {
            DirBuilder::new().recursive(true).create(parent_dir).unwrap();
        }

        let mut file = File::create(path).unwrap();

        file.write_all(self.output_file.as_bytes()).unwrap();
    }

    pub fn destroy(mut self) {
        for interface_wrapped in self.interfaces.get_all() {
            interface_wrapped.borrow_mut().destroy();
        }

        for type_wrapped in self.types.get_all() {
            type_wrapped.borrow_mut().destroy();
        }

        for typedef_wrapped in self.typedefs.get_all() {
            typedef_wrapped.borrow_mut().destroy();
        }

        for function_wrapped in self.functions.get_all() {
            function_wrapped.borrow_mut().destroy();
        }

        for global_wrapped in self.global_vars.get_all() {
            global_wrapped.borrow_mut().destroy();
        }
    }
}
