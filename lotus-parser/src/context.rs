use std::{collections::{HashMap}, fmt::format, ops::Deref};

use parsable::Parsable;

use crate::{constants::KEYWORDS, definitions::struct_definition::{FieldKind, StructDefinition}, error::{Error}, items::{expr::{VarPath, VarPrefix}, file::LotusFile, function_declaration::FunctionDeclaration, identifier::Identifier, statement::{VarDeclaration, VarDeclarationQualifier}, struct_declaration::{MethodQualifier, StructDeclaration, StructQualifier}, top_level_block::TopLevelBlock}};

#[derive(Default)]
pub struct Context {
    pub errors: Vec<Error>,
    pub world_name: Option<Identifier>,
    pub user_name: Option<Identifier>,
    pub struct_declarations: HashMap<Identifier, StructDeclaration>,
    pub function_declarations: HashMap<Identifier, FunctionDeclaration>,
    pub constant_declarations: HashMap<Identifier, VarDeclaration>,

    pub struct_definitions: HashMap<Identifier, StructDefinition>
}

impl Context {
    pub fn new(files: Vec<LotusFile>) -> Self {
        let mut context = Self::default();

        context.build_index(files);
        context.process_structs();

        context
    }

    fn error<T : Parsable, S : Deref<Target=str>>(&mut self, data: &T, error: S) {
        self.errors.push(Error::from(data, error));
    }

