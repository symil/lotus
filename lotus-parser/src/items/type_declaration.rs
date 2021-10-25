use std::{collections::HashMap, hash::Hash, rc::Rc};
use colored::Colorize;
use indexmap::IndexMap;
use parsable::{DataLocation, parsable};
use crate::{program::{ActualTypeContent, AssociatedTypeInfo, DEFAULT_FUNC_NAME, DynamicMethodInfo, Error, FieldInfo, FuncRef, OBJECT_HEADER_SIZE, OBJECT_TYPE_NAME, ParentInfo, ProgramContext, THIS_TYPE_NAME, Type, TypeBlueprint, VI, WasmStackType}, utils::Link, vasm};
use super::{AssociatedTypeDeclaration, EventCallbackQualifier, FieldDeclaration, FullType, Identifier, MethodDeclaration, StackType, StackTypeWrapped, TypeParameters, TypeQualifier, Visibility, VisibilityWrapper};

#[parsable]
pub struct TypeDeclaration {
    pub visibility: VisibilityWrapper,
    pub qualifier: TypeQualifier,
    #[parsable(brackets="()")]
    pub stack_type: Option<Identifier>,
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
    pub fn process_name(&self, index: usize, context: &mut ProgramContext) {
        let type_id = self.location.get_hash();
        let type_unwrapped = TypeBlueprint {
            declaration_index: index,
            type_id,
            name: self.name.clone(),
            visibility: self.visibility.value.unwrap_or(Visibility::Private),
            qualifier: self.qualifier,
            stack_type: WasmStackType::Fixed(StackType::Void),
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
        let parameters = self.parameters.process(context);
        let stack_type = match &self.stack_type {
            Some(name) => match name.as_str() {
                "i32" => WasmStackType::Fixed(StackType::Int),
                "f32" => WasmStackType::Fixed(StackType::Float),
                "void" => WasmStackType::Fixed(StackType::Void),
                other => match parameters.get_index_of(other) {
                    Some(index) => WasmStackType::TypeParameter(index),
                    None => {
                        context.errors.add(name, format!("undefined type parameter `{}`", other.bold()));
                        WasmStackType::Fixed(StackType::Int)
                    },
                }
            },
            None => WasmStackType::Fixed(StackType::Int),
        };

        type_wrapped.with_mut(|mut type_unwrapped| {
            type_unwrapped.self_type = Type::Actual(ActualTypeContent {
                type_blueprint: type_wrapped.clone(),
                parameters: parameters.values().map(|param| {
                    Type::TypeParameter(param.clone())
                }).collect(),
            });
            type_unwrapped.parameters = parameters;
            type_unwrapped.stack_type = stack_type;
        });
    }

    pub fn compute_type_dependencies(&self, context: &ProgramContext) -> Vec<Link<TypeBlueprint>> {
        let mut list = vec![];

        match &self.parent {
            Some(parent) => parent.collected_instancied_type_names(&mut list),
            None => match self.name.as_str() == OBJECT_TYPE_NAME || self.qualifier != TypeQualifier::Class {
                true => {},
                false => {
                    list.push(Identifier::unlocated(OBJECT_TYPE_NAME))
                },
            },
        };

        for field_declaration in &self.body.fields {
            match &field_declaration.default_value {
                Some(default_value) => default_value.collected_instancied_type_names(&mut list),
                None => {field_declaration.ty.collected_instancied_type_names(&mut list)}
            }
        }

        let mut dependancies = vec![];

        for identifier in list {
            if let Some(type_blueprint) = context.types.get_by_identifier(&identifier) {
                dependancies.push(type_blueprint);
            }
        }

        dependancies
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
                            default_values.insert(field_info.name.to_string(), field_info.default_value.replace_type_parameters(&parent.ty, self.location.get_hash()));
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

                        default_value_vasm.ty = field_info.ty.clone();

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
                    .filter_map(|func_ref| match func_ref.function.borrow().is_dynamic() {
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
}