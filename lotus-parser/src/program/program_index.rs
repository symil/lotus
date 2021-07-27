use std::{collections::{HashMap, HashSet}};

use crate::{items::{expression::{Expression, Operand, PathSegment, VarPath, VarPrefix}, file::LotusFile, function_declaration::{FunctionDeclaration, FunctionSignature}, identifier::Identifier, statement::{VarDeclaration, VarDeclarationQualifier}, struct_declaration::{MethodDeclaration, MethodQualifier, StructDeclaration, StructQualifier, Type}, top_level_block::TopLevelBlock}, program::{builtin_methods::{get_array_field_type, get_builtin_field_type}, struct_annotation::{FieldPrimitiveType, StructAnnotation}}};
use super::{context::Context, error::Error, expression_type::{ExpressionType}, function_annotation::FunctionAnnotation};

const KEYWORDS : &'static[&'static str] = &[
    "let", "const", "struct", "view", "entity", "event", "world", "user"
];

#[derive(Default)]
pub struct ProgramIndex {
    pub world_type_name: Option<Identifier>,
    pub user_type_name: Option<Identifier>,
    pub struct_declarations: HashMap<Identifier, StructDeclaration>,
    pub function_declarations: HashMap<Identifier, FunctionDeclaration>,
    pub const_declarations: HashMap<Identifier, VarDeclaration>,
}

#[derive(Default)]
struct ProgramAnnotations {
    pub structs: HashMap<Identifier, StructAnnotation>,
    pub functions: HashMap<Identifier, FunctionAnnotation>,
    pub constants: HashMap<Identifier, ExpressionType>,
}

impl ProgramIndex {
    pub fn from_parsed_files(files: Vec<LotusFile>) -> Result<Self, Vec<Error>> {
        let mut index = Self::default();
        let mut annotations = ProgramAnnotations::default();
        let mut errors = vec![];

        index.build_index(files, &mut annotations, &mut errors);
        index.process_structs_fields(&mut annotations, &mut errors);
        index.process_structs_method_signature(&mut annotations, &mut errors);
        index.process_functions_signatures(&mut annotations, &mut errors);
        index.process_constants(&mut annotations, &mut errors);
        index.process_function_bodies(&mut annotations, &mut errors);

        match errors.is_empty() {
            true => Ok(index),
            false => Err(errors)
        }
    }

