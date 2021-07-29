use std::{collections::{HashMap, HashSet}};

use crate::{items::{expression::{Expression, Operand, Operation, PathSegment, VarPath, VarPrefix}, file::LotusFile, function_declaration::{FunctionDeclaration, FunctionSignature}, identifier::Identifier, statement::{VarDeclaration, VarDeclarationQualifier}, struct_declaration::{MethodDeclaration, MethodQualifier, StructDeclaration, StructQualifier, Type}, top_level_block::TopLevelBlock}, program::{builtin_methods::{get_array_field_type, get_builtin_field_type}, expression_type::ItemType, struct_annotation::{StructAnnotation}, utils::display_join}};
use super::{error::Error, expression_type::{BuiltinType, ExpressionType}, function_annotation::FunctionAnnotation, binary_operations::{OperationTree, get_operator_valid_types}, program_context::ProgramContext, struct_annotation::FieldDetails};

const KEYWORDS : &'static[&'static str] = &[
    "let", "const", "struct", "view", "entity", "event", "world", "user", "true", "false"
];

#[derive(Default)]
pub struct ProgramIndex {
    pub world_type_name: Option<Identifier>,
    pub user_type_name: Option<Identifier>,
    pub struct_declarations: HashMap<Identifier, StructDeclaration>,
    pub function_declarations: HashMap<Identifier, FunctionDeclaration>,
    pub const_declarations: HashMap<Identifier, VarDeclaration>,
}

impl ProgramIndex {
    pub fn from_parsed_files(files: Vec<LotusFile>) -> Result<Self, Vec<Error>> {
        let mut index = Self::default();
        let mut context = ProgramContext::new();

        index.build_index(files, &mut context);
        index.process_structs_fields(&mut context);
        index.process_structs_method_signature(&mut context);
        index.process_functions_signatures(&mut context);
        index.process_constants(&mut context);
        index.process_function_bodies(&mut context);

        match context.errors.is_empty() {
            true => Ok(index),
            false => Err(context.errors)
        }
    }

    fn build_index(&mut self, files: Vec<LotusFile>, context: &mut ProgramContext) {
        for file in files {
            for block in file.blocks {
                let identifier = match &block {
                    TopLevelBlock::StructDeclaration(struct_declaration) => &struct_declaration.name,
                    TopLevelBlock::ConstDeclaration(const_declaration) => &const_declaration.name,
                    TopLevelBlock::FunctionDeclaration(function_declaration) => &function_declaration.name,
                }.clone();

                if self.struct_declarations.contains_key(&identifier) || self.function_declarations.contains_key(&identifier) || self.const_declarations.contains_key(&identifier) {
                    context.error(&identifier, format!("duplicate declaration: {}", identifier));
                }

                match block {
                    TopLevelBlock::StructDeclaration(struct_declaration) => {
                        context.structs.insert(identifier.clone(), StructAnnotation::new(&identifier, &struct_declaration.qualifier));
                        self.struct_declarations.insert(identifier, struct_declaration);
                    },
                    TopLevelBlock::ConstDeclaration(var_declaration) => {
                        context.constants.insert(identifier.clone(), ExpressionType::Void);
                        self.const_declarations.insert(identifier, var_declaration);
                    },
                    TopLevelBlock::FunctionDeclaration(def) => {
                        context.functions.insert(identifier.clone(), FunctionAnnotation::new(&identifier));
                        self.function_declarations.insert(identifier, def);
                    },
                }
            }
        }

        let world_structs : Vec<Identifier> = self.struct_declarations.values().filter(|s| s.qualifier == StructQualifier::World).map(|s| s.name.clone()).collect();
        let user_structs : Vec<Identifier> = self.struct_declarations.values().filter(|s| s.qualifier == StructQualifier::User).map(|s| s.name.clone()).collect();

        if world_structs.len() > 1 {
            for name in &world_structs {
                context.error(name, "multiple worlds declared");
            }
        }

        if let Some(name) = world_structs.first() {
            self.world_type_name = Some(name.clone());
        }

        if user_structs.len() > 1 {
            for name in &user_structs {
                context.error(name, "multiple users declared");
            }
        }

        if let Some(name) = user_structs.first() {
            self.user_type_name = Some(name.clone());
        }
    }

