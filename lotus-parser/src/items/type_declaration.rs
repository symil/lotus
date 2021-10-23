use std::{collections::HashMap, hash::Hash, rc::Rc};
use indexmap::IndexMap;
use parsable::{DataLocation, parsable};
use crate::{program::{ActualTypeContent, AssociatedTypeInfo, DEFAULT_FUNC_NAME, DynamicMethodInfo, Error, FieldInfo, FuncRef, OBJECT_HEADER_SIZE, OBJECT_TYPE_NAME, ParentInfo, ProgramContext, THIS_TYPE_NAME, Type, TypeBlueprint, VI}, utils::Link, vasm};
use super::{AssociatedTypeDeclaration, EventCallbackQualifier, FieldDeclaration, FullType, Identifier, MethodDeclaration, StackType, StackTypeWrapped, TypeParameters, TypeQualifier, Visibility, VisibilityWrapper};

#[parsable]
pub struct TypeDeclaration {
    pub visibility: VisibilityWrapper,
    pub qualifier: TypeQualifier,
    #[parsable(brackets="()")]
    pub stack_type: Option<StackTypeWrapped>,
    pub name: Identifier,
    pub parameters: TypeParameters,
    #[parsable(prefix="extends")]
    pub parent: Option<FullType>,
    #[parsable(brackets="{}")]
    pub body: TypeDeclarationBody,
}

#[parsable]
#[derive(Default)]
pub struct TypeDeclarationBody {
    pub associated_types: Vec<AssociatedTypeDeclaration>,
    #[parsable(separator=",")]
    pub fields: Vec<FieldDeclaration>,
    pub methods: Vec<MethodDeclaration>
}

impl TypeDeclaration {
    pub fn process_name(&self, context: &mut ProgramContext) {
        let type_id = self.location.get_hash();
        let type_unwrapped = TypeBlueprint {
            type_id,
            name: self.name.clone(),
            visibility: self.visibility.value.unwrap_or(Visibility::Private),
            qualifier: self.qualifier,
            stack_type: self.stack_type.as_ref().and_then(|stack_type| Some(stack_type.value)).unwrap_or(StackType::Int),
            inheritance_chain_length: 0,
            descendants: vec![],
            ancestors: vec![],
            parameters: IndexMap::new(),
            associated_types: IndexMap::new(),
            self_type: Type::Undefined,
            parent: None,
            fields: IndexMap::new(),
            regular_methods: IndexMap::new(),
            static_methods: IndexMap::new(),
            dynamic_methods: vec![],
            hook_event_callbacks: IndexMap::new(),
            before_event_callbacks: IndexMap::new(),
            after_event_callbacks: IndexMap::new(),
        };
        
        if context.types.get_by_identifier(&self.name).is_some() {
            context.errors.add(&self.name, format!("duplicate type declaration: `{}`", &self.name));
        }

        let type_wrapped = context.types.insert(type_unwrapped, None);

        type_wrapped.with_mut(|mut type_unwrapped| {
            let parameters = self.parameters.process(context);

            type_unwrapped.parameters = parameters;
            type_unwrapped.self_type = Type::Actual(ActualTypeContent {
                type_blueprint: type_wrapped.clone(),
                parameters: type_unwrapped.parameters.values().map(|param| {
                    Type::TypeParameter(param.clone())
                }).collect(),
            })
        });
    }

