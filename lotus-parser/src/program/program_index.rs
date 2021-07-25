use std::{collections::{HashMap}};

use crate::{items::{expression::{Expression, Operand, PathSegment, VarPrefix}, file::LotusFile, function_declaration::{FunctionArgument, FunctionDeclaration}, identifier::Identifier, statement::{VarDeclaration, VarDeclarationQualifier}, struct_declaration::{MethodDeclaration, MethodQualifier, StructDeclaration, StructQualifier, Type}, top_level_block::TopLevelBlock}, program::struct_definition::{FieldType, StructDefinition}};
use super::{context::Context, error::Error, expression_type::{ExpressionType, TypeKind}, function_definition::FunctionDefinition};

const KEYWORDS : &'static[&'static str] = &[
    "let", "const", "struct", "view", "entity", "event", "world",
];

#[derive(Default)]
pub struct ProgramIndex {
    pub world_type_name: Option<Identifier>,
    pub user_type_name: Option<Identifier>,
    pub struct_declarations: HashMap<Identifier, StructDeclaration>,
    pub function_declarations: HashMap<Identifier, FunctionDeclaration>,
    pub constant_declarations: HashMap<Identifier, VarDeclaration>,

    pub struct_definitions: HashMap<Identifier, StructDefinition>,
    pub function_definitions: HashMap<Identifier, FunctionDefinition>,
    pub const_definitions: HashMap<Identifier, ExpressionType>,
}

impl ProgramIndex {
    pub fn from_parsed_files(files: Vec<LotusFile>) -> Result<Self, Vec<Error>> {
        let mut index = Self::default();
        let mut errors = vec![];

        index.build_index(files, &mut errors);
        index.process_structs(&mut errors);
        index.process_functions(&mut errors);
        index.process_constants(&mut errors);
        index.process_function_bodies(&mut errors);


        match errors.is_empty() {
            true => Ok(index),
            false => Err(errors)
        }
    }

    fn build_index(&mut self, files: Vec<LotusFile>, errors: &mut Vec<Error>) {
        for file in files {
            for block in file.blocks {
                let identifier = match &block {
                    TopLevelBlock::StructDeclaration(struct_declaration) => &struct_declaration.name,
                    TopLevelBlock::ConstDeclaration(const_declaration) => &const_declaration.name,
                    TopLevelBlock::FunctionDeclaration(function_declaration) => &function_declaration.name,
                }.clone();

                if self.struct_declarations.contains_key(&identifier) || self.function_declarations.contains_key(&identifier) || self.constant_declarations.contains_key(&identifier) {
                    errors.push(Error::located(&identifier, format!("duplicate declaration: {}", identifier)));
                }

                match block {
                    TopLevelBlock::StructDeclaration(def) => { self.struct_declarations.insert(identifier, def); },
                    TopLevelBlock::ConstDeclaration(var_declaration) => { self.constant_declarations.insert(identifier, var_declaration); },
                    TopLevelBlock::FunctionDeclaration(def) => { self.function_declarations.insert(identifier, def); },
                }
            }
        }

        let world_structs : Vec<Identifier> = self.struct_declarations.values().filter(|s| s.qualifier == StructQualifier::World).map(|s| s.name.clone()).collect();
        let user_structs : Vec<Identifier> = self.struct_declarations.values().filter(|s| s.qualifier == StructQualifier::User).map(|s| s.name.clone()).collect();

        if world_structs.len() > 1 {
            for name in &world_structs {
                errors.push(Error::located(name, "multiple worlds declared"));
            }
        }

        if let Some(name) = world_structs.first() {
            self.world_type_name = Some(name.clone());
        }

        if user_structs.len() > 1 {
            for name in &user_structs {
                errors.push(Error::located(name, "multiple users declared"));
            }
        }

        if let Some(name) = user_structs.first() {
            self.user_type_name = Some(name.clone());
        }
    }

    fn process_constants(&mut self, errors: &mut Vec<Error>) {
        for const_declaration in self.constant_declarations.values() {
            let mut context = Context::new();

            if const_declaration.qualifier != VarDeclarationQualifier::Const {
                errors.push(Error::located(const_declaration, "global variables must be declared with the `const` qualifier"));
            }

            if let Some(expr_type) = self.get_expression_type(&const_declaration.value, true, &mut context, errors) {
                self.const_definitions.insert(const_declaration.name.clone(), expr_type);
            }
        }
    }

