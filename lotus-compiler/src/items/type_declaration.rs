use std::{collections::{HashMap, HashSet}, fmt::format, hash::Hash, rc::Rc, slice::Iter};
use colored::Colorize;
use enum_iterator::IntoEnumIterator;
use indexmap::{IndexMap, IndexSet};
use parsable::{DataLocation, parsable};
use crate::{program::{ActualTypeContent, AssociatedTypeInfo, BUILTIN_DEFAULT_METHOD_NAME, BuiltinType, DEFAULT_METHOD_NAME, DESERIALIZE_DYN_METHOD_NAME, DynamicMethodInfo, ENUM_TYPE_NAME, EVENT_CALLBACKS_GLOBAL_NAME, EnumVariantInfo, FieldInfo, FuncRef, FunctionBlueprint, FunctionCall, NONE_METHOD_NAME, NamedFunctionCallDetails, OBJECT_HEADER_SIZE, OBJECT_TYPE_NAME, ParentInfo, ProgramContext, ScopeKind, Signature, SELF_TYPE_NAME, Type, TypeBlueprint, TypeCategory, VI, WasmStackType, hashmap_get_or_insert_with, MainType}, utils::Link, vasm};
use super::{AssociatedTypeDeclaration, EventCallbackQualifier, FieldDeclaration, ParsedType, Identifier, MethodDeclaration, StackType, StackTypeWrapped, TypeParameters, TypeQualifier, Visibility, VisibilityWrapper};

#[parsable]
pub struct TypeDeclaration {
    pub visibility: VisibilityWrapper,
    pub qualifier: TypeQualifier,
    #[parsable(brackets="()")]
    pub stack_type: Option<Identifier>,
    pub name: Identifier,
    pub parameters: TypeParameters,
    #[parsable(prefix="extends")]
    pub parent: Option<ParsedType>,
    #[parsable(brackets="{}")]
    pub body: Vec<TypeItem>,
}

#[parsable]
pub enum TypeItem {
    AssociatedTypeDeclaration(AssociatedTypeDeclaration),
    MethodDeclaration(MethodDeclaration),
    FieldDeclaration(FieldDeclaration)
}

impl TypeDeclaration {
    fn get_associated_types(&self) -> Vec<&AssociatedTypeDeclaration> {
        self.body.iter().filter_map(|item| match item {
            TypeItem::AssociatedTypeDeclaration(value) => Some(value),
            _ => None,
        }).collect()
    }

    fn get_fields(&self) -> Vec<&FieldDeclaration> {
        self.body.iter().filter_map(|item| match item {
            TypeItem::FieldDeclaration(value) => Some(value),
            _ => None,
        }).collect()
    }

    fn get_methods(&self) -> Vec<&MethodDeclaration> {
        self.body.iter().filter_map(|item| match item {
            TypeItem::MethodDeclaration(value) => Some(value),
            _ => None,
        }).collect()
    }