    fn build_index(&mut self, files: Vec<LotusFile>, annotations: &mut ProgramAnnotations, errors: &mut Vec<Error>) {
        for file in files {
            for block in file.blocks {
                let identifier = match &block {
                    TopLevelBlock::StructDeclaration(struct_declaration) => &struct_declaration.name,
                    TopLevelBlock::ConstDeclaration(const_declaration) => &const_declaration.name,
                    TopLevelBlock::FunctionDeclaration(function_declaration) => &function_declaration.name,
                }.clone();

                if self.struct_declarations.contains_key(&identifier) || self.function_declarations.contains_key(&identifier) || self.const_declarations.contains_key(&identifier) {
                    errors.push(Error::located(&identifier, format!("duplicate declaration: {}", identifier)));
                }

                match block {
                    TopLevelBlock::StructDeclaration(struct_declaration) => {
                        annotations.structs.insert(identifier.clone(), StructAnnotation::new(&identifier, &struct_declaration.qualifier));
                        self.struct_declarations.insert(identifier, struct_declaration);
                    },
                    TopLevelBlock::ConstDeclaration(var_declaration) => {
                        annotations.constants.insert(identifier.clone(), ExpressionType::void());
                        self.const_declarations.insert(identifier, var_declaration);
                    },
                    TopLevelBlock::FunctionDeclaration(def) => {
                        annotations.functions.insert(identifier.clone(), FunctionAnnotation::new(&identifier));
                        self.function_declarations.insert(identifier, def);
                    },
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

    fn process_structs_fields(&mut self, annotations: &mut ProgramAnnotations, errors: &mut Vec<Error>) {
        for (struct_declaration, struct_annotation) in self.struct_declarations.values().zip(annotations.structs.values_mut()) {
            if self.is_forbidden_identifier(&struct_declaration.name) {
                errors.push(Error::located(struct_declaration, format!("invalid type name: {}", &struct_declaration.name)));
            } else {
                self.collect_struct_types(&struct_declaration.name, &mut struct_annotation.types, errors);
                self.collect_struct_fields(struct_annotation, errors);
            }
        }
    }

    fn process_structs_method_signature(&mut self, annotations: &mut ProgramAnnotations, errors: &mut Vec<Error>) {
        for struct_declaration in self.struct_declarations.values() {
            for method in &struct_declaration.body.methods {
                match method.qualifier {
                    Some(MethodQualifier::Builtin) => {
                        match method.name.as_str() {
                            "on_user_connect" | "on_user_disconnect" => {
                                if struct_declaration.qualifier != StructQualifier::World {
                                    errors.push(Error::located(&method.name, format!("method @{} can only be implemented on a world", &method.name)));
                                }

                                self.check_builtin_method(method, errors);
                            },
                            "trigger" => {
                                if struct_declaration.qualifier != StructQualifier::Event && struct_declaration.qualifier != StructQualifier::Request {
                                    errors.push(Error::located(&method.name, format!("method @{} can only be implemented on a events and requests", &method.name)));
                                }

                                self.check_builtin_method(method, errors);
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

                        self.check_struct_qualifier(&method.name, StructQualifier::Event, errors);

                        for condition in &method.conditions {
                            if let Some(VarPrefix::Payload) = &condition.left.prefix {
                                // ok
                            } else {
                                errors.push(Error::located(&condition.left, "left-hand side of event callback condition must be prefixed by $"));
                            }

                            let event_struct_annotation = annotations.structs.get(&method.name).unwrap();

                            if let Some(field) = event_struct_annotation.fields.get(&condition.left.name) {
                                if field.primitive_type != FieldPrimitiveType::Entity {
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

                                let this_struct_annotation = annotations.structs.get(&struct_declaration.name).unwrap();

                                if let Some(field) = this_struct_annotation.fields.get(&var_path.name) {
                                    if field.primitive_type != FieldPrimitiveType::Entity {
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

                        if let Some(signature) = &method.signature {
                            errors.push(Error::located(signature, "event callbacks do not take arguments nor have a return type"));
                        }

                        // no need to check for name unicity, multiple event callbacks on the same struct are allowed
                    },
                    None => {
                        let mut method_annotation = FunctionAnnotation::new(&method.name);

                        if !method.conditions.is_empty() {
                            errors.push(Error::located(&method.conditions[0], format!("only event callbacks can have conditions")));
                        }

                        if let Some(signature) = &method.signature {
                            self.process_function_signature(signature, &mut method_annotation, errors);
                        } else {
                            errors.push(Error::located(&method.name, format!("missing method arguments")));
                        }

                        let struct_annotation = annotations.structs.get_mut(&struct_declaration.name).unwrap();

                        if struct_annotation.fields.contains_key(&method.name) {
                            errors.push(Error::located(&method.name, format!("duplicate method declaration: field `{}` already exists", &method.name)));
                        }

                        if struct_annotation.methods.insert(method.name.clone(), method_annotation).is_some() {
                            errors.push(Error::located(&method.name, format!("duplicate method declaration: method `{}` already exists", &method.name)));
                        }
                    },
                }
            }
        }
    }

    fn process_functions_signatures(&mut self, annotations: &mut ProgramAnnotations, errors: &mut Vec<Error>) {
        for (function_declaration, function_annotation) in self.function_declarations.values().zip(annotations.functions.values_mut()) {
            self.process_function_signature(&function_declaration.signature, function_annotation, errors);
        }
    }

    fn process_constants(&mut self, annotations: &mut ProgramAnnotations, errors: &mut Vec<Error>) {
        for const_declaration in self.const_declarations.values() {
            let mut context = Context::new();

            if const_declaration.qualifier != VarDeclarationQualifier::Const {
                errors.push(Error::located(const_declaration, "global variables must be declared with the `const` qualifier"));
            }

            if let Some(expr_type) = self.get_expression_type(&const_declaration.value, true, &mut context, annotations, errors) {
                *annotations.constants.get_mut(&const_declaration.name).unwrap() = expr_type;
            }
        }
    }

    fn process_function_bodies(&mut self, annotations: &mut ProgramAnnotations, errors: &mut Vec<Error>) {
        // TODO: functions
    }

    fn process_function_signature(&self, signature: &FunctionSignature, function_annotation: &mut FunctionAnnotation, errors: &mut Vec<Error>) {
        let mut arg_names = HashSet::new();
        let mut arguments = vec![];

        for argument in &signature.arguments {
            if !arg_names.insert(&argument.name) {
                errors.push(Error::located(&argument.name, format!("duplicate argument: {}", &argument.name)));
            }

            let arg_type = match self.check_type_name(&argument.type_.name, errors) {
                true => ExpressionType::from_value_type(&argument.type_),
                false => ExpressionType::void()
            };

            arguments.push(arg_type);
        }

        function_annotation.arguments = arguments;

        if let Some(return_type) = &signature.return_type {
            if self.check_type_name(&return_type.name, errors) {
                function_annotation.return_type = ExpressionType::from_value_type(return_type);
            }
        }
    }

    fn get_expression_type(&self, expr: &Expression, is_const: bool, context: &mut Context, annotations: &mut ProgramAnnotations, errors: &mut Vec<Error>) -> Option<ExpressionType> {
        let first_operand_type = self.get_operand_type(&expr.first, is_const, context, annotations, errors);

        None
    }

    fn get_operand_type(&self, operand: &Operand, is_const: bool, context: &mut Context, annotations: &mut ProgramAnnotations, errors: &mut Vec<Error>) -> Option<ExpressionType> {
        match operand {
            Operand::Parenthesized(_) => todo!(),
            Operand::Number(_) => todo!(),
            Operand::Boolean(_) => todo!(),
            Operand::UnaryOperation(_) => todo!(),
            Operand::VarPath(var_path) => self.get_var_path_type(var_path, is_const, context, annotations, errors)
        }
    }

    fn get_var_path_type(&self, var_path: &VarPath, is_const: bool, context: &mut Context, annotations: &mut ProgramAnnotations, errors: &mut Vec<Error>) -> Option<ExpressionType> {
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

                if let Some(ExpressionType::Single(type_name)) = prefix_type {
                    let type_def = annotations.structs.get(type_name).unwrap();

                    if var_path.name.is("_") {
                        // special case: `_` refers to the value itself rather than a field
                        // e.g `#foo` means `self.foo`, but `#_` means `self`
                        Some(ExpressionType::Single(type_name.clone()))
                    } else if let Some(field) = type_def.fields.get(&var_path.name) {
                        Some(field.get_expr_type())
                    } else {
                        errors.push(Error::located(&var_path.name, format!("type `{}` does not have a `{}` field", type_name, &var_path.name)));
                        None
                    }
                } else {
                    None
                }
            },
            None => {
                if is_const {
                    if let Some(referenced_const) = self.const_declarations.get(&var_path.name) {
                        if let Some(_) = context.visit_constant(&var_path.name) {
                            errors.push(Error::located(&referenced_const.name, format!("circular reference to `{}`", &referenced_const.name)));

                            None
                        } else {
                            self.get_expression_type(&referenced_const.value, is_const, context, annotations, errors)
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
            errors.push(Error::located(&var_path.path[0], "field paths are not supported in const expressions"));

            None
        } else if let Some(expr_type) = var_type {
            let mut final_type = expr_type.clone();

            for segment in &var_path.path {
                let next_type : Option<ExpressionType> = match segment {
                    PathSegment::FieldAccess(field_name) => {
                        match final_type {
                            ExpressionType::Void => {
                                errors.push(Error::located(field_name, format!("void type has no field `{}`", field_name)));
                                None
                            },
                            ExpressionType::Single(type_name) => {
                                let mut result = None;

                                if let Some(struct_annotation) = annotations.structs.get(&type_name) {
                                    if let Some(field) = struct_annotation.fields.get(field_name) {
                                        result = Some(field.get_expr_type());
                                    } else if let Some(method) = struct_annotation.methods.get(field_name) {
                                        result = Some(method.get_expr_type());
                                    }
                                } else {
                                    result = get_builtin_field_type(&type_name, field_name);
                                }

                                if result.is_none() {
                                    errors.push(Error::located(field_name, format!("type `{}` has no field `{}`", &type_name, field_name)));
                                }

                                result
                            },
                            ExpressionType::Array(type_name) => get_array_field_type(Some(&type_name), field_name),
                            ExpressionType::Function(_, _) => {
                                errors.push(Error::located(field_name, format!("functions have no field `{}`", field_name)));
                                None
                            },
                            ExpressionType::SingleAny => {
                                errors.push(Error::located(field_name, format!("invalid field access `{}`: cannot infer parent type", field_name)));
                                None
                            },
                            ExpressionType::ArrayAny => get_array_field_type(None, field_name),
                            
                        }
                    },
                    PathSegment::BracketIndexing(_) => todo!(),
                    PathSegment::FunctionCall(_) => todo!(),
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

    fn collect_struct_fields(&self, struct_annotation: &mut StructAnnotation, errors: &mut Vec<Error>) {
        for type_name in struct_annotation.types.clone().iter().rev() {
            let struct_declaration = self.struct_declarations.get(type_name).unwrap();

            for field in &struct_declaration.body.fields {
                if self.is_forbidden_identifier(&field.name) {
                    errors.push(Error::located(&field.name, format!("forbidden field name: {}", &field.name)));
                } else {
                    match &field.type_ {
                        Type::Value(value_type) => {
                            if let Some(type_declaration) = self.struct_declarations.get(&value_type.name) {
                                if self.is_entity_qualifier(type_declaration.qualifier) {
                                    errors.push(Error::located(&field.name, format!("invalid field type: {} (must be bool, num or an entity)", &value_type.name)));
                                } else {
                                    struct_annotation.add_field(&field.name, value_type);
                                }
                            } else if !self.is_builtin_type_name(&value_type.name) {
                                errors.push(Error::located(&field.name, format!("undefined type `{}`", &value_type.name)));
                            }
                        },
                        Type::Function(function_type) => {
                            errors.push(Error::located(function_type, format!("invalid field type: <function> (must be `bool`, `num` or an entity)")));
                        },
                    }
                }
            }
        }
    }

    fn check_builtin_method(&self, method: &MethodDeclaration, errors: &mut Vec<Error>) {
        if !method.conditions.is_empty() {
            errors.push(Error::located(&method.conditions[0], format!("only event callbacks can have conditions")));
        }

        if let Some(signature) = &method.signature {
            errors.push(Error::located(signature, format!("built-in methods do not take arguments nor have a return type")));
        }
    }

    fn is_entity_qualifier(&self, qualifier: StructQualifier) -> bool {
        match qualifier {
            StructQualifier::Entity | StructQualifier::World | StructQualifier::User => true,
            _ => false
        }
    }

    fn check_struct_qualifier(&self, type_name: &Identifier, required_qualifier: StructQualifier, errors: &mut Vec<Error>) {
        if self.is_builtin_type_name(type_name) {
            errors.push(Error::located(type_name, format!("required {} (found {})", required_qualifier, type_name)));
        } else if let Some(struct_def) = self.struct_declarations.get(type_name) {
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

    fn check_type_name(&self, name: &Identifier, errors: &mut Vec<Error>) -> bool {
        let valid = self.is_builtin_type_name(name) || self.struct_declarations.contains_key(name);

        if !valid {
            errors.push(Error::located(name, format!("undefined type: {}", name)));
        }

        valid
    }
}