    fn process_structs_fields(&mut self, context: &mut ProgramContext) {
        for struct_declaration in self.struct_declarations.values() {
            if self.is_forbidden_identifier(&struct_declaration.name) {
                context.error(struct_declaration, format!("invalid type name: {}", &struct_declaration.name));
            } else {
                let struct_types = self.collect_struct_types(&struct_declaration.name, vec![], context);
                let struct_fields = self.collect_struct_fields(&struct_types, context);
                let struct_annotation = context.structs.get_mut(&struct_declaration.name).unwrap();

                struct_annotation.types = struct_types;
                struct_annotation.fields = struct_fields;
            }
        }
    }

    fn process_structs_method_signature(&mut self, context: &mut ProgramContext) {
        for struct_declaration in self.struct_declarations.values() {
            for method in &struct_declaration.body.methods {
                match method.qualifier {
                    Some(MethodQualifier::Builtin) => {
                        match method.name.as_str() {
                            "on_user_connect" | "on_user_disconnect" => {
                                if struct_declaration.qualifier != StructQualifier::World {
                                    context.error(&method.name, format!("method @{} can only be implemented on a world", &method.name));
                                }

                                self.check_builtin_method(method, context);
                            },
                            "trigger" => {
                                if struct_declaration.qualifier != StructQualifier::Event && struct_declaration.qualifier != StructQualifier::Request {
                                    context.error(&method.name, format!("method @{} can only be implemented on a events and requests", &method.name));
                                }

                                self.check_builtin_method(method, context);
                            },
                            _ => {
                                context.error(method, format!("invalid built-in method name @{}", &method.name));
                            }
                        }
                    },
                    Some(MethodQualifier::Hook | MethodQualifier::Before | MethodQualifier::After) => {
                        if !self.is_entity_qualifier(&struct_declaration.qualifier) {
                            context.error(method, "event callbacks can only be defined on an entity, world or user");
                        }

                        self.check_struct_qualifier(&method.name, StructQualifier::Event, context);

                        for condition in &method.conditions {
                            if let Some(VarPrefix::Payload) = &condition.left.prefix {
                                // ok
                            } else {
                                context.error(&condition.left, "left-hand side of event callback condition must be prefixed by $");
                            }

                            let event_struct_annotation = context.structs.get(&method.name).unwrap();

                            if let Some(field) = event_struct_annotation.fields.get(&condition.left.name) {
                                let mut ok = false;

                                if let ExpressionType::Single(ItemType::Struct(struct_name)) = &field.expr_type {
                                    let field_type = context.structs.get(struct_name).unwrap();

                                    if self.is_entity_qualifier(&field_type.qualifier) {
                                        ok = true;
                                    }
                                }

                                if !ok {
                                    context.error(&condition.left.name, format!("event callback condition: left-side must refer to an entity field"));
                                }
                            } else {
                                context.error(&condition.left.name, format!("event `{}` does not have a `{}` field", &method.name, &condition.left.name));
                            }

                            if !condition.left.path.is_empty() {
                                context.error(&condition.left.path[0], format!("paths are not supported on event callback conditions"));
                            }

                            if let Some(var_path) = &condition.right {
                                if let Some(VarPrefix::This) = var_path.prefix {
                                    // ok
                                } else {
                                    context.error(&condition.right, "right-hand side of event callback condition must be prefixed by #");
                                }

                                let this_struct_annotation = context.structs.get(&struct_declaration.name).unwrap();

                                if let Some(field) = this_struct_annotation.fields.get(&var_path.name) {
                                    let mut ok = false;

                                    if let ExpressionType::Single(ItemType::Struct(struct_name)) = &field.expr_type {
                                        let field_type = context.structs.get(struct_name).unwrap();

                                        if self.is_entity_qualifier(&field_type.qualifier) {
                                            ok = true;
                                        }
                                    }

                                    if !ok {
                                        context.error(&var_path.name, format!("event callback condition: right-side must refer to an entity field"));
                                    }
                                } else {
                                    context.error(&var_path.name, format!("entity `{}` does not have a `{}` field", &struct_declaration.name, &var_path.name));
                                }

                                if !var_path.path.is_empty() {
                                    context.error(&var_path.path[0], format!("paths are not supported on event callback conditions"));
                                }
                            } else {
                                context.error(&condition.left, "right-hand side of event callback condition must not be empty");
                            }
                        }

                        if let Some(signature) = &method.signature {
                            context.error(signature, "event callbacks do not take arguments nor have a return type");
                        }

                        // no need to check for name unicity, multiple event callbacks on the same struct are allowed
                    },
                    None => {
                        let mut method_annotation = FunctionAnnotation::new(&method.name);

                        if !method.conditions.is_empty() {
                            context.error(&method.conditions[0], format!("only event callbacks can have conditions"));
                        }

                        if let Some(signature) = &method.signature {
                            let (arguments, return_type) = self.process_function_signature(signature, context);

                            method_annotation.arguments = arguments;
                            method_annotation.return_type = return_type;
                        } else {
                            context.error(&method.name, format!("missing method arguments"));
                        }

                        let struct_annotation = context.structs.get_mut(&struct_declaration.name).unwrap();
                        let field_exists = struct_annotation.fields.contains_key(&method.name);
                        let method_exists = struct_annotation.methods.contains_key(&method.name);

                        if !field_exists && !method_exists {
                            struct_annotation.methods.insert(method.name.clone(), method_annotation);
                        }

                        if field_exists {
                            context.error(&method.name, format!("duplicate method declaration: field `{}` already exists", &method.name));
                        }

                        if method_exists {
                            context.error(&method.name, format!("duplicate method declaration: method `{}` already exists", &method.name));
                        }
                    },
                }
            }
        }
    }

