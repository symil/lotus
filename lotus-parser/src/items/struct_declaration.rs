use std::collections::HashMap;

use parsable::parsable;
use crate::program::{FieldDetails, KEYWORDS, ProgramContext, StructAnnotation, Type, Wasm};
use super::{FieldDeclaration, Identifier, MethodDeclaration, MethodQualifier, StructQualifier};

#[parsable]
#[derive(Default)]
pub struct StructDeclaration {
    pub qualifier: StructQualifier,
    pub name: Identifier,
    #[parsable(prefix=":")]
    pub parent: Option<Identifier>,
    #[parsable(brackets="{}")]
    pub body: StructDeclarationBody,
}

#[parsable]
#[derive(Default)]
pub struct StructDeclarationBody {
    #[parsable(sep=",")]
    pub fields: Vec<FieldDeclaration>,
    pub methods: Vec<MethodDeclaration>
}

impl StructDeclaration {
    pub fn process_name(&self, context: &mut ProgramContext) {
        if is_forbidden_identifier(&self.name) {
            context.error(self, format!("forbidden struct name: {}", &self.name));
        } else {
            let mut struct_annotation = StructAnnotation::default();
            
            struct_annotation.qualifier = self.qualifier.clone();
            struct_annotation.name = self.name.clone();
            struct_annotation.type_id = context.structs.len() + 1;

            if context.structs.insert(self.name.clone(), struct_annotation).is_some() {
                context.error(&self.name, format!("duplicate type declaration: `{}`", &self.name));
            }
        }
    }

    pub fn process_parent(&self, context: &mut ProgramContext) {
        let mut final_parent = None;

        if let Some(parent_name) = &self.parent {
            if let Some(parent) = context.structs.get(&self.name) {
                if parent.qualifier == self.qualifier {
                    final_parent = Some(parent_name.clone());
                } else {
                    context.error(parent_name, format!("a `{}` cannot inherit from a `{}`", &self.qualifier, &parent.qualifier));
                }
            } else if is_builtin_type_name(parent_name) {
                context.error(parent_name, format!("cannot inherit from built-in type `{}`", parent_name));
            } else {
                context.error(parent_name, format!("cannot inherit from undefined type `{}`", parent_name))
            }
        }

        if let Some(struct_annotation) = context.structs.get_mut(&self.name) {
            struct_annotation.parent_name = final_parent;
        }
    }

    pub fn process_inheritence(&self, context: &mut ProgramContext) {
        let mut types = vec![self.name.clone()];
        let mut parent_opt = self.parent.as_ref();

        while let Some(parent_name) = parent_opt {
            if let Some(parent_annotation) = context.structs.get(parent_name) {
                if types.contains(parent_name) {
                    if parent_name == &self.name {
                        context.error(&self.name, format!("circular inheritance: `{}`", &self.name));
                    }
                } else {
                    types.push(parent_name.clone());
                    parent_opt = parent_annotation.parent_name.as_ref();
                }
            }
        }

        if let Some(struct_annotation) = context.structs.get_mut(&self.name) {
            struct_annotation.types = types;
        }
    }

    pub fn process_self_fields(&self, context: &mut ProgramContext) {
        let mut fields = HashMap::new();

        for field in &self.body.fields {
            if is_forbidden_identifier(&field.name) {
                context.error(&field.name, format!("forbidden field name '{}'", &self.name));
            } else {
                if fields.contains_key(&field.name) {
                    context.error(&field.name, format!("duplicate field `{}`", &self.name));
                }

                if let Some(field_type) = Type::from_parsed_type(&field.ty, context) {
                    let ok = match field_type.item_type() {
                        Type::Void => false,
                        Type::System => false,
                        Type::Pointer => false,
                        Type::Boolean => true,
                        Type::Integer => true,
                        Type::Float => true,
                        Type::String => true,
                        Type::Null => false,
                        Type::Struct(_) => true,
                        Type::TypeId(_) => false,
                        Type::Function(_, _) => false,
                        Type::Array(_) => unreachable!(),
                        Type::Any(_) => unreachable!(),
                    };

                    if ok {
                        let field_details = FieldDetails {
                            name: field.name.clone(),
                            ty: field_type,
                            offset: fields.len(),
                        };

                        fields.insert(field.name.clone(), field_details);
                    } else {
                        context.error(&field.name, format!("forbidden field type: `{}`", field_type));
                    }
                }
            }
        }

        if let Some(struct_annotation) = context.structs.get_mut(&self.name) {
            struct_annotation.self_fields = fields;
        }
    }

