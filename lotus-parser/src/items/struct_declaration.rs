use std::collections::HashMap;

use parsable::parsable;
use crate::program::{Error, FieldDetails, KEYWORDS, ProgramContext, StructAnnotation, Type, Wasm};
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
    pub fn process_name(&self, index: usize, context: &mut ProgramContext) {
        if is_forbidden_identifier(&self.name) {
            context.error(self, format!("forbidden struct name: {}", &self.name));
        } else {
            let mut struct_annotation = StructAnnotation::default();
            
            struct_annotation.index = index;
            struct_annotation.qualifier = self.qualifier.clone();
            struct_annotation.name = self.name.clone();
            struct_annotation.type_id = context.structs.len() + 1;

            if context.structs.contains_key(&self.name) {
                context.error(&self.name, format!("duplicate type declaration: `{}`", &self.name));
            }

            context.structs.insert(&self.name, struct_annotation);

            if self.qualifier == StructQualifier::World {
                if let Some(current_world) = &context.world_struct_name {
                    context.error(self, format!("re-declaration of world structure"));
                } else {
                    context.world_struct_name = Some(self.name.clone());
                }
            } else if self.qualifier == StructQualifier::User {
                if let Some(current_user) = &context.user_struct_name {
                    context.error(self, format!("re-declaration of user structure"));
                } else {
                    context.user_struct_name = Some(self.name.clone());
                }
            }
        }
    }

    pub fn process_parent(&self, index: usize, context: &mut ProgramContext) {
        let mut errors = vec![];
        let mut final_parent = None;

        if let Some(parent_name) = &self.parent {
            if let Some(parent) = context.structs.get(&self.name) {
                if parent.qualifier == self.qualifier {
                    final_parent = Some(parent_name.clone());
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

        if let Some(struct_annotation) = context.structs.get_mut_by_id(&self.name, index) {
            struct_annotation.parent_name = final_parent;
        }
    }

    pub fn process_inheritence(&self, index: usize, context: &mut ProgramContext) {
        let mut types = vec![self.name.clone()];
        let mut parent_opt = self.parent.as_ref();
        let mut errors = vec![];

        while let Some(parent_name) = parent_opt {
            if let Some(parent_annotation) = context.structs.get(parent_name) {
                if types.contains(parent_name) {
                    if parent_name == &self.name {
                        errors.push(Error::located(&self.name, format!("circular inheritance: `{}`", &self.name)));
                    }
                } else {
                    types.push(parent_name.clone());
                    parent_opt = parent_annotation.parent_name.as_ref();
                }
            }
        }

        context.errors.extend(errors);

        if let Some(struct_annotation) = context.structs.get_mut_by_id(&self.name, index) {
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
                            offset: fields.len(),
                        };

                        fields.insert(field.name.clone(), field_details);
                    } else {
                        context.error(&field.name, format!("forbidden field type: `{}`", field_type));
                    }
                }
            }
        }

        if let Some(struct_annotation) = context.structs.get_mut_by_id(&self.name, index) {
            struct_annotation.self_fields = fields;
        }
    }

    pub fn process_all_fields(&self, index: usize, context: &mut ProgramContext) {
        let mut fields = HashMap::new();
        let type_names = context.structs.get(&self.name).map_or(vec![], |s| s.types.clone());
        let mut errors = vec![];

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
                        errors.push(Error::located(&field.name, format!("duplicate field '{}' (already declared by parent struct `{}`)", &self.name, type_name)));
                    }
                } else {
                    fields.insert(field.name.clone(), field_info);
                }
            }
        }

        context.errors.extend(errors);

        if let Some(struct_annotation) = context.structs.get_mut_by_id(&self.name, index) {
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