    fn process_functions_signatures(&mut self, context: &mut ProgramContext) {
        for function_declaration in self.function_declarations.values() {
            let (arguments, return_type) = self.process_function_signature(&function_declaration.signature, context);
            let function_annotation = context.functions.get_mut(&function_declaration.name).unwrap();

            function_annotation.arguments = arguments;
            function_annotation.return_type = return_type;
        }
    }

    fn process_constants(&mut self, context: &mut ProgramContext) {
        context.inside_const_expr = true;

        for const_declaration in self.const_declarations.values() {
            if const_declaration.qualifier != VarDeclarationQualifier::Const {
                context.error(const_declaration, "global variables must be declared with the `const` qualifier");
            }

            if let Some(expr_type) = self.get_expression_type(&const_declaration.value, context) {
                *context.constants.get_mut(&const_declaration.name).unwrap() = expr_type;
            }
        }

        context.inside_const_expr = false;
    }

    fn process_function_bodies(&mut self, context: &mut ProgramContext) {
        // TODO: functions
    }

    fn process_function_signature(&self, signature: &FunctionSignature, context: &mut ProgramContext) -> (Vec<ExpressionType>, ExpressionType) {
        let mut arg_names = HashSet::new();
        let mut arguments = vec![];
        let mut return_type = ExpressionType::Void;

        for argument in &signature.arguments {
            if !arg_names.insert(&argument.name) {
                context.error(&argument.name, format!("duplicate argument: {}", &argument.name));
            }

            let arg_type = match self.check_type_name(&argument.type_.name, context) {
                true => ExpressionType::from_value_type(&argument.type_),
                false => ExpressionType::Void
            };

            arguments.push(arg_type);
        }

        if let Some(return_type_parsed) = &signature.return_type {
            if self.check_type_name(&return_type_parsed.name, context) {
                return_type = ExpressionType::from_value_type(return_type_parsed);
            }
        }

        (arguments, return_type)
    }

    fn get_expression_type(&self, expr: &Expression, context: &mut ProgramContext) -> Option<ExpressionType> {
        self.get_operation_type(expr, context)
    }