    fn get_expression_type(&self, expr: &Expression, is_const: bool, context: &mut Context, errors: &mut Vec<Error>) -> Option<ExpressionType> {
        let first_operand_type = self.get_operand_type(&expr.first, is_const, context, errors);

        None
    }

    fn get_operand_type(&self, operand: &Operand, is_const: bool, context: &mut Context, errors: &mut Vec<Error>) -> Option<ExpressionType> {
        match operand {
            Operand::Parenthesized(_) => todo!(),
            Operand::Number(_) => todo!(),
            Operand::Boolean(_) => todo!(),
            Operand::UnaryOperation(_) => todo!(),
            Operand::VarPath(var_path) => {
                let var_type : Option<ExpressionType> = match &var_path.prefix {
                    Some(prefix) => {
                        let prefix_type = match prefix {
                            VarPrefix::This => {
                                if context.this().is_none() {
                                    errors.push(Error::located(prefix, "no `this` value can be referenced in this context"));
                                }

                                context.this()
                            },
                            VarPrefix::Payload => {
                                if context.payload().is_none() {
                                    errors.push(Error::located(prefix, "no `payload` value can be referenced in this context"));
                                }

                                context.payload()
                            },
                        };

                        if let Some(prefix_type) = prefix_type {
                            let type_def = self.struct_definitions.get(&prefix_type.type_name).unwrap();

                            if let Some(field) = type_def.fields.get(&var_path.name) {
                                Some(field.get_expr_type())
                            } else {
                                errors.push(Error::located(&var_path.name, format!("type `{}` does not have a `{}` field", &prefix_type.type_name, &var_path.name)));
                                None
                            }
                        } else {
                            None
                        }
                    },
                    None => {
                        if is_const {
                            if let Some(referenced_const) = self.constant_declarations.get(&var_path.name) {
                                if let Some(_) = context.visit_constant(&var_path.name) {
                                    errors.push(Error::located(&referenced_const.name, format!("circular reference to `{}`", &referenced_const.name)));

                                    None
                                } else {
                                    self.get_expression_type(&referenced_const.value, is_const, context, errors)
                                }
                            } else {
                                errors.push(Error::located(&var_path.name, format!("undefined constant `{}`", &var_path.name)));
                                None
                            }
                        } else {
                            context.get_var_type(&var_path.name).cloned()
                        }
                    }
                };

                if is_const && !var_path.path.is_empty() {
                    errors.push(Error::located(&var_path.path[0], "field paths not supported in const expressions"));

                    None
                } else if let Some(expr_type) = var_type {
                    let mut final_type = expr_type.clone();

                    for segment in &var_path.path {
                        match segment {
                            PathSegment::FieldAccess(_) => todo!(),
                            PathSegment::BracketIndexing(_) => todo!(),
                            PathSegment::FunctionCall(_) => todo!(),
                        }
                    }

                    Some(final_type)
                } else {
                    None
                }
            }
        }
    }

    fn process_structs(&mut self, errors: &mut Vec<Error>) {
        for struct_declaration in self.struct_declarations.values() {
            if self.is_forbidden_identifier(&struct_declaration.name) {
                errors.push(Error::located(struct_declaration, format!("invalid type name: {}", &struct_declaration.name)));
            } else {
                let mut struct_def = StructDefinition {
                    name: struct_declaration.name.clone(),
                    qualifier: struct_declaration.qualifier,
                    types: vec![],
                    fields: HashMap::new(),
                };

                self.collect_struct_types(&struct_declaration.name, &mut struct_def.types, errors);
                self.collect_struct_fields(&mut struct_def, errors);
            }
        }
    }

    fn process_functions(&mut self, errors: &mut Vec<Error>) {
        

    }

    fn process_function_bodies(&mut self, errors: &mut Vec<Error>) {
        for struct_declaration in self.struct_declarations.values() {
            self.validate_struct_methods(struct_declaration, errors);
        }

        // TODO: functions
    }

