use std::{collections::HashMap, hash::Hash};
use indexmap::IndexMap;
use parsable::parsable;
use crate::program::{Error, FieldDetails, KEYWORDS, OBJECT_HEADER_SIZE, ProgramContext, StructInfo, Type, TypeBlueprint, TypeOld, TypeRef, Wasm};
use super::{FieldDeclaration, FullType, GenericParameters, Identifier, MethodDeclaration, EventCallbackQualifier, StackType, StackTypeToken, TypeQualifier, Visibility, VisibilityToken};

#[parsable]
pub struct TypeDeclaration {
    pub visibility: VisibilityToken,
    pub qualifier: TypeQualifier,
    #[parsable(brackets="()")]
    pub stack_type: Option<StackTypeToken>,
    pub generics: GenericParameters,
    pub name: Identifier,
    #[parsable(prefix=":")]
    pub parent: Option<FullType>,
    #[parsable(brackets="{}")]
    pub body: TypeDeclarationBody,
}

#[parsable]
#[derive(Default)]
pub struct TypeDeclarationBody {
    #[parsable(sep=",")]
    pub fields: Vec<FieldDeclaration>,
    pub methods: Vec<MethodDeclaration>
}

impl TypeDeclaration {
    pub fn process_name(&self, context: &mut ProgramContext) -> u64 {
        let type_id = self.location.get_hash();
        let type_blueprint = TypeBlueprint {
            type_id,
            name: self.name.to_string(),
            location: self.location.clone(),
            visibility: self.visibility.value.unwrap_or(Visibility::Private),
            qualifier: self.qualifier,
            stack_type: self.stack_type.and_then(|stack_type| Some(stack_type.value)).unwrap_or(StackType::Pointer),
            generics: self.generics.process(context),
            parent: None,
            inheritance_chain: vec![],
            fields: IndexMap::new(),
            static_fields: IndexMap::new(),
            methods: IndexMap::new(),
            static_methods: IndexMap::new(),
            hook_event_callbacks: IndexMap::new(),
            before_event_callbacks: IndexMap::new(),
            after_event_callbacks: IndexMap::new(),
        };
        
        if context.types.get_by_name(&self.name).is_some() {
            context.errors.add(&self.name, format!("duplicate type declaration: `{}`", &self.name));
        }

        context.types.insert(type_blueprint);

        type_id
    }

    pub fn process_parent(&self, context: &mut ProgramContext) {
        let type_id = self.location.get_hash();
        let mut result = None;

        if let Some(parsed_parent_type) = &self.parent {
            if let Some(parent_type) = parsed_parent_type.process(context) {
                if self.qualifier == TypeQualifier::Type {
                    context.errors.add(parsed_parent_type, format!("regular types cannot inherit"));
                }

                match parent_type {
                    Type::Generic(_) => {
                        context.errors.add(parsed_parent_type, format!("cannot inherit from generic parameter"));
                    },
                    Type::Actual(type_ref) => {
                        let parent_blueprint = context.types.get_by_id(type_ref.type_id).unwrap();

                        match &parent_blueprint.qualifier {
                            TypeQualifier::Type => {
                                context.errors.add(parsed_parent_type, format!("cannot inherit from regular types"));
                            },
                            _ => {
                                if parent_blueprint.qualifier != self.qualifier {
                                    context.errors.add(parsed_parent_type, format!("`{}` types cannot inherit from `{}` types", self.qualifier, parent_blueprint.qualifier));
                                } else if self.qualifier != TypeQualifier::Type {
                                    result = Some(type_ref);
                                }
                            }
                        };
                    },
                    _ => unreachable!()
                }
            }
        }

        context.types.get_mut_by_id(type_id).parent = result;
    }

    pub fn process_inheritence(&self, context: &mut ProgramContext) {
        let type_id = self.location.get_hash();
        let type_blueprint = context.types.get_by_id(type_id).unwrap();
        let mut types = vec![type_blueprint.get_typeref()];
        let mut parent_opt = type_blueprint.parent;

        while let Some(parent_typeref) = parent_opt {
            let parent = context.types.get_by_id(parent_typeref.type_id).unwrap();

            if types.iter().any(|typeref| typeref.type_id == parent.type_id) {
                if parent.type_id == type_blueprint.type_id {
                    context.errors.add(&self.name, format!("circular inheritance: `{}`", &self.name));
                }
            } else {
                types.push(parent_typeref.clone());
                parent_opt = parent.parent.clone();
            }
        }

        types.reverse();

        context.types.get_mut_by_id(type_id).inheritance_chain = types;
    }

    pub fn process_self_fields(&self, context: &mut ProgramContext) {
        let type_id = self.location.get_hash();
        let type_blueprint = context.types.get_by_id(type_id).unwrap();
        let mut fields = IndexMap::new();

        for field in &self.body.fields {
            if fields.contains_key(field.name.as_str()) {
                context.errors.add(&field.name, format!("duplicate field `{}`", &self.name));
            }

            if let Some(field_type) = field.ty.process(context) {
                let field_details = FieldDetails {
                    ty: field_type,
                    name: field.name.clone(),
                    owner_type_id: type_blueprint.type_id,
                    offset: 0,
                };

                fields.insert(field.name.to_string(), field_details);
            }
        }
        
        context.types.get_mut_by_id(type_id).fields = fields;
    }

    pub fn process_all_fields(&self, context: &mut ProgramContext) {
        let type_id = self.location.get_hash();
        let type_blueprint = context.types.get_by_id(type_id).unwrap();
        let mut fields : IndexMap<String, FieldDetails> = IndexMap::new();

        for typeref in &type_blueprint.inheritance_chain {
            let parent_blueprint = context.types.get_by_id(typeref.type_id).unwrap();

            for field in parent_blueprint.fields.values() {
                if field.owner_type_id == typeref.type_id {
                    let field_info = FieldDetails {
                        name: field.name.clone(),
                        ty: field.ty.clone(),
                        owner_type_id: field.owner_type_id,
                        offset: fields.len() + OBJECT_HEADER_SIZE
                    };

                    if let Some(other_field) = fields.get(field.name.as_str()) {
                        context.errors.add(&field.name, format!("duplicate field `{}` (already declared by parent struct `{}`)", &self.name, &other_field.name));
                    } else {
                        fields.insert(field.name.to_string(), field_info);
                    }
                }
            }
        }

        context.types.get_mut_by_id(type_id).fields = fields;
    }

    pub fn process_methods_signatures(&self, context: &mut ProgramContext) {
        let type_id = self.location.get_hash();

        context.current_type = Some(type_id);

        for method in self.body.methods.iter() {
            method.process_signature(context);
        }

        context.current_type = None;
    }

    pub fn process_methods_bodies(&self, context: &mut ProgramContext) {
        let type_id = self.location.get_hash();

        context.current_type = Some(type_id);

        for method in self.body.methods.iter() {
            method.process_body(context);
        }

        context.current_type = None;
    }
}