    fn get_operation_type(&self, operation: &Operation, context: &mut ProgramContext) -> Option<ExpressionType> {
        let operation_tree = OperationTree::from_operation(operation);

        self.get_operation_tree_type(&operation_tree, context)
    }

    fn get_operation_tree_type(&self, operation_tree: &OperationTree, context: &mut ProgramContext) -> Option<ExpressionType> {
        match operation_tree {
            OperationTree::Operation(left, operator, right) => {
                let left_type = self.get_operation_tree_type(left, context);
                let right_type = self.get_operation_tree_type(right, context);

                match (left_type, right_type) {
                    (Some(ltype), Some(rtype)) => {
                        let operator_valid_types = get_operator_valid_types(operator);

                        let left_ok = operator_valid_types.iter().any(|expected| expected.match_actual(&ltype, &mut HashMap::new()));
                        let right_ok = operator_valid_types.iter().any(|expected| expected.match_actual(&rtype, &mut HashMap::new()));

                        if !left_ok {
                            context.error(left.get_leftmost(), format!("operator `{}` left operand: expected {}, got `{}`", operator, display_join(&operator_valid_types), ltype));
                        }

                        if !right_ok {
                            context.error(left.get_leftmost(), format!("operator `{}`, right operand: expected {}, got `{}`", operator, display_join(&operator_valid_types), rtype));
                        }

                        if left_ok && right_ok && ltype != rtype {
                            context.error(left.get_leftmost(), format!("operator `{}`: operand types must match (got `{}` and `{}`)", operator, ltype, rtype));
                        }
                        None
                    },
                    _ => None
                }
            },
            OperationTree::Value(operand) => self.get_operand_type(operand, context),
        }
    }

    fn get_operand_type(&self, operand: &Operand, context: &mut ProgramContext) -> Option<ExpressionType> {
        match operand {
            Operand::NullLiteral => Some(ExpressionType::Anonymous(0)),
            Operand::BooleanLiteral(_) => Some(ExpressionType::single_builtin(BuiltinType::Boolean)),
            Operand::NumberLiteral(_) => Some(ExpressionType::single_builtin(BuiltinType::Number)),
            Operand::StringLiteral(_) => Some(ExpressionType::single_builtin(BuiltinType::String)),
            Operand::ArrayLiteral(_) => todo!(),
            Operand::Parenthesized(expr) => self.get_expression_type(expr, context),
            Operand::UnaryOperation(_) => todo!(),
            Operand::VarPath(var_path) => self.get_var_path_type(var_path, context),
        }
    }

