use std::{collections::HashMap, hash::Hash};
use indexmap::IndexMap;
use parsable::parsable;
use crate::program::{ActualTypeInfo, Error, FieldDetails, KEYWORDS, MethodDetails, OBJECT_HEADER_SIZE, ProgramContext, StructInfo, Type, TypeBlueprint, TypeOld, Wasm};
use super::{FieldDeclaration, FullType, TypeParameters, Identifier, MethodDeclaration, EventCallbackQualifier, StackType, StackTypeToken, TypeQualifier, Visibility, VisibilityWrapper};

#[parsable]
pub struct TypeDeclaration {
    pub visibility: VisibilityWrapper,
    pub qualifier: TypeQualifier,
    #[parsable(brackets="()")]
    pub stack_type: Option<StackTypeToken>,
    pub parameters: TypeParameters,
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
            parameters: self.parameters.process(context),
            associated_types: IndexMap::new(),
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

        context.current_type = Some(type_id);

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

        context.current_type = None;
        context.types.get_mut_by_id(type_id).parent = result;
    }

    pub fn process_inheritance_chain(&self, context: &mut ProgramContext) {
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

    pub fn process_fields(&self, context: &mut ProgramContext) {
        let type_id = self.location.get_hash();
        let type_blueprint = context.types.get_by_id(type_id).unwrap();
        let mut fields = IndexMap::new();

        context.current_type = Some(type_id);

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
        
        context.current_type = None;
        context.types.get_mut_by_id(type_id).fields = fields;
    }

    pub fn process_method_signatures(&self, context: &mut ProgramContext) {
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

    fn process_inheritance<'a, F : FnMut(u64, Vec<u64>, &mut ProgramContext)>(&self, context: &'a mut ProgramContext, f : F) -> &'a mut TypeBlueprint {
        let type_id = self.location.get_hash();
        let type_blueprint = context.types.get_by_id(type_id).unwrap();
        let parent_type_id_list : Vec<u64> = type_blueprint.inheritance_chain.iter().map(|info| info.type_id).collect();

        f(type_id, parent_type_id_list, context);

        context.types.get_mut_by_id(type_id)
    }

    pub fn process_fields_inheritance(&self, context: &mut ProgramContext) {
        let mut fields : IndexMap<String, FieldDetails> = IndexMap::new();
        let mut type_blueprint = self.process_inheritance(context, |type_id, parent_type_id_list, context| {
            for parent_type_id in parent_type_id_list {
                let parent_blueprint = context.types.get_by_id(parent_type_id).unwrap();

                for field in parent_blueprint.fields.values() {
                    if field.owner_type_id == parent_type_id {
                        let field_info = FieldDetails {
                            name: field.name.clone(),
                            ty: field.ty.clone(),
                            owner_type_id: field.owner_type_id,
                            offset: fields.len() + OBJECT_HEADER_SIZE
                        };

                        if fields.insert(field.name.to_string(), field_info).is_some() && field.owner_type_id == type_id {
                            context.errors.add(&self.name, format!("duplicate field `{}` (already declared by parent struct `{}`)", &self.name, &parent_blueprint.name));
                        }
                    }
                }
            }
        });

        type_blueprint.fields = fields;
    }

    pub fn process_methods_inheritance(&self, context: &mut ProgramContext) {
        let mut methods : IndexMap<String, MethodDetails> = IndexMap::new();
        let mut type_blueprint = self.process_inheritance(context, |type_id, parent_type_id_list, context| {
            for parent_type_id in parent_type_id_list {
                let parent_blueprint = context.types.get_by_id(parent_type_id).unwrap();

                for method in parent_blueprint.methods.values() {
                    if method.owner_type_id == parent_type_id {
                        if methods.insert(method.name.to_string(), method.clone()).is_some() && method.owner_type_id == type_id {
                            context.errors.add(&self.name, format!("duplicate method `{}` (already declared by parent struct `{}`)", &self.name, &parent_blueprint.name));
                        }
                    }
                }
            }
        });

        type_blueprint.methods = methods;
    }
}