    fn collect_struct_types(&self, struct_name: &Identifier, types: &mut Vec<Identifier>, errors: &mut Vec<Error>) {
        if types.contains(struct_name) {
            errors.push(Error::located(struct_name, format!("circular inheritance: {}", struct_name)));
        } else {
            types.push(struct_name.clone());

            let struct_def = self.struct_declarations.get(&struct_name).unwrap();

            if let Some(parent_name) = &struct_def.parent {
                if let Some(parent) = self.struct_declarations.get(parent_name) {
                    if parent.qualifier != struct_def.qualifier {
                        errors.push(Error::located(parent_name, format!("a {} cannot inherit from a {}", struct_def.qualifier, parent.qualifier)));
                    } else {
                        self.collect_struct_types(parent_name, types, errors);
                    }
                } else if self.is_builtin_type_name(parent_name) {
                    errors.push(Error::located(parent_name, format!("cannot inherit from built-in type: {}", parent_name)))
                } else {
                    errors.push(Error::located(parent_name, format!("unknown type: {}", parent_name)))
                }
            }
        }
    }

    fn collect_struct_fields(&self, struct_def: &mut StructDefinition, errors: &mut Vec<Error>) {
        for type_name in struct_def.types.clone().iter().rev() {
            let struct_declaration = self.struct_declarations.get(type_name).unwrap();

            for field in &struct_declaration.body.fields {
                if self.is_forbidden_identifier(&field.name) {
                    errors.push(Error::located(&field.name, format!("invalid field name: {}", &field.name)));
                } else {
                    if !self.is_builtin_type_name(&field.type_.name) {
                        if let Some(field_struct_declaration) = self.struct_declarations.get(&field.type_.name) {
                            if self.is_entity_qualifier(field_struct_declaration.qualifier) {
                                errors.push(Error::located(&field.name, format!("invalid field type: {} (must be bool, num or an entity)", &field.type_.name)));
                            } else {
                                struct_def.add_field(&field.name, &field.type_.name, TypeKind::from_suffix(&field.type_.suffix));
                            }
                        } else {
                            errors.push(Error::located(&field.name, format!("invalid field type: {} (must be bool, num or an entity)", &field.type_.name)));
                        }
                    }
                }
            }
        }
    }

    fn validate_builtin_method(&self, method: &MethodDeclaration, errors: &mut Vec<Error>) {
        if !method.conditions.is_empty() {
            errors.push(Error::located(&method.conditions[0], format!("only event callbacks can have conditions")));
        }

        if !method.arguments.is_empty() {
            errors.push(Error::located(&method.arguments[0], format!("built-in methods do not take arguments")));
        }

        if let Some(return_type) = &method.return_type {
            errors.push(Error::located(return_type, format!("built-in methods do not have a return type")));
        }
    }