    fn get_field_access_type(&self, parent_type: &ExpressionType, field_name: &Identifier, context: &mut ProgramContext) -> Option<ExpressionType> {
        let result = match parent_type {
            ExpressionType::Void => None,
            ExpressionType::Single(item_type) => match item_type {
                ItemType::Builtin(builtin_type) => get_builtin_field_type(builtin_type, field_name),
                ItemType::Struct(struct_name) => {
                    if struct_name.is("_") {
                        // special case: `_` refers to the value itself rather than a field
                        // e.g `#foo` means `self.foo`, but `#_` means `self`
                        Some(parent_type.clone())
                    } else if let Some(struct_annotation) = context.structs.get(struct_name) {
                        if let Some(field) = struct_annotation.fields.get(field_name) {
                            Some(field.get_expr_type())
                        } else if let Some(method) = struct_annotation.methods.get(field_name) {
                            Some(method.get_expr_type())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                },
                ItemType::Function(_, _) => None,
            },
            ExpressionType::Array(item_type) => get_array_field_type(item_type, field_name),
            ExpressionType::Anonymous(_) => None,
            
        };

        if result.is_none() {
            context.error(field_name, format!("type `{}` has no field `{}`", parent_type, field_name));
        }

        result
    }

    fn get_var_path_type(&self, var_path: &VarPath, context: &mut ProgramContext) -> Option<ExpressionType> {
        let var_type : Option<ExpressionType> = match &var_path.prefix {
            Some(prefix) => {
                let prefix_type_opt = match prefix {
                    VarPrefix::This => {
                        if context.get_this_type().is_none() {
                            context.error(prefix, "no `this` value can be referenced in this context");
                        }

                        context.get_this_type()
                    },
                    VarPrefix::Payload => {
                        if context.get_payload_type().is_none() {
                            context.error(prefix, "no `payload` value can be referenced in this context");
                        }

                        context.get_payload_type()
                    },
                };

                if let Some(prefix_type) = &prefix_type_opt {
                    self.get_field_access_type(prefix_type, &var_path.name, context)
                } else {
                    None
                }
            },
            None => {
                if context.inside_const_expr {
                    if let Some(referenced_const) = self.const_declarations.get(&var_path.name) {
                        if let Some(_) = context.visit_constant(&var_path.name) {
                            context.error(&referenced_const.name, format!("circular reference to `{}`", &referenced_const.name));

                            None
                        } else {
                            self.get_expression_type(&referenced_const.value, context)
                        }
                    } else {
                        context.error(&var_path.name, format!("undefined constant `{}`", &var_path.name));
                        None
                    }
                } else {
                    context.get_var_type(&var_path.name).cloned()
                }
            }
        };

        if context.inside_const_expr && !var_path.path.is_empty() {
            context.error(&var_path.path[0], "field paths are not supported in const expressions");

            None
        } else if let Some(expr_type) = var_type {
            let mut final_type = expr_type.clone();

            for segment in &var_path.path {
                let next_type : Option<ExpressionType> = match segment {
                    PathSegment::FieldAccess(field_name) => self.get_field_access_type(&final_type, field_name, context),
                    PathSegment::BracketIndexing(expr) => {
                        let array_item_type = match final_type {
                            ExpressionType::Array(item_type) => Some(*item_type),
                            _ => {
                                context.error(expr, format!("bracket indexing target: expected array, got `{}`", final_type)); // TODO: display actual type
                                None
                            }
                        };

                        let indexing_ok = match self.get_expression_type(expr, context) {
                            Some(expr_type) => {
                                if let ExpressionType::Single(ItemType::Builtin(BuiltinType::Number)) = &expr_type {
                                    true
                                } else {
                                    context.error(expr, format!("bracket indexing argument: expected `{}`, got `{}`", BuiltinType::Number, &expr_type));
                                    false
                                }
                            },
                            None => false,
                        };

                        match indexing_ok {
                            true => array_item_type,
                            false => None
                        }
                    },
                    PathSegment::FunctionCall(arguments) => {
                        match final_type {
                            ExpressionType::Single(ItemType::Function(expected_arguments, return_type)) => {
                                if arguments.len() != expected_arguments.len() {
                                    context.error(arguments, format!("function call arguments: expected {} arguments, got `{}`", expected_arguments.len(), arguments.len()));
                                }

                                let mut ok = false;
                                let mut anonymous_types = HashMap::new();

                                for (i, (arg_expr, expected_type)) in arguments.iter().zip(expected_arguments.iter()).enumerate() {
                                    if let Some(actual_type) = self.get_expression_type(arg_expr, context) {
                                        if !expected_type.match_actual(&actual_type, &mut anonymous_types) {
                                            context.error(arg_expr, format!("function call argument #{}: expected `{}`, got `{}`", i, expected_type, actual_type));
                                            ok = false;
                                        }
                                    }
                                }

                                match ok {
                                    true => Some(*return_type),
                                    false => None
                                }
                            }
                            _ => {
                                context.error(arguments, format!("function call target: expected function, got `{}`", final_type));
                                None
                            }
                        }
                    },
                };

                if let Some(t) = next_type {
                    final_type = t;
                } else {
                    return None;
                }
            }

            Some(final_type)
        } else {
            None
        }
    }

    fn collect_struct_types(&self, struct_name: &Identifier, mut types: Vec<Identifier>, context: &mut ProgramContext) -> Vec<Identifier> {
        if types.contains(struct_name) {
            context.error(struct_name, format!("circular inheritance: {}", struct_name));
        } else {
            types.push(struct_name.clone());

            let struct_def = self.struct_declarations.get(&struct_name).unwrap();

            if let Some(parent_name) = &struct_def.parent {
                if let Some(parent) = self.struct_declarations.get(parent_name) {
                    if parent.qualifier != struct_def.qualifier {
                        context.error(parent_name, format!("a {} cannot inherit from a {}", struct_def.qualifier, parent.qualifier));
                    } else {
                        types = self.collect_struct_types(parent_name, types, context);
                    }
                } else if self.is_builtin_type_name(parent_name) {
                    context.error(parent_name, format!("cannot inherit from built-in type: {}", parent_name))
                } else {
                    context.error(parent_name, format!("unknown type: {}", parent_name))
                }
            }
        }

        types
    }

    fn collect_struct_fields(&self, struct_types: &[Identifier], context: &mut ProgramContext) -> HashMap<Identifier, FieldDetails> {
        let mut fields = HashMap::new();

        for type_name in struct_types.iter().rev() {
            let struct_declaration = self.struct_declarations.get(type_name).unwrap();

            for field in &struct_declaration.body.fields {
                if self.is_forbidden_identifier(&field.name) {
                    context.error(&field.name, format!("forbidden field name: {}", &field.name));
                } else {
                    match &field.type_ {
                        Type::Value(value_type) => {
                            if let Some(type_declaration) = self.struct_declarations.get(&value_type.name) {
                                if self.is_entity_qualifier(&type_declaration.qualifier) {
                                    context.error(&field.name, format!("invalid field type: {} (must be bool, num or an entity)", &value_type.name));
                                } else {
                                    let field_details = FieldDetails {
                                        name: field.name.clone(),
                                        expr_type: ExpressionType::from_value_type(value_type),
                                        offset: fields.len(),
                                    };

                                    if fields.insert(field.name.clone(), field_details).is_some() {
                                        context.error(&field.name, format!("duplicate field declaration: {}", &field.name));
                                    }
                                }
                            } else if !self.is_builtin_type_name(&value_type.name) {
                                context.error(&field.name, format!("undefined type `{}`", &value_type.name));
                            }
                        },
                        Type::Function(function_type) => {
                            context.error(function_type, format!("invalid field type: <function> (accepted: builtin type or entity type)"));
                        },
                    }
                }
            }
        }

        fields
    }

    fn check_builtin_method(&self, method: &MethodDeclaration, context: &mut ProgramContext) {
        if !method.conditions.is_empty() {
            context.error(&method.conditions[0], format!("only event callbacks can have conditions"));
        }

        if let Some(signature) = &method.signature {
            context.error(signature, format!("built-in methods do not take arguments nor have a return type"));
        }
    }

    fn is_entity_qualifier(&self, qualifier: &StructQualifier) -> bool {
        match qualifier {
            StructQualifier::Entity | StructQualifier::World | StructQualifier::User => true,
            _ => false
        }
    }

    fn check_struct_qualifier(&self, type_name: &Identifier, required_qualifier: StructQualifier, context: &mut ProgramContext) {
        if self.is_builtin_type_name(type_name) {
            context.error(type_name, format!("required {} (found {})", required_qualifier, type_name));
        } else if let Some(struct_def) = self.struct_declarations.get(type_name) {
            if struct_def.qualifier != required_qualifier {
                context.error(type_name, format!("required {} (found {})", required_qualifier, type_name));
            }
        } else {
            context.error(type_name, format!("unkown type {}", type_name));
        }
    }

    fn is_forbidden_identifier(&self, identifier: &Identifier) -> bool {
        self.is_builtin_type_name(identifier) || KEYWORDS.contains(&identifier.value.as_str())
    }

    fn is_builtin_type_name(&self, name: &Identifier) -> bool {
        name.value == "bool" || name.value == "num" || name.value == "string"
    }

    fn check_type_name(&self, name: &Identifier, context: &mut ProgramContext) -> bool {
        let valid = self.is_builtin_type_name(name) || self.struct_declarations.contains_key(name);

        if !valid {
            context.error(name, format!("undefined type: {}", name));
        }

        valid
    }
}