use std::{collections::HashMap, hash::Hash};
use indexmap::IndexMap;
use parsable::parsable;
use crate::{program::{ActualTypeInfo, AssociatedType, Error, FieldDetails, OBJECT_HEADER_SIZE, ProgramContext, THIS_TYPE_NAME, Type, TypeBlueprint}, utils::Link};
use super::{AssociatedTypeDeclaration, EventCallbackQualifier, FieldDeclaration, FullType, Identifier, MethodDeclaration, StackType, StackTypeToken, TypeParameters, TypeQualifier, Visibility, VisibilityWrapper};

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
    pub associated_types: Vec<AssociatedTypeDeclaration>,
    pub fields: Vec<FieldDeclaration>,
    pub methods: Vec<MethodDeclaration>
}

impl TypeDeclaration {
    pub fn process_name(&self, context: &mut ProgramContext) -> u64 {
        let type_id = self.location.get_hash();
        let type_blueprint = TypeBlueprint {
            type_id,
            name: self.name.clone(),
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
            dynamic_methods: vec![],
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

    pub fn process_associated_types(&self, context: &mut ProgramContext) {
        self.process(context, |type_blueprint, context| {
            let mut associated_types = IndexMap::new();

            for associated_type in &self.body.associated_types {
                let (name, ty) = associated_type.process(context);
                let item = AssociatedType {
                    owner: type_blueprint.clone(),
                    name: name.clone(),
                    value: ty,
                };

                if associated_types.insert(name.to_string(), item).is_some() {
                    context.errors.add(&associated_type.name, format!("duplicate associated type `{}`", &name));
                }
                
                if name.as_str() == THIS_TYPE_NAME {
                    context.errors.add(&associated_type.name, format!("forbidden associated type name `{}`", THIS_TYPE_NAME));
                }
            }

            associated_types.insert(THIS_TYPE_NAME.to_string(), AssociatedType {
                owner: type_blueprint.clone(),
                name: Identifier::new(THIS_TYPE_NAME, &self.name),
                value: Type::Actual(type_blueprint.get_info()),
            });

            type_blueprint.borrow_mut().associated_types = associated_types;
        });
    }

    pub fn process_parent(&self, context: &mut ProgramContext) {
        self.process(context, |type_blueprint, context| {
            let mut result = None;

            if let Some(parsed_parent_type) = &self.parent {
                if let Some(parent_type) = parsed_parent_type.process(context) {
                    if self.qualifier == TypeQualifier::Type {
                        context.errors.add(parsed_parent_type, format!("regular types cannot inherit"));
                    }

                    match parent_type {
                        Type::Parameter(_) => {
                            context.errors.add(parsed_parent_type, format!("cannot inherit from generic parameter"));
                        },
                        Type::Actual(info) => {
                            let parent_blueprint = info.type_blueprint.borrow();

                            match &parent_blueprint.qualifier {
                                TypeQualifier::Type => {
                                    context.errors.add(parsed_parent_type, format!("cannot inherit from regular types"));
                                },
                                _ => {
                                    if parent_blueprint.qualifier != self.qualifier {
                                        context.errors.add(parsed_parent_type, format!("`{}` types cannot inherit from `{}` types", self.qualifier, parent_blueprint.qualifier));
                                    } else if self.qualifier != TypeQualifier::Type {
                                        result = Some(info);
                                    }
                                }
                            };
                        },
                        _ => unreachable!()
                    }
                }
            }

            type_blueprint.borrow_mut().parent = result;
        });
    }

    pub fn process_inheritance_chain(&self, context: &mut ProgramContext) {
        self.process(context, |type_blueprint, context| {
            let mut types = vec![type_blueprint.get_info()];
            let mut parent_opt = &type_blueprint.borrow().parent;

            while let Some(parent_info) = parent_opt {
                if types.iter().any(|info| info.type_blueprint == parent_info.type_blueprint) {
                    if &parent_info.type_blueprint == type_blueprint {
                        context.errors.add(&self.name, format!("circular inheritance: `{}`", &self.name));
                    }
                } else {
                    types.push(parent_info.clone());
                    parent_opt = &parent_info.type_blueprint.borrow().parent;
                }
            }

            types.reverse();

            type_blueprint.borrow_mut().inheritance_chain = types;
        });
    }

    pub fn process_fields(&self, context: &mut ProgramContext) {
        self.process(context, |type_blueprint, context| {
            let mut fields = IndexMap::new();

            for field in &self.body.fields {
                if fields.contains_key(field.name.as_str()) {
                    context.errors.add(&field.name, format!("duplicate field `{}`", &self.name));
                }

                if let Some(field_type) = field.ty.process(context) {
                    let field_details = FieldDetails {
                        owner: type_blueprint.clone(),
                        ty: field_type,
                        name: field.name.clone(),
                        offset: 0,
                    };

                    fields.insert(field.name.to_string(), field_details);
                }
            }

            type_blueprint.borrow_mut().fields = fields;
        });
    }

    pub fn process_method_signatures(&self, context: &mut ProgramContext) {
        self.process(context, |type_blueprint, context| {
            for method in self.body.methods.iter() {
                method.process_signature(context);
            }
        });
    }

    pub fn process_methods_bodies(&self, context: &mut ProgramContext) {
        self.process(context, |type_blueprint, context| {
            for method in self.body.methods.iter() {
                method.process_body(context);
            }
        });
    }

    fn process<'a, F : FnMut(&Link<TypeBlueprint>, &mut ProgramContext)>(&self, context: &mut ProgramContext, f : F) {
        let type_blueprint = context.types.get_by_location(&self.name);

        context.current_type = Some(type_blueprint.clone());
        f(type_blueprint, context);
        context.current_type = None;
    }

    fn process_inheritance<'a, F : FnMut(&Link<TypeBlueprint>, Vec<Link<TypeBlueprint>>, &mut ProgramContext)>(&self, context: &'a mut ProgramContext, f : F) {
        let type_blueprint = context.types.get_by_location(&self.name);
        let parent_type_list : Vec<Link<TypeBlueprint>> = type_blueprint.borrow().inheritance_chain.iter().map(|info| info.type_blueprint.clone()).collect();

        f(type_blueprint, parent_type_list, context);
    }