    pub fn process_parent(&self, context: &mut ProgramContext) {
        self.process(context, |type_wrapped, context| {
            let mut result = None;

            if let Some(parsed_parent_type) = &self.parent {
                if let Some(parent_type) = parsed_parent_type.process(false, context) {
                    if self.qualifier == TypeQualifier::Type {
                        context.errors.add(parsed_parent_type, format!("regular types cannot inherit"));
                    }

                    match &parent_type {
                        Type::TypeParameter(_) => {
                            context.errors.add(parsed_parent_type, format!("cannot inherit from type parameter"));
                        },
                        Type::Actual(info) => {
                            let parent_unwrapped = info.type_blueprint.borrow();

                            match &parent_unwrapped.qualifier {
                                TypeQualifier::Type => {
                                    context.errors.add(parsed_parent_type, format!("cannot inherit from regular types"));
                                },
                                _ => {
                                    if parent_unwrapped.qualifier != self.qualifier {
                                        context.errors.add(parsed_parent_type, format!("`{}` types cannot inherit from `{}` types", self.qualifier, parent_unwrapped.qualifier));
                                    } else if self.qualifier != TypeQualifier::Type {
                                        result = Some(ParentInfo{
                                            location: parsed_parent_type.location.clone(),
                                            ty: parent_type.clone(),
                                        });
                                    }
                                }
                            };
                        },
                        _ => unreachable!()
                    }
                }
            } else if self.qualifier == TypeQualifier::Class {
                let base_object = context.types.get_by_name(OBJECT_TYPE_NAME).unwrap();

                if type_wrapped != base_object {
                    result = Some(ParentInfo {
                        location: DataLocation::default(),
                        ty: base_object.borrow().self_type.clone(),
                    });
                }
            }

            type_wrapped.with_mut(|mut type_unwrapped| {
                type_unwrapped.parent = result;
            });
        });
    }

    pub fn process_inheritance_chain(&self, context: &mut ProgramContext) -> usize {
        let mut chain_length = 0;

        self.process(context, |type_wrapped, context| {
            let mut types = vec![type_wrapped.clone()];
            let mut parent_opt = type_wrapped.borrow().parent.as_ref().and_then(|parent| Some(parent.ty.get_type_blueprint()));

            while let Some(parent_blueprint) = parent_opt {
                if types.contains(&parent_blueprint) {
                    if parent_blueprint == type_wrapped {
                        context.errors.add(&self.name, format!("circular inheritance: `{}`", &self.name));
                        type_wrapped.borrow_mut().parent = None;
                    }

                    parent_opt = None;
                } else {
                    types.push(parent_blueprint.clone());
                    parent_opt = parent_blueprint.borrow().parent.as_ref().and_then(|parent| Some(parent.ty.get_type_blueprint()));
                }
            }

            chain_length = types.len();

            type_wrapped.with_mut(|mut type_unwrapped| {
                type_unwrapped.inheritance_chain_length = chain_length;
            });
        });

        chain_length
    }

    pub fn compute_descendants(&self, context: &mut ProgramContext) {
        self.process(context, |type_wrapped, context| {
            type_wrapped.with_mut(|mut type_unwrapped| {
                type_unwrapped.descendants.insert(0, type_wrapped.clone());
            });

            type_wrapped.with_ref(|type_unwrapped| {
                if let Some(parent_info) = &type_unwrapped.parent {
                    parent_info.ty.get_type_blueprint().with_mut(|mut parent_unwrapped| {
                        for d in &type_unwrapped.descendants {
                            parent_unwrapped.descendants.push(d.clone());
                        }
                    });
                }
            });
        });
    }

    pub fn compute_ancestors(&self, context: &mut ProgramContext) {
        self.process(context, |type_wrapped, context| {
            let mut ancestors = vec![];

            type_wrapped.with_ref(|type_unwrapped| {
                ancestors.push(type_unwrapped.self_type.clone());

                if let Some(parent_info) = &type_unwrapped.parent {
                    parent_info.ty.get_type_blueprint().with_ref(|parent_unwrapped| {
                        for parent_ancestor in &parent_unwrapped.ancestors {
                            let ancestor = parent_ancestor.replace_parameters(Some(&parent_info.ty), &[]);

                            ancestors.push(ancestor);
                        }
                    });
                }
            });

            type_wrapped.with_mut(|mut type_unwrapped| {
                type_unwrapped.ancestors = ancestors;
            });
        });
    }