    fn build_index(&mut self, files: Vec<LotusFile>) {
        for file in files {
            for block in file.blocks {
                let identifier = match &block {
                    TopLevelBlock::StructDeclaration(struct_declaration) => &struct_declaration.name,
                    TopLevelBlock::ConstDeclaration(const_declaration) => {
                        if const_declaration.qualifier != VarDeclarationQualifier::Const {
                            self.error(const_declaration, "global variables must be declared with the \"const\" qualifier");
                        }

                        &const_declaration.name
                    },
                    TopLevelBlock::FunctionDeclaration(function_declaration) => &function_declaration.name,
                }.clone();

                if self.struct_declarations.contains_key(&identifier) || self.function_declarations.contains_key(&identifier) || self.constant_declarations.contains_key(&identifier) {
                    self.error(&identifier, format!("duplicate declaration: {}", identifier));
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
                self.error(name, "multiple worlds declared");
            }
        }

        if let Some(name) = world_structs.first() {
            self.world_name = Some(name.clone());
        }

        if user_structs.len() > 1 {
            for name in &user_structs {
                self.error(name, "multiple users declared");
            }
        }

        if let Some(name) = user_structs.first() {
            self.user_name = Some(name.clone());
        }
    }

    fn process_structs(&mut self) {
        let mut errors = vec![];

        for struct_declaration in self.struct_declarations.values() {
            if self.is_keyword(&struct_declaration.name) {
                errors.push(Error::from(struct_declaration, format!("invalid type name: {}", &struct_declaration.name)));
            } else {
                let mut struct_def = StructDefinition {
                    name: struct_declaration.name.clone(),
                    qualifier: struct_declaration.qualifier,
                    types: vec![],
                    fields: HashMap::new(),
                };

                self.collect_struct_types(&struct_declaration.name, &mut struct_def.types, &mut errors);
                self.collect_struct_fields(&mut struct_def, &mut errors);
            }
        }

        self.errors.append(&mut errors);
    }

    fn collect_struct_types(&self, struct_name: &Identifier, types: &mut Vec<Identifier>, errors: &mut Vec<Error>) {
        if types.contains(struct_name) {
            errors.push(Error::from(struct_name, format!("circular inheritance: {}", struct_name)));
        } else {
            types.push(struct_name.clone());

            let struct_def = self.struct_declarations.get(&struct_name).unwrap();

            if let Some(parent_name) = &struct_def.parent {
                if let Some(parent) = self.struct_declarations.get(parent_name) {
                    if parent.qualifier != struct_def.qualifier {
                        errors.push(Error::from(parent_name, format!("a {} cannot inherit from a {}", struct_def.qualifier, parent.qualifier)));
                    } else {
                        self.collect_struct_types(parent_name, types, errors);
                    }
                } else if self.is_builtin_type_name(parent_name) {
                    errors.push(Error::from(parent_name, format!("cannot inherit from builtin type: {}", parent_name)))
                } else {
                    errors.push(Error::from(parent_name, format!("unknown type: {}", parent_name)))
                }
            }
        }
    }

    fn collect_struct_fields(&self, struct_def: &mut StructDefinition, errors: &mut Vec<Error>) {
        for type_name in struct_def.types.clone().iter().rev() {
            let struct_declaration = self.struct_declarations.get(type_name).unwrap();

            for field in &struct_declaration.body.fields {
                if self.is_keyword(&field.name) {
                    errors.push(Error::from(&field.name, format!("invalid field name: {}", &field.name)));
                } else {
                    if !self.is_builtin_type_name(&field.type_name) {
                        if let Some(field_struct_declaration) = self.struct_declarations.get(&field.type_name) {
                            if field_struct_declaration.qualifier != StructQualifier::Entity {
                                errors.push(Error::from(&field.name, format!("invalid field type: {} (must be bool, num or an entity)", &field.type_name)));
                            } else {
                                struct_def.add_field(&field.name, &field.type_name, FieldKind::from_suffix(&field.suffix));
                            }
                        } else {
                            errors.push(Error::from(&field.name, format!("invalid field type: {} (must be bool, num or an entity)", &field.type_name)));
                        }
                    }
                }
            }
        }
    }

    fn validate_struct_methods(&self, struct_declaration: &StructDeclaration, errors: &mut Vec<Error>) {
        for method in &struct_declaration.body.methods {
            match method.qualifier {
                Some(MethodQualifier::Builtin) => {
                    match method.name.as_str() {
                        "on_user_connect" | "on_user_disconnect" => {
                            if struct_declaration.qualifier != StructQualifier::World {
                                errors.push(Error::from(&method.name, format!("method @{} can only be implemented on a world", &method.name)));
                            }

                            if let Some(condition) = &method.condition {
                                errors.push(Error::from(condition, format!("method @{} cannot have a condition", &method.name)));
                            }

                            if !method.arguments.is_empty() {
                                errors.push(Error::from(&method.arguments[0], format!("method @{} does not take arguments", &method.name)));
                            }
                        },
                        "trigger" => {
                            if struct_declaration.qualifier != StructQualifier::Event && struct_declaration.qualifier != StructQualifier::Request {
                                errors.push(Error::from(&method.name, format!("method @{} can only be implemented on a events and requests", &method.name)));
                            }

                            if !method.arguments.is_empty() {
                                errors.push(Error::from(&method.arguments[0], format!("method @{} does not take arguments", &method.name)));
                            }

                            if let Some(condition) = &method.condition {
                                errors.push(Error::from(condition, format!("method @{} cannot have a condition", &method.name)));
                            }
                        },
                        _ => {}
                    }
                },
                Some(MethodQualifier::Hook | MethodQualifier::Before | MethodQualifier::After) => {
                    self.check_type_name(&method.name, StructQualifier::Event, errors);

                    if let Some(condition) = &method.condition {
                        if let Some(VarPrefix::Other) = &condition.left.prefix {
                            // ok
                        } else {
                            errors.push(Error::from(&condition.left, "left-hand side of event callback condition must be prefixed by $"));
                        }

                        if let Some(var_path) = &condition.right {
                            if let Some(VarPrefix::This) = var_path.prefix {
                                // ok
                            } else {
                                errors.push(Error::from(&condition.right, "right-hand side of event callback condition must be prefixed by #"));
                            }
                        } else {
                            errors.push(Error::from(&condition.left, "right-hand side of event callback condition must not be empty"));
                        }
                    }

                    // no need to check for name unicity, multiple event callbacks on the same struct are allowed
                },
                None => todo!(),
            }
        }
    }

    fn check_type_name(&self, type_name: &Identifier, required_qualifier: StructQualifier, errors: &mut Vec<Error>) {
        if self.is_builtin_type_name(type_name) {
            errors.push(Error::from(type_name, format!("required {} (found {})", required_qualifier, type_name)));
        } else if let Some(struct_def) = self.struct_definitions.get(type_name) {
            if struct_def.qualifier != required_qualifier {
                errors.push(Error::from(type_name, format!("required {} (found {})", required_qualifier, type_name)));
            }
        } else {
            errors.push(Error::from(type_name, format!("unkown type {}", type_name)));
        }
    }

    fn check_condition_side(&self, struct_def: &StructDefinition, var_path: &VarPath, required_prefix: VarPrefix, errors: &mut Vec<Error>) {
        let prefix_ok = match &var_path.prefix {
            Some(prefix) => *prefix == required_prefix,
            None => false,
        };

        if !prefix_ok {
            errors.push(Error::from(var_path, format!("must be prefixed by {}", required_prefix)));
        }

        if let Some(field) = struct_def.fields.get(&var_path.name) {

        } else {
            errors.push(Error::from(&var_path.name, format!("forbidden field access in method condition {}", required_prefix)));
        }

        if var_path.path.is_empty() {
            errors.push(Error::from(&var_path.path, format!("forbidden field access in method condition {}", required_prefix)));
        }
    }

    fn is_keyword(&self, identifier: &Identifier) -> bool {
        KEYWORDS.contains(&identifier.value.as_str())
    }

    fn is_builtin_type_name(&self, name: &Identifier) -> bool {
        name.value == "bool" || name.value == "num"
    }

    fn is_type_name(&self, name: &Identifier) -> bool {
        self.is_builtin_type_name(name) || self.struct_declarations.contains_key(name)
    }
}