    pub fn process_fields_inheritance(&self, context: &mut ProgramContext) {
        self.process_inheritance(context, |type_blueprint, parent_type_list, context| {
            let mut fields : IndexMap<String, FieldDetails> = IndexMap::new();

            for parent_blueprint in parent_type_list {
                for field in parent_blueprint.borrow().fields.values() {
                    if field.owner == parent_blueprint {
                        let field_info = FieldDetails {
                            owner: field.owner.clone(),
                            name: field.name.clone(),
                            ty: field.ty.clone(),
                            offset: fields.len() + OBJECT_HEADER_SIZE
                        };

                        if fields.insert(field.name.to_string(), field_info).is_some() && &field.owner == type_blueprint {
                            context.errors.add(&self.name, format!("duplicate field `{}` (already declared by parent struct `{}`)", &self.name, &parent_blueprint.borrow().name));
                        }
                    }
                }
            }

            type_blueprint.borrow_mut().fields = fields;
        });
    }

    pub fn process_methods_inheritance(&self, context: &mut ProgramContext) {
        self.process_inheritance(context, |type_blueprint, parent_type_list, context| {
            let mut methods = IndexMap::new();
            let mut static_methods = IndexMap::new();
            let mut dynamic_methods = vec![];

            for parent_blueprint in &parent_type_list {
                let parent_unwrapped = parent_blueprint.borrow();

                for method_blueprint in parent_unwrapped.methods.values().chain(parent_unwrapped.static_methods.values()) {
                    let method_unwrapped = method_blueprint.borrow();
                    let is_static = method_unwrapped.is_static();
                    let is_dynamic = method_unwrapped.is_dynamic;
                    let owner = method_unwrapped.owner_type.as_ref().unwrap();
                    let name = &method_unwrapped.name;

                    if owner == parent_blueprint {
                        let (mut indexmap, s) = match is_static {
                            true => (&mut static_methods, "static "),
                            false => (&mut methods, ""),
                        };

                        if let Some(previous_method) = indexmap.insert(name.to_string(), method_blueprint.clone()) {
                            if owner == type_blueprint && (!previous_method.borrow().is_dynamic || !is_dynamic) {
                                context.errors.add(&self.name, format!("duplicate {}method `{}` (already declared by parent struct `{}`)", s, &self.name, &parent_blueprint.borrow().name));
                            }
                        }
                    }
                }
            }

            for method_blueprint in methods.values() {
                let ok = method_blueprint.with_mut(|method| {
                    if method.is_dynamic {
                        let index = dynamic_methods.len() as i32;

                        if method.dynamic_index == -1 {
                            method.dynamic_index = index;
                            return true;
                        } else if method.dynamic_index != index {
                            panic!("method `{}` had dynamic index `{}`, tried to assign `{}`", &method.name, method.dynamic_index, index);
                        } else {
                            return true;
                        }
                    }

                    false
                });

                if ok {
                    dynamic_methods.push(method_blueprint.clone());
                }
            }

            type_blueprint.with_mut(|t| {
                t.methods = methods;
                t.static_methods = static_methods;
                t.dynamic_methods = dynamic_methods;
            });
        });
    }
}