    fn validate_struct_methods(&self, struct_declaration: &StructDeclaration, errors: &mut Vec<Error>) {
        for method in &struct_declaration.body.methods {
            match method.qualifier {
                Some(MethodQualifier::Builtin) => {
                    match method.name.as_str() {
                        "on_user_connect" | "on_user_disconnect" => {
                            if struct_declaration.qualifier != StructQualifier::World {
                                errors.push(Error::located(&method.name, format!("method @{} can only be implemented on a world", &method.name)));
                            }

                            self.validate_builtin_method(method, errors);
                        },
                        "trigger" => {
                            if struct_declaration.qualifier != StructQualifier::Event && struct_declaration.qualifier != StructQualifier::Request {
                                errors.push(Error::located(&method.name, format!("method @{} can only be implemented on a events and requests", &method.name)));
                            }

                            self.validate_builtin_method(method, errors);
                        },
                        _ => {
                            errors.push(Error::located(method, format!("invalid built-in method name @{}", &method.name)));
                        }
                    }
                },
                Some(MethodQualifier::Hook | MethodQualifier::Before | MethodQualifier::After) => {
                    if !self.is_entity_qualifier(struct_declaration.qualifier) {
                        errors.push(Error::located(method, "event callbacks can only be defined on an entity, world or user"));
                    }

                    self.check_type_name(&method.name, StructQualifier::Event, errors);

                    for condition in &method.conditions {
                        if let Some(VarPrefix::Payload) = &condition.left.prefix {
                            // ok
                        } else {
                            errors.push(Error::located(&condition.left, "left-hand side of event callback condition must be prefixed by $"));
                        }

                        let event_struct = self.struct_definitions.get(&method.name).unwrap();
                        
                        if let Some(field) = event_struct.fields.get(&condition.left.name) {
                            if field.primitive_type != FieldType::Entity {
                                errors.push(Error::located(&condition.left.name, format!("cannot match event callback on non-entity field")));
                            }
                        } else {
                            errors.push(Error::located(&condition.left.name, format!("event `{}` does not have a `{}` field", &method.name, &condition.left.name)));
                        }

                        if !condition.left.path.is_empty() {
                            errors.push(Error::located(&condition.left.path[0], format!("paths are not supported on event callback conditions")));
                        }

                        if let Some(var_path) = &condition.right {
                            if let Some(VarPrefix::This) = var_path.prefix {
                                // ok
                            } else {
                                errors.push(Error::located(&condition.right, "right-hand side of event callback condition must be prefixed by #"));
                            }

                            let struct_def = self.struct_definitions.get(&struct_declaration.name).unwrap();

                            if let Some(field) = struct_def.fields.get(&var_path.name) {
                                if field.primitive_type != FieldType::Entity {
                                    errors.push(Error::located(&var_path.name, format!("cannot match event callback on non-entity field")));
                                }
                            } else {
                                errors.push(Error::located(&var_path.name, format!("entity `{}` does not have a `{}` field", &struct_declaration.name, &var_path.name)));
                            }

                            if !var_path.path.is_empty() {
                                errors.push(Error::located(&var_path.path[0], format!("paths are not supported on event callback conditions")));
                            }
                        } else {
                            errors.push(Error::located(&condition.left, "right-hand side of event callback condition must not be empty"));
                        }
                    }

                    if !method.arguments.is_empty() {
                        errors.push(Error::located(&method.arguments[0], "event callbacks do not take arguments"));
                    }

                    if let Some(return_type) = &method.return_type {
                        errors.push(Error::located(return_type, "event callbacks do not have a return type"));
                    }

                    // no need to check for name unicity, multiple event callbacks on the same struct are allowed
                },
                None => {
                    // check unicity

                    if !method.conditions.is_empty() {
                        errors.push(Error::located(&method.conditions[0], format!("only event callbacks can have conditions")));
                    }

                    self.check_arguments(&method.arguments, errors);

                    // check arguments
                    // check return type
                },
            }

            // check body
        }
    }

    fn check_arguments(&self, arguments: &[FunctionArgument], errors: &mut Vec<Error>) {
        for argument in arguments {

        }
    }

    fn is_entity_qualifier(&self, qualifier: StructQualifier) -> bool {
        match qualifier {
            StructQualifier::Entity | StructQualifier::World | StructQualifier::User => true,
            _ => false
        }
    }

    fn check_type_name(&self, type_name: &Identifier, required_qualifier: StructQualifier, errors: &mut Vec<Error>) {
        if self.is_builtin_type_name(type_name) {
            errors.push(Error::located(type_name, format!("required {} (found {})", required_qualifier, type_name)));
        } else if let Some(struct_def) = self.struct_definitions.get(type_name) {
            if struct_def.qualifier != required_qualifier {
                errors.push(Error::located(type_name, format!("required {} (found {})", required_qualifier, type_name)));
            }
        } else {
            errors.push(Error::located(type_name, format!("unkown type {}", type_name)));
        }
    }

    fn is_forbidden_identifier(&self, identifier: &Identifier) -> bool {
        self.is_builtin_type_name(identifier) || KEYWORDS.contains(&identifier.value.as_str())
    }

    fn is_builtin_type_name(&self, name: &Identifier) -> bool {
        name.value == "bool" || name.value == "num"
    }

    fn is_type_name(&self, name: &Identifier) -> bool {
        self.is_builtin_type_name(name) || self.struct_declarations.contains_key(name)
    }
}