    pub fn process_associated_types(&self, context: &mut ProgramContext) {
        self.process(context, |type_wrapped, context| {
            let mut associated_types = IndexMap::new();

            type_wrapped.with_ref(|type_unwrapped| {
                if let Some(parent) = &type_unwrapped.parent {
                    parent.ty.get_type_blueprint().with_ref(|parent_unwrapped| {
                        for associated in parent_unwrapped.associated_types.values() {
                            let associatd_type_info = Rc::new(AssociatedTypeInfo {
                                owner: associated.owner.clone(),
                                name: associated.name.clone(),
                                ty: associated.ty.replace_parameters(Some(&parent.ty), &[]),
                                wasm_pattern: associated.wasm_pattern.clone(),
                            });

                            associated_types.insert(associatd_type_info.name.to_string(), associatd_type_info);
                        }
                    });
                }

                for associated_type in &self.body.associated_types {
                    let (name, ty) = associated_type.process(context);
                    let wasm_pattern = format!("<{}>", name);
                    let associatd_type_info = Rc::new(AssociatedTypeInfo {
                        owner: type_wrapped.clone(),
                        name: name.clone(),
                        ty,
                        wasm_pattern,
                    });

                    if associated_types.insert(associatd_type_info.name.to_string(), associatd_type_info).is_some() {
                        context.errors.add(&associated_type.name, format!("duplicate associated type `{}`", &name));
                    }

                    if name.as_str() == THIS_TYPE_NAME {
                        context.errors.add(&associated_type.name, format!("forbidden associated type name `{}`", THIS_TYPE_NAME));
                    }
                }
            });

            type_wrapped.with_mut(|mut type_unwrapped| {
                type_unwrapped.associated_types = associated_types;
            });
        });
    }

    pub fn process_fields(&self, context: &mut ProgramContext) {
        self.process(context, |type_wrapped, context| {
            let mut fields = IndexMap::new();
            let mut offset = OBJECT_HEADER_SIZE;

            type_wrapped.with_ref(|type_unwrapped| {
                if let Some(parent) = &type_unwrapped.parent {
                    parent.ty.get_type_blueprint().with_ref(|parent_unwrapped| {
                        for field_info in parent_unwrapped.fields.values() {
                            let field_details = Rc::new(FieldInfo {
                                owner: field_info.owner.clone(),
                                ty: field_info.ty.replace_parameters(Some(&parent.ty), &[]),
                                name: field_info.name.clone(),
                                offset,
                                default_value: vasm![]
                            });

                            offset += 1;
                            fields.insert(field_info.name.to_string(), field_details);
                        }
                    });
                }

                for field in &self.body.fields {
                    if fields.contains_key(field.name.as_str()) {
                        context.errors.add(&field.name, format!("duplicate field `{}`", &self.name));
                    }

                    if let Some(field_type) = field.ty.process(false, context) {
                        let field_details = Rc::new(FieldInfo {
                            owner: type_wrapped.clone(),
                            ty: field_type,
                            name: field.name.clone(),
                            offset,
                            default_value: vasm![]
                        });
                        
                        offset += 1;
                        fields.insert(field.name.to_string(), field_details);
                    }
                }
            });

            type_wrapped.with_mut(|mut type_unwrapped| {
                type_unwrapped.fields = fields;
            });
        });
    }

    pub fn process_method_signatures(&self, context: &mut ProgramContext) {
        self.process(context, |type_wrapped, context| {
            let mut regular_methods = IndexMap::new();
            let mut static_methods = IndexMap::new();
            
            type_wrapped.with_ref(|type_unwrapped| {
                if let Some(parent) = &type_unwrapped.parent {
                    parent.ty.get_type_blueprint().with_ref(|parent_unwrapped| {
                        for (name, func_ref) in parent_unwrapped.regular_methods.iter() {
                            regular_methods.insert(name.clone(), FuncRef {
                                function: func_ref.function.clone(),
                                this_type: func_ref.this_type.replace_parameters(Some(&parent.ty), &[]),
                            });
                        }
                    });

                    parent.ty.get_type_blueprint().with_ref(|parent_unwrapped| {
                        for (name, func_ref) in parent_unwrapped.static_methods.iter() {
                            static_methods.insert(name.clone(), FuncRef {
                                function: func_ref.function.clone(),
                                this_type: func_ref.this_type.replace_parameters(Some(&parent.ty), &[]),
                            });
                        }
                    });
                }
            });

            type_wrapped.with_mut(|mut type_unwrapped| {
                type_unwrapped.regular_methods = regular_methods;
                type_unwrapped.static_methods = static_methods;
            });

            for method in self.body.methods.iter().filter(|method| !method.is_autogen()) {
                method.process_signature(context);
            }
        });
    }

