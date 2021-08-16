use std::{collections::HashMap, hash::Hash};

use parsable::parsable;
use crate::program::{Error, FieldDetails, ItemMetadata, KEYWORDS, ProgramContext, StructAnnotation, StructInfo, Type, Wasm};
use super::{FieldDeclaration, Identifier, MethodDeclaration, MethodQualifier, StructQualifier, Visibility};

#[parsable]
pub struct StructDeclaration {
    pub visibility: Visibility,
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
    pub fn process_name(&self, index: usize, context: &mut ProgramContext) {
        if is_forbidden_identifier(&self.name) {
            context.error(self, format!("forbidden struct name: {}", &self.name));
        } else {
            let mut struct_annotation = StructAnnotation {
                metadata: ItemMetadata {
                    id: index,
                    name: self.name.clone(),
                    visibility: self.visibility.get_token(),
                    namespace_name: context.get_current_namespace_name(),
                    file_name: context.get_current_file_name(),
                },
                qualifier: self.qualifier.clone(),
                parent: None,
                types: vec![],
                self_fields: HashMap::new(),
                fields: HashMap::new(),
                user_methods: HashMap::new(),
                builtin_methods: HashMap::new(),
                hook_event_callbacks: HashMap::new(),
                before_event_callbacks: HashMap::new(),
                after_event_callbacks: HashMap::new(),
            };
            
            if context.get_struct_by_name(&self.name).is_some() {
                context.error(&self.name, format!("duplicate type declaration: `{}`", &self.name));
            }

            context.add_struct(struct_annotation);

            // if self.qualifier == StructQualifier::World {
            //     if let Some(current_world) = &context.world_struct_name {
            //         context.error(self, format!("re-declaration of world structure"));
            //     } else {
            //         context.world_struct_name = Some(self.name.clone());
            //     }
            // } else if self.qualifier == StructQualifier::User {
            //     if let Some(current_user) = &context.user_struct_name {
            //         context.error(self, format!("re-declaration of user structure"));
            //     } else {
            //         context.user_struct_name = Some(self.name.clone());
            //     }
            // }
        }
    }

    pub fn process_parent(&self, index: usize, context: &mut ProgramContext) {
        let mut errors = vec![];
        let mut final_parent = None;

        if let Some(parent_name) = &self.parent {
            if let Some(parent) = context.get_struct_by_name(&self.name) {
                if parent.qualifier == self.qualifier {
                    final_parent = Some(parent.get_id());
                } else {
                    errors.push(Error::located(parent_name, format!("a `{}` cannot inherit from a `{}`", &self.qualifier, &parent.qualifier)));
                }
            } else if is_builtin_type_name(parent_name) {
                errors.push(Error::located(parent_name, format!("cannot inherit from built-in type `{}`", parent_name)));
            } else {
                errors.push(Error::located(parent_name, format!("cannot inherit from undefined type `{}`", parent_name)));
            }
        }

        context.errors.extend(errors);

        if let Some(struct_annotation) = context.get_struct_by_id_mut(index) {
            struct_annotation.parent = final_parent;
        }
    }

    pub fn process_inheritence(&self, index: usize, context: &mut ProgramContext) {
        let mut errors = vec![];
        let mut types = vec![index];

        if let Some(struct_annotation) = context.get_struct_by_id(index) {
            let mut parent_opt = struct_annotation.parent;

            while let Some(parent_id) = parent_opt {
                let parent = context.get_struct_by_id(parent_id).unwrap();

                if types.contains(&parent_id) {
                    if parent.get_name() == &self.name {
                        errors.push(Error::located(&self.name, format!("circular inheritance: `{}`", &self.name)));
                    }
                } else {
                    types.push(parent_id);
                    parent_opt = parent.parent.clone();
                }
            }
        }

        context.errors.extend(errors);

        if let Some(struct_annotation) = context.get_struct_by_id_mut(index) {
            struct_annotation.types = types;
        }
    }

    pub fn process_self_fields(&self, index: usize, context: &mut ProgramContext) {
        let mut fields = HashMap::new();

        for field in &self.body.fields {
            if is_forbidden_identifier(&field.name) {
                context.error(&field.name, format!("forbidden field name '{}'", &self.name));
            } else {
                if fields.contains_key(&field.name) {
                    context.error(&field.name, format!("duplicate field `{}`", &self.name));
                }

                if let Some(field_type) = Type::from_parsed_type(&field.ty, context) {
                    let ok = match field_type.leaf_item_type() {
                        Type::Void => false,
                        Type::System => false,
                        Type::Pointer(_) => false,
                        Type::Boolean => true,
                        Type::Integer => true,
                        Type::Float => true,
                        Type::String => true,
                        Type::Null => false,
                        Type::TypeId => false,
                        Type::Struct(_) => true,
                        Type::Function(_, _) => false,
                        Type::Array(_) => unreachable!(),
                        Type::Any(_) => unreachable!(),
                    };

                    if ok {
                        let field_details = FieldDetails {
                            name: field.name.clone(),
                            ty: field_type,
                            offset: 0,
                        };

                        fields.insert(field.name.clone(), field_details);
                    } else {
                        context.error(&field.name, format!("forbidden field type: `{}`", field_type));
                    }
                }
            }
        }

        if let Some(struct_annotation) = context.get_struct_by_id_mut(index) {
            struct_annotation.self_fields = fields;
        }
    }

    pub fn process_all_fields(&self, index: usize, context: &mut ProgramContext) {
        let mut fields = HashMap::new();
        let type_ids = context.get_struct_by_id(index).map_or(vec![], |s| s.types.clone());
        let mut errors = vec![];

        for type_id in type_ids.iter().rev() {
            let struct_annotation = context.get_struct_by_id(*type_id).unwrap();

            for field in struct_annotation.self_fields.values() {
                let field_info = FieldDetails {
                    name: field.name.clone(),
                    ty: field.ty.clone(),
                    offset: fields.len() + 1
                };

                if fields.contains_key(&field.name) {
                    if *type_id != index {
                        errors.push(Error::located(&field.name, format!("duplicate field '{}' (already declared by parent struct `{}`)", &self.name, type_id)));
                    }
                } else {
                    fields.insert(field.name.clone(), field_info);
                }
            }
        }

        context.errors.extend(errors);

        if let Some(struct_annotation) = context.get_struct_by_id_mut(index) {
            struct_annotation.fields = fields;
        }
    }

    pub fn process_methods_signatures(&self, index: usize, context: &mut ProgramContext) {
        for (i, method) in self.body.methods.iter().enumerate() {
            method.process_signature(self, index, i, context);
        }
    }

    pub fn process_methods_bodies(&self, context: &mut ProgramContext) -> Option<Wasm> {
        todo!()
    }
}

fn is_builtin_type_name(name: &Identifier) -> bool {
    Type::builtin_from_str(name.as_str()).is_some()
}

fn is_forbidden_identifier(name: &Identifier) -> bool {
    KEYWORDS.contains(&name.as_str()) || is_builtin_type_name(name)
}