    pub fn process_all_fields(&self, context: &mut ProgramContext) {
        let mut fields = HashMap::new();
        let type_names = context.structs.get(&self.name).map_or(vec![], |s| s.types.clone());

        for type_name in type_names.iter().rev() {
            let struct_annotation = context.structs.get(type_name).unwrap();

            for field in struct_annotation.self_fields.values() {
                let field_info = FieldDetails {
                    name: field.name.clone(),
                    ty: field.ty.clone(),
                    offset: fields.len()
                };

                if fields.contains_key(&field.name) {
                    if type_name != &self.name {
                        context.error(&field.name, format!("duplicate field '{}' (already declared by parent struct `{}`)", &self.name, type_name));
                    }
                } else {
                    fields.insert(field.name.clone(), field_info);
                }
            }
        }

        if let Some(struct_annotation) = context.structs.get_mut(&self.name) {
            struct_annotation.fields = fields;
        }
    }

    pub fn process_methods(&self, context: &mut ProgramContext) {
        for method in &self.body.methods {
            match method.qualifier {
                Some(MethodQualifier::Builtin) => {
                    if let Some((valid_qualifiers, _)) = get_builtin_method_info(&method.name) {
                        if !valid_qualifiers.iter().any(|qualifier| qualifier == &struct_declaration.qualifier) {
                            context.error(&method.name, format!("method `@{}` can only be implemented on {}", &method.name, display_join(&valid_qualifiers)));
                        }

                        self.check_builtin_method(method, context);
                    } else {
                        context.error(method, format!("invalid built-in method name `@{}`", &method.name));
                    }
                },
                Some(MethodQualifier::Hook | MethodQualifier::Before | MethodQualifier::After) => {
                    if !self.is_entity_qualifier(&struct_declaration.qualifier) {
                        context.error(method, "event callbacks can only be defined on an entity, world or user");
                    }

                    self.check_struct_qualifier(&method.name, StructQualifier::Event, context);

                    for condition in &method.conditions {
                        if !condition.left.has_payload_prefix() {
                            context.error(&condition.left, "left-hand side of event callback condition must be prefixed by $");
                        }

                        let event_struct_annotation = context.structs.get(&method.name).unwrap();

                        if let Some(field) = event_struct_annotation.fields.get(&condition.left.name) {
                            let mut ok = false;

                            if let Type::Single(ItemType::Struct(struct_name)) = &field.expr_type {
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

                        // if !condition.left.path.is_empty() {
                        //     context.error(&condition.left.path[0], format!("paths are not supported on event callback conditions"));
                        // }

                        if let Some(var_path) = &condition.right {
                            if !var_path.has_this_prefix() {
                                context.error(&condition.right, "right-hand side of event callback condition must be prefixed by #");
                            }

                            let this_struct_annotation = context.structs.get(&struct_declaration.name).unwrap();

                            if let Some(field) = this_struct_annotation.fields.get(&var_path.name) {
                                let mut ok = false;

                                if let Type::Single(ItemType::Struct(struct_name)) = &field.expr_type {
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

                            // if !var_path.path.is_empty() {
                            //     context.error(&var_path.path[0], format!("paths are not supported on event callback conditions"));
                            // }
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

    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        todo!()
    }
}

fn is_builtin_type_name(name: &Identifier) -> bool {
    Type::builtin_from_str(name.as_str()).is_some()
}

fn is_forbidden_identifier(name: &Identifier) -> bool {
    KEYWORDS.contains(&name.as_str()) || is_builtin_type_name(name)
}