    pub fn process_autogen_method_signatures(&self, context: &mut ProgramContext) {
        self.process(context, |type_wrapped, context| {
            let children = type_wrapped.borrow().descendants.clone();

            for method in self.body.methods.iter().filter(|method| method.is_autogen()) {
                for child in &children {
                    context.current_type = Some(child.clone());
                    method.process_signature(context);
                }
            }
        });
    }

    pub fn process_fields_default_values(&self, context: &mut ProgramContext) {
        self.process(context, |type_wrapped, context| {
            let mut default_values = HashMap::new();

            type_wrapped.with_ref(|type_unwrapped| {
                if let Some(parent) = &type_unwrapped.parent {
                    parent.ty.get_type_blueprint().with_ref(|parent_unwrapped| {
                        for field_info in parent_unwrapped.fields.values() {
                            default_values.insert(field_info.name.to_string(), field_info.default_value.replace_type_parameters(&parent.ty));
                        }
                    });
                }

                for field in &self.body.fields {
                    if let Some(field_info) = type_unwrapped.fields.get(field.name.as_str()) {
                        let mut default_value_vasm = vasm![VI::call_static_method(&field_info.ty, DEFAULT_FUNC_NAME, &[], vec![], context)];

                        if let Some(default_value) = &field.default_value {
                            if let Some(vasm) = default_value.process(Some(&field_info.ty), context) {
                                if vasm.ty.is_assignable_to(&field_info.ty) {
                                    default_value_vasm = vasm;
                                } else {
                                    context.errors.add(default_value, format!("expected `{}`, got `{}`", &field_info.ty, &vasm.ty));
                                }
                            } 
                        };

                        default_values.insert(field_info.name.to_string(), default_value_vasm);
                    }
                }
            });

            type_wrapped.with_mut(|mut type_unwrapped| {
                for (name, default_value) in default_values.into_iter() {
                    let mut field_info = Rc::get_mut(type_unwrapped.fields.get_mut(&name).unwrap()).unwrap();

                    field_info.default_value = default_value;
                }
            });
        });
    }

    pub fn process_dynamic_methods(&self, context: &mut ProgramContext) {
        self.process(context, |type_wrapped, context| {
            let dynamic_methods = type_wrapped.with_ref(|type_unwrapped| {
                let mut result : Vec<FuncRef> = type_unwrapped.regular_methods.values()
                    .filter_map(|func_ref| match func_ref.function.borrow().is_dynamic {
                        true => Some(func_ref.clone()),
                        false => None,
                    })
                    .collect();
                
                result.sort_by_cached_key(|func_ref| func_ref.function.borrow().name.to_string());
                result.sort_by_cached_key(|func_ref| func_ref.function.borrow().owner_type.as_ref().unwrap().borrow().ancestors.len());

                result
            });

            for (i, func_ref) in dynamic_methods.iter().enumerate() {
                func_ref.function.with_mut(|mut function_unwrapped| {
                    if function_unwrapped.dynamic_index == -1 {
                        function_unwrapped.dynamic_index = i as i32;
                    } else if function_unwrapped.dynamic_index != i as i32 {
                        panic!("attempt to assign dynamic index {} to method {}, but it already has dynamic index {}", i, function_unwrapped.name.as_str(), function_unwrapped.dynamic_index);
                    }
                });
            }

            type_wrapped.with_mut(|mut type_unwrapped| {
                type_unwrapped.dynamic_methods = dynamic_methods;
            });
        });
    }

    pub fn process_method_bodies(&self, context: &mut ProgramContext) {
        self.process(context, |type_wrapped, context| {
            for method in self.body.methods.iter().filter(|method| !method.is_autogen()) {
                method.process_body(context);
            }
        });
    }

    pub fn process_autogen_method_bodies(&self, context: &mut ProgramContext) {
        self.process(context, |type_wrapped, context| {
            let children = type_wrapped.borrow().descendants.clone();

            for method in self.body.methods.iter().filter(|method| method.is_autogen()) {
                for child in &children {
                    context.current_type = Some(child.clone());
                    method.process_body(context);
                }
            }
        });
    }

    fn process<'a, F : FnMut(Link<TypeBlueprint>, &mut ProgramContext)>(&self, context: &mut ProgramContext, mut f: F) {
        let type_blueprint = context.types.get_by_location(&self.name, None);

        context.current_type = Some(type_blueprint.clone());
        f(type_blueprint, context);
        context.current_type = None;
    }

    // fn process_inheritance<'a, F : FnMut(Link<TypeBlueprint>, Vec<Link<TypeBlueprint>>, &mut ProgramContext)>(&self, context: &'a mut ProgramContext, mut f : F) {
    //     let type_blueprint = context.types.get_by_location(&self.name);
    //     let parent_type_list : Vec<Link<TypeBlueprint>> = type_blueprint.borrow().inheritance_chain.iter().map(|info| info.type_blueprint.clone()).collect();

    //     f(type_blueprint, parent_type_list, context);
    // }

    // pub fn process_fields_inheritance(&self, context: &mut ProgramContext) {
    //     self.process_inheritance(context, |type_blueprint, parent_type_list, context| {
    //         let mut fields = IndexMap::new();

    //         for parent_blueprint in parent_type_list {
    //             for field in parent_blueprint.borrow().fields.values() {
    //                 if field.owner == parent_blueprint {
    //                     let field_info = Rc::new(FieldDetails {
    //                         owner: field.owner.clone(),
    //                         name: field.name.clone(),
    //                         ty: field.ty.clone(),
    //                         offset: fields.len() + OBJECT_HEADER_SIZE
    //                     });

    //                     if fields.insert(field.name.to_string(), field_info).is_some() && &field.owner == &type_blueprint {
    //                         context.errors.add(&self.name, format!("duplicate field `{}` (already declared by parent struct `{}`)", &self.name, &parent_blueprint.borrow().name));
    //                     }
    //                 }
    //             }
    //         }

    //         type_blueprint.borrow_mut().fields = fields;
    //     });
    // }

    // pub fn process_methods_inheritance(&self, context: &mut ProgramContext) {
    //     self.process_inheritance(context, |type_wrapped, parent_type_list, context| {
    //         let mut methods = IndexMap::new();
    //         let mut static_methods = IndexMap::new();
    //         let mut dynamic_methods = vec![];

    //         for parent_blueprint in &parent_type_list {
    //             let parent_unwrapped = parent_blueprint.borrow();

    //             for method_blueprint in parent_unwrapped.regular_methods.values().chain(parent_unwrapped.static_methods.values()) {
    //                 let method_unwrapped = method_blueprint.borrow();
    //                 let is_static = method_unwrapped.is_static();
    //                 let is_dynamic = method_unwrapped.is_dynamic;
    //                 let owner = method_unwrapped.owner_type.as_ref().unwrap();
    //                 let name = &method_unwrapped.name;

    //                 if owner == parent_blueprint {
    //                     let (mut indexmap, s) = match is_static {
    //                         true => (&mut static_methods, "static "),
    //                         false => (&mut methods, ""),
    //                     };

    //                     if let Some(previous_method) = indexmap.insert(name.to_string(), method_blueprint.clone()) {
    //                         if owner == &type_wrapped && (!previous_method.borrow().is_dynamic || !is_dynamic) {
    //                             context.errors.add(&self.name, format!("duplicate {}method `{}` (already declared by parent struct `{}`)", s, &self.name, &parent_blueprint.borrow().name));
    //                         }
    //                     }
    //                 }
    //             }
    //         }

    //         for method_wrapped in methods.values() {
    //             let ok = method_wrapped.with_mut(|mut method_unwrapped| {
    //                 if method_unwrapped.is_dynamic {
    //                     let index = dynamic_methods.len() as i32;

    //                     if method_unwrapped.dynamic_index == -1 {
    //                         method_unwrapped.dynamic_index = index;
    //                         return true;
    //                     } else if method_unwrapped.dynamic_index != index {
    //                         panic!("method `{}` had dynamic index `{}`, tried to assign `{}`", &method_unwrapped.name, method_unwrapped.dynamic_index, index);
    //                     } else {
    //                         return true;
    //                     }
    //                 }

    //                 false
    //             });

    //             if ok {
    //                 dynamic_methods.push(method_wrapped.clone());
    //             }
    //         }

    //         type_wrapped.borrow_mut().regular_methods = methods;
    //         type_wrapped.borrow_mut().static_methods = static_methods;
    //         type_wrapped.borrow_mut().dynamic_methods = dynamic_methods;
    //     });
    // }
}