    pub fn process_name(&self, index: usize, context: &mut ProgramContext) {
        let type_id = self.location.get_hash();
        let mut type_unwrapped = TypeBlueprint {
            declaration_index: index,
            type_id,
            name: self.name.clone(),
            visibility: self.visibility.value.unwrap_or(Visibility::Private),
            category: self.qualifier.to_type_category(),
            stack_type: WasmStackType::Fixed(StackType::Void),
            descendants: vec![],
            ancestors: vec![],
            parameters: IndexMap::new(),
            associated_types: IndexMap::new(),
            self_type: Type::Undefined,
            parent: None,
            enum_variants: IndexMap::new(),
            fields: IndexMap::new(),
            regular_methods: IndexMap::new(),
            static_methods: IndexMap::new(),
            dynamic_methods: vec![],
            event_callbacks: HashMap::new(),
        };

        for main_type in MainType::into_enum_iter() {
            if self.name.as_str() == main_type.get_name() {
                type_unwrapped.visibility = Visibility::Export;
            }
        }
        
        if context.types.get_by_identifier(&self.name).is_some() {
            context.errors.generic(&self.name, format!("duplicate type declaration: `{}`", &self.name));
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
                        context.errors.generic(name, format!("undefined type parameter `{}`", other.bold()));
                        WasmStackType::Fixed(StackType::Int)
                    },
                }
            },
            None => WasmStackType::Fixed(StackType::Int),
        };

        context.declare_shared_identifier(&self.name, Some(&self.name), None);

        for details in parameters.values() {
            context.declare_shared_identifier(&details.name, Some(&details.name), None);
        }

        type_wrapped.with_mut(|mut type_unwrapped| {
            type_unwrapped.self_type = Type::Actual(ActualTypeContent {
                type_blueprint: type_wrapped.clone(),
                parameters: parameters.values().map(|param| {
                    Type::TypeParameter(param.clone())
                }).collect(),
                location: self.name.location.clone()
            });
            type_unwrapped.parameters = parameters;
            type_unwrapped.stack_type = stack_type;
        });
    }

    pub fn compute_type_dependencies(&self, context: &mut ProgramContext) -> IndexSet<Link<TypeBlueprint>> {
        let mut list = vec![];

        match &self.parent {
            Some(parent) => parent.collected_instancied_type_names(&mut list, context),
            None => match self.name.as_str() == OBJECT_TYPE_NAME || self.qualifier.to_type_category() != TypeCategory::Class {
                true => {},
                false => {
                    list.push(Identifier::unlocated(OBJECT_TYPE_NAME))
                },
            },
        };

        for field_declaration in self.get_fields() {
            match &field_declaration.default_value {
                Some(default_value) => default_value.collected_instancied_type_names(&mut list, context),
                None => {
                    if let Some(ty) = &field_declaration.ty {
                        ty.collected_instancied_type_names(&mut list, context);
                    }
                }
            }
        }

        let mut dependancies = IndexSet::new();

        for identifier in list {
            if let Some(type_blueprint) = context.types.get_by_identifier(&identifier) {
                dependancies.insert(type_blueprint);
            }
        }

        dependancies
    }

    pub fn process_parent(&self, context: &mut ProgramContext) {
        self.process(context, |type_wrapped, context| {
            let mut result = None;

            if let Some(parsed_parent_type) = &self.parent {
                if let Some(parent_type) = parsed_parent_type.process(false, context) {
                    if !type_wrapped.borrow().is_class() {
                        context.errors.generic(parsed_parent_type, format!("only class types can inherit"));
                    } else {
                        match &parent_type {
                            Type::TypeParameter(_) => {
                                context.errors.generic(parsed_parent_type, format!("cannot inherit from type parameter"));
                            },
                            Type::Actual(info) => {
                                let parent_unwrapped = info.type_blueprint.borrow();

                                if parent_unwrapped.is_class() {
                                    result = Some(ParentInfo{
                                        location: parsed_parent_type.location.clone(),
                                        ty: parent_type.clone(),
                                    });
                                } else {
                                    context.errors.generic(parsed_parent_type, format!("cannot inherit from non-class types"));
                                }
                            },
                            _ => unreachable!()
                        }
                    }
                }
            } else if let Some(inherited_type) = self.qualifier.get_inherited_type() {
                let parent_type_wrapped = context.types.get_by_name(inherited_type.get_name()).unwrap();

                if parent_type_wrapped != type_wrapped {
                    result = Some(ParentInfo {
                        location: DataLocation::default(),
                        ty: parent_type_wrapped.borrow().self_type.clone(),
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

                for associated_type in self.get_associated_types() {
                    let (name, ty) = associated_type.process(context);
                    let wasm_pattern = format!("<{}>", name);
                    let associatd_type_info = Rc::new(AssociatedTypeInfo {
                        owner: type_wrapped.clone(),
                        name: name.clone(),
                        ty,
                        wasm_pattern,
                    });

                    context.declare_shared_identifier(&name, Some(&name), Some(&associatd_type_info.ty));

                    if associated_types.insert(associatd_type_info.name.to_string(), associatd_type_info).is_some() {
                        context.errors.generic(&associated_type.name, format!("duplicate associated type `{}`", &name));
                    }

                    if name.as_str() == SELF_TYPE_NAME {
                        context.errors.generic(&associated_type.name, format!("forbidden associated type name `{}`", SELF_TYPE_NAME));
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
            let mut variants = IndexMap::new();
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

                for field in self.get_fields() {
                    match &field.ty {
                        Some(ty) => {
                            // Regular field

                            if !type_unwrapped.is_class() {
                                context.errors.generic(&field.name, format!("only classes can have fields"));
                                continue;
                            }

                            if fields.contains_key(field.name.as_str()) {
                                context.errors.generic(&field.name, format!("duplicate field `{}`", self.name.as_str().bold()));
                            }

                            if let Some(field_type) = ty.process(false, context) {
                                context.declare_shared_identifier(&field.name, Some(&field.name), Some(&field_type));

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
                        },
                        None => {
                            // Enum variant

                            context.declare_shared_identifier(&field.name, Some(&field.name), None);

                            if !type_unwrapped.is_enum() {
                                context.errors.generic(&field.name, format!("only enums can have variants"));
                                continue;
                            }

                            if variants.contains_key(field.name.as_str()) {
                                context.errors.generic(&field.name, format!("duplicate variant `{}`", self.name.as_str().bold()));
                            }

                            let variant_details = Rc::new(EnumVariantInfo {
                                name: field.name.clone(),
                                value: variants.len(),
                            });

                            variants.insert(field.name.to_string(), variant_details);
                        },
                    }
                }
            });

            type_wrapped.with_mut(|mut type_unwrapped| {
                type_unwrapped.fields = fields;
                type_unwrapped.enum_variants = variants;
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

            for method in self.get_methods().iter().filter(|method| !method.is_autogen()) {
                method.process_signature(context);
            }

            type_wrapped.with_mut(|mut type_unwrapped| {
                if let Some(parent) = &type_unwrapped.parent {
                    parent.ty.get_type_blueprint().with_ref(|parent_unwrapped| {
                        for (event_type_wrapped, callback_list) in parent_unwrapped.event_callbacks.iter() {
                            let self_callback_list = hashmap_get_or_insert_with(&mut type_unwrapped.event_callbacks, event_type_wrapped, || vec![]);

                            for callback in callback_list {
                                self_callback_list.push(callback.clone());
                            }
                        }
                    });
                }
            });
        });
    }

    pub fn process_autogen_method_signatures(&self, context: &mut ProgramContext) {
        self.process(context, |type_wrapped, context| {
            let children = type_wrapped.borrow().descendants.clone();

            context.autogen_type = Some(type_wrapped);

            for method in self.get_methods().iter().filter(|method| method.is_autogen()) {
                for child in &children {
                    context.push_scope(ScopeKind::Type(child.clone()));
                    method.process_signature(context);
                    context.pop_scope();
                }
            }

            context.autogen_type = None;
        });
    }

    pub fn process_fields_default_values(&self, context: &mut ProgramContext) {
        self.process(context, |type_wrapped, context| {
            if !type_wrapped.borrow().is_class() {
                return;
            }
            
            let mut default_values = HashMap::new();

            type_wrapped.with_ref(|type_unwrapped| {
                if let Some(parent) = &type_unwrapped.parent {
                    parent.ty.get_type_blueprint().with_ref(|parent_unwrapped| {
                        for field_info in parent_unwrapped.fields.values() {
                            default_values.insert(field_info.name.to_string(), field_info.default_value.clone());
                        }
                    });
                }

                for field in self.get_fields() {
                    if field.ty.is_none() {
                        continue;
                    }

                    if let Some(field_info) = type_unwrapped.fields.get(field.name.as_str()) {
                        let mut default_value_vasm = vasm![];

                        if let Some(default_value) = &field.default_value {
                            if let Some(vasm) = default_value.process(Some(&field_info.ty), context) {
                                if vasm.ty.is_assignable_to(&field_info.ty) {
                                    let function_blueprint = FunctionBlueprint {
                                        function_id: field_info.name.location.get_hash(),
                                        name: Identifier::unique(&format!("{}_{}_default", self.name.as_str(), field_info.name.as_str())),
                                        visibility: Visibility::None,
                                        parameters: IndexMap::new(),
                                        argument_names: vec![],
                                        signature: Signature {
                                            this_type: None,
                                            argument_types: vec![],
                                            return_type: field_info.ty.clone(),
                                        },
                                        argument_variables: vec![],
                                        owner_type: Some(type_wrapped.clone()),
                                        owner_interface: None,
                                        closure_details: None,
                                        method_details: None,
                                        is_raw_wasm: false,
                                        body: vasm,
                                    };

                                    default_value_vasm = vasm![VI::call_function(FunctionCall::Named(NamedFunctionCallDetails {
                                        caller_type: None,
                                        function: Link::new(function_blueprint),
                                        parameters: vec![]
                                    }), vec![])];
                                } else {
                                    context.errors.type_mismatch(default_value, &field_info.ty, &vasm.ty);
                                }
                            }
                        } else if field.ty.as_ref().unwrap().is_option() {
                            default_value_vasm = vasm![VI::call_static_method(&field_info.ty, NONE_METHOD_NAME, &[], vec![], context)];
                        } else if let Some(_) = field_info.ty.get_static_method(DEFAULT_METHOD_NAME, context) {
                            default_value_vasm = vasm![VI::call_static_method(&field_info.ty, DEFAULT_METHOD_NAME, &[], vec![], context)];
                        } else {
                            default_value_vasm = vasm![VI::call_static_method(&field_info.ty, BUILTIN_DEFAULT_METHOD_NAME, &[], vec![], context)];
                        }

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
                result.sort_by_cached_key(|func_ref| func_ref.function.borrow().method_details.as_ref().unwrap().first_declared_by.as_ref().unwrap().borrow().ancestors.len());

                result
            });

            // if self.name.is("Object") || self.name.is("Foo") {
            //     println!("=> {}", &self.name);
            //     for func_ref in dynamic_methods.iter() {
            //         println!("{}", func_ref.function.borrow().name);
            //     }
            // }

            for (i, func_ref) in dynamic_methods.iter().enumerate() {
                func_ref.function.with_mut(|mut function_unwrapped| {
                    let mut method_details = function_unwrapped.method_details.as_mut().unwrap();
                    let dynamic_index = method_details.dynamic_index.unwrap();

                    if dynamic_index == -1 {
                        method_details.dynamic_index = Some(i as i32);
                    } else if dynamic_index != i as i32 {
                        panic!("attempt to assign dynamic index {} to method `{}`, but it already has dynamic index {}", i, function_unwrapped.name.as_str().bold(), dynamic_index);
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
            for method in self.get_methods().iter().filter(|method| !method.is_autogen()) {
                method.process_body(context);
            }
        });
    }

    pub fn process_autogen_method_bodies(&self, context: &mut ProgramContext) {
        self.process(context, |type_wrapped, context| {
            let children = type_wrapped.borrow().descendants.clone();

            context.autogen_type = Some(type_wrapped);

            for method in self.get_methods().iter().filter(|method| method.is_autogen()) {
                for child in &children {
                    context.push_scope(ScopeKind::Type(child.clone()));
                    method.process_body(context);
                    context.pop_scope();
                }
            }

            context.autogen_type = None;
        });
    }

    fn process<'a, F : FnMut(Link<TypeBlueprint>, &mut ProgramContext)>(&self, context: &mut ProgramContext, mut f: F) {
        let type_blueprint = context.types.get_by_location(&self.name, None);

        context.push_scope(ScopeKind::Type(type_blueprint.clone()));
        f(type_blueprint, context);
        context.pop_scope();
    }
}