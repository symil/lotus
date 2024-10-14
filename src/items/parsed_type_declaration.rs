use std::{collections::{HashMap, HashSet}, fmt::format, hash::Hash, rc::Rc, slice::Iter};
use colored::Colorize;
use enum_iterator::IntoEnumIterator;
use indexmap::{IndexMap, IndexSet};
use parsable::{ItemLocation, parsable};
use crate::{program::{ActualTypeContent, AssociatedTypeInfo, DEFAULT_METHOD_NAME, BuiltinType, DESERIALIZE_DYN_METHOD_NAME, DynamicMethodInfo, ENUM_TYPE_NAME, EVENT_CALLBACKS_GLOBAL_NAME, EnumVariantInfo, FieldInfo, FuncRef, FunctionBlueprint, FunctionCall, NONE_METHOD_NAME, NamedFunctionCallDetails, OBJECT_HEADER_SIZE, OBJECT_TYPE_NAME, ParentInfo, ProgramContext, ScopeKind, Signature, SELF_TYPE_NAME, Type, TypeBlueprint, TypeCategory, WasmStackType, hashmap_get_or_insert_with, MainType, TypeContent, Visibility, FunctionBody, SELF_VAR_NAME, FieldVisibility, ANY_TYPE_NAME, ArgumentInfo, FunctionKind}, utils::Link};
use super::{ParsedAssociatedTypeDeclaration, ParsedEventCallbackQualifierKeyword, ParsedFieldDeclaration, ParsedType, Identifier, ParsedMethodDeclaration, ParsedTypeParameters, ParsedTypeQualifier, ParsedVisibilityToken, ParsedVisibility, ParsedEventCallbackDeclaration, ParsedSuperFieldDefaultValue, ParsedTypeExtend, ParsedStackTypeDeclaration};

#[parsable]
pub struct ParsedTypeDeclaration {
    pub visibility: Option<ParsedVisibility>,
    pub qualifier: ParsedTypeQualifier,
    pub stack_type: Option<ParsedStackTypeDeclaration>,
    pub name: Identifier,
    pub parameters: Option<ParsedTypeParameters>,
    pub parent: Option<ParsedTypeExtend>,
    pub body: Option<ParsedTypeDeclarationBody>
}

#[parsable]
pub struct ParsedTypeDeclarationBody {
    #[parsable(brackets="{}")]
    pub items: Vec<ParsedTypeDeclarationBodyItem>,
}

#[parsable]
pub enum ParsedTypeDeclarationBodyItem {
    EventCallbackDeclaration(ParsedEventCallbackDeclaration),
    AssociatedTypeDeclaration(ParsedAssociatedTypeDeclaration),
    SuperFieldDefaultValue(ParsedSuperFieldDefaultValue),
    MethodDeclaration(ParsedMethodDeclaration),
    FieldDeclaration(ParsedFieldDeclaration),
}

impl ParsedTypeDeclaration {
    fn get_body_items(&self) -> &[ParsedTypeDeclarationBodyItem] {
        match &self.body {
            Some(body) => &body.items,
            None => &[],
        }
    }

    fn get_associated_types(&self) -> Vec<&ParsedAssociatedTypeDeclaration> {
        self.get_body_items().iter().filter_map(|item| match item {
            ParsedTypeDeclarationBodyItem::AssociatedTypeDeclaration(value) => Some(value),
            _ => None,
        }).collect()
    }

    fn get_super_fields_default_values(&self) -> Vec<&ParsedSuperFieldDefaultValue> {
        self.get_body_items().iter().filter_map(|item| match item {
            ParsedTypeDeclarationBodyItem::SuperFieldDefaultValue(value) => Some(value),
            _ => None,
        }).collect()
    }

    fn get_fields(&self) -> Vec<&ParsedFieldDeclaration> {
        self.get_body_items().iter().filter_map(|item| match item {
            ParsedTypeDeclarationBodyItem::FieldDeclaration(value) => Some(value),
            _ => None,
        }).collect()
    }

    fn get_methods(&self) -> Vec<&ParsedMethodDeclaration> {
        self.get_body_items().iter().filter_map(|item| match item {
            ParsedTypeDeclarationBodyItem::MethodDeclaration(value) => Some(value),
            _ => None,
        }).collect()
    }

    fn get_event_callbacks(&self) -> Vec<&ParsedEventCallbackDeclaration> {
        self.get_body_items().iter().filter_map(|item| match item {
            ParsedTypeDeclarationBodyItem::EventCallbackDeclaration(value) => Some(value),
            _ => None,
        }).collect()
    }

    pub fn process_name(&self, index: usize, context: &mut ProgramContext) {
        let type_id = self.location.get_hash();
        let category = self.qualifier.to_type_category();
        let mut type_unwrapped = TypeBlueprint {
            declaration_index: index,
            type_id,
            name: self.name.clone(),
            visibility: ParsedVisibility::process_or(&self.visibility, Visibility::Private),
            category,
            stack_type: WasmStackType::Void,
            descendants: vec![],
            ancestors: vec![],
            parameters: IndexMap::new(),
            associated_types: IndexMap::new(),
            self_type: Type::undefined(),
            parent: None,
            enum_variants: IndexMap::new(),
            fields: IndexMap::new(),
            regular_methods: IndexMap::new(),
            static_methods: IndexMap::new(),
            dynamic_methods: vec![],
            event_callbacks: HashMap::new(),
        };
        
        if context.types.get_by_identifier(&self.name).is_some() {
            context.errors.generic(&self.name, format!("duplicate type declaration: `{}`", &self.name));
        }

        let type_wrapped = context.types.insert(type_unwrapped, None);

        let stack_type = self.stack_type.as_ref()
            .and_then(|declaration| declaration.process(context))
            .unwrap_or(category.get_default_wasm_stack_type());
        
        context.rename_provider.add_occurence(&self.name, &self.name);

        type_wrapped.with_mut(|mut type_unwrapped| {
            type_unwrapped.stack_type = stack_type;
        });
    }

    pub fn process_parameters(&self, context: &mut ProgramContext) {
        self.process(context, |type_wrapped, context| {
            let parameters = match &self.parameters {
                Some(params) => params.process(context),
                None => IndexMap::new()
            };

            for details in parameters.values() {
                context.rename_provider.add_occurence(&details.name, &details.name);
            }

            type_wrapped.with_mut(|mut type_unwrapped| {
                type_unwrapped.parameters = parameters.clone();
            });

            let self_type = Type::actual(&type_wrapped, parameters.values().map(|param| Type::type_parameter(param)).collect(), &self.name.location);

            type_wrapped.with_mut(|mut type_unwrapped| {
                type_unwrapped.self_type = self_type;
            });
        });
    }

    pub fn compute_type_dependencies(&self, context: &mut ProgramContext) -> IndexSet<Link<TypeBlueprint>> {
        let mut type_names = vec![];
        let mut builtin_types = vec![];

        if let Some(builtin_type) = self.qualifier.get_inherited_type() {
            let builtin_type_name = builtin_type.get_name();

            if self.name.as_str() != builtin_type_name {
                builtin_types.push(builtin_type);
            }
        }

        if let Some(parent) = &self.parent {
            parent.collect_instancied_type_names(&mut type_names, context);
        }

        for field_declaration in self.get_fields() {
            if let Some(default_value) = &field_declaration.default_value {
                if let Some(expression) = &default_value.expression {
                    expression.collect_instancied_type_names(&mut type_names, context);
                }
            }
        }

        for super_field_default_value in self.get_super_fields_default_values() {
            if let Some(expression) = &super_field_default_value.expression {
                expression.collect_instancied_type_names(&mut type_names, context);
            }
        }

        let mut dependancies = IndexSet::new();

        for builtin_type in builtin_types {
            dependancies.insert(context.get_builtin_type(builtin_type, vec![]).get_type_blueprint());
        }

        for name in type_names {
            let identifier = Identifier::new(&name, Some(&ItemLocation { file: self.location.file.clone(), start: 0, end: 0 }));

            if let Some(type_blueprint) = context.types.get_by_identifier(&identifier) {
                dependancies.insert(type_blueprint);
            }
        }

        dependancies
    }

    pub fn process_parent(&self, context: &mut ProgramContext) {
        self.process(context, |type_wrapped, context| {
            let mut result = None;

            if let Some(inherited_type) = self.qualifier.get_inherited_type() {
                let parent_type_wrapped = context.types.get_by_name(inherited_type.get_name()).unwrap();

                if parent_type_wrapped != type_wrapped {
                    result = Some(ParentInfo {
                        location: ItemLocation::default(),
                        ty: parent_type_wrapped.borrow().self_type.clone(),
                    });
                }
            }

            if let Some(parsed_parent_type) = &self.parent {
                if let Some(parent_type) = parsed_parent_type.process(context) {
                    if !type_wrapped.borrow().is_class() {
                        context.errors.generic(parsed_parent_type, format!("only class types can inherit"));
                    } else {
                        match &parent_type.content() {
                            TypeContent::TypeParameter(_) => {
                                context.errors.generic(parsed_parent_type, format!("cannot inherit from type parameter"));
                            },
                            TypeContent::Actual(info) => {
                                let parent_unwrapped = info.type_blueprint.borrow();

                                if parent_unwrapped.is_class() || parent_unwrapped.name.as_str() == ANY_TYPE_NAME {
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
                        if parent_unwrapped.name.as_str() == ANY_TYPE_NAME {
                            return;
                        }
                        
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

                    context.rename_provider.add_occurence(&name, &name);

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
                                visibility: field_info.visibility.clone(),
                                offset,
                                default_value: None,
                                is_required: field_info.is_required
                            });

                            offset += 1;
                            fields.insert(field_info.name.to_string(), field_details);
                        }
                    });
                }

                for field in self.get_fields() {
                    field.process(type_unwrapped.parent.as_ref().map(|p| &p.ty), context);

                    match &field.ty {
                        Some(ty) => {
                            // Regular field

                            if !type_unwrapped.is_class() {
                                context.errors.generic(&field.name, format!("only classes can have fields"));
                                continue;
                            }

                            if fields.contains_key(field.name.as_str()) {
                                context.errors.generic(&field.name, format!("duplicate field `{}`", field.name.as_str()));
                            }

                            if let Some(field_type) = ty.process(context) {
                                context.rename_provider.add_occurence(&field.name, &field.name);

                                let field_details = Rc::new(FieldInfo {
                                    owner: type_wrapped.clone(),
                                    ty: field_type,
                                    name: field.name.clone(),
                                    visibility: FieldVisibility::from_name(field.name.as_str()),
                                    offset,
                                    default_value: None,
                                    is_required: field.default_value.is_none()
                                });

                                offset += 1;
                                fields.insert(field.name.to_string(), field_details);
                            }
                        },
                        None => {
                            // Enum variant

                            context.rename_provider.add_occurence(&field.name, &field.name);

                            if !type_unwrapped.is_enum() {
                                context.errors.generic(&field.name, format!("only enums can have variants"));
                                continue;
                            }

                            if variants.contains_key(field.name.as_str()) {
                                context.errors.generic(&field.name, format!("duplicate variant `{}`", self.name.as_str().bold()));
                            }

                            let variant_details = Rc::new(EnumVariantInfo {
                                owner: type_wrapped.clone(),
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
            let mut overriden_default_values = HashSet::new();

            type_wrapped.with_ref(|type_unwrapped| {
                if let Some(parent) = &type_unwrapped.parent {
                    parent.ty.get_type_blueprint().with_ref(|parent_unwrapped| {
                        for field_info in parent_unwrapped.fields.values() {
                            default_values.insert(field_info.name.to_string(), field_info.default_value.clone());
                        }
                    });
                }

                let self_type = type_unwrapped.self_type.clone();
                let self_argument = ArgumentInfo {
                    name: Identifier::unlocated("self"),
                    ty: self_type.clone(),
                    is_optional: false,
                    default_value: context.vasm(),
                };

                for field in self.get_fields() {
                    if field.ty.as_ref().and_then(|ty| ty.ty.as_ref()).is_none() {
                        continue;
                    }

                    if let Some(field_info) = type_unwrapped.fields.get(field.name.as_str()) {
                        let mut default_value = None;

                        if let Some(parsed_default_value) = &field.default_value {
                            let function_blueprint = FunctionBlueprint {
                                name: Identifier::unique(&format!("{}_{}_default", self.name.as_str(), field_info.name.as_str())),
                                visibility: Visibility::None,
                                parameters: IndexMap::new(),
                                arguments: vec![self_argument.clone()],
                                signature: Signature::create(None, vec![self_type.clone()], field_info.ty.clone()),
                                argument_variables: vec![],
                                owner_type: Some(type_wrapped.clone()),
                                owner_interface: None,
                                closure_details: None,
                                method_details: None,
                                kind: FunctionKind::DefaultValue,
                                body: FunctionBody::Empty,
                            };
                            let function_wrapped = context.functions.insert(function_blueprint, None);

                            context.push_scope(ScopeKind::Function(function_wrapped.clone()));

                            if let Some(vasm) = parsed_default_value.process(Some(&field_info.ty), context) {
                                if vasm.ty.is_assignable_to(&field_info.ty) {
                                    function_wrapped.borrow_mut().body = FunctionBody::Vasm(vasm);
                                    default_value = Some(function_wrapped.clone());
                                } else {
                                    context.errors.type_mismatch(parsed_default_value, &field_info.ty, &vasm.ty);
                                }
                            }

                            context.pop_scope();
                        } else {
                            default_value = None;
                        }

                        default_values.insert(field_info.name.to_string(), default_value);
                    }
                }

                for super_field in self.get_super_fields_default_values() {
                    if let Some(field_name) = &super_field.name {
                        if let Some(field_info) = type_unwrapped.fields.get(field_name.as_str()) {
                            overriden_default_values.insert(field_name.to_string());
                            
                            let function_blueprint = FunctionBlueprint {
                                name: Identifier::unique(&format!("{}_{}_default", self.name.as_str(), field_info.name.as_str())),
                                visibility: Visibility::None,
                                parameters: IndexMap::new(),
                                arguments: vec![self_argument.clone()],
                                signature: Signature::create(None, vec![self_type.clone()], field_info.ty.clone()),
                                argument_variables: vec![],
                                owner_type: Some(type_wrapped.clone()),
                                owner_interface: None,
                                closure_details: None,
                                method_details: None,
                                kind: FunctionKind::DefaultValue,
                                body: FunctionBody::Empty,
                            };
                            let function_wrapped = context.functions.insert(function_blueprint, None);

                            context.push_scope(ScopeKind::Function(function_wrapped.clone()));

                            if let Some((name, vasm)) = super_field.process(&type_unwrapped.self_type, context) {
                                function_wrapped.borrow_mut().body = FunctionBody::Vasm(vasm);
                                default_values.insert(name.clone(), Some(function_wrapped.clone()));
                            }

                            context.pop_scope();
                        } else {
                            context.errors.generic(field_name, format!("type `{}` has no field `{}`", &type_unwrapped.self_type, field_name.as_str()));
                        }
                    }
                }
            });

            type_wrapped.with_mut(|mut type_unwrapped| {
                for (name, default_value) in default_values.into_iter() {
                    let mut field_info = Rc::get_mut(type_unwrapped.fields.get_mut(&name).unwrap()).unwrap();

                    field_info.default_value = default_value;
                }

                for name in overriden_default_values {
                    if let Some(rc) = type_unwrapped.fields.get_mut(&name) {
                        let mut field_info = Rc::get_mut(rc).unwrap();

                        field_info.is_required = false;
                    }
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

    pub fn process_method_default_arguments(&self, context: &mut ProgramContext) {
        self.process(context, |type_wrapped, context| {
            for method in self.get_methods().iter().filter(|method| !method.is_autogen()) {
                method.process_default_arguments(context);
            }
        });
    }

    pub fn process_autogen_method_default_arguments(&self, context: &mut ProgramContext) {
        self.process(context, |type_wrapped, context| {
            let children = type_wrapped.borrow().descendants.clone();

            context.autogen_type = Some(type_wrapped);

            for method in self.get_methods().iter().filter(|method| method.is_autogen()) {
                for child in &children {
                    context.push_scope(ScopeKind::Type(child.clone()));
                    method.process_default_arguments(context);
                    context.pop_scope();
                }
            }

            context.autogen_type = None;
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

    pub fn process_event_callbacks(&self, context: &mut ProgramContext) {
        self.process(context, |type_wrapped, context| {
            let parent_opt = type_wrapped.borrow().parent.as_ref().map(|info| info.ty.get_type_blueprint());

            if let Some(parent_type) = parent_opt {
                type_wrapped.with_mut(|mut type_unwrapped| {
                    parent_type.with_ref(|parent_unwrapped| {
                        for (event_type_wrapped, callback_list) in parent_unwrapped.event_callbacks.iter() {
                            let self_callback_list = hashmap_get_or_insert_with(&mut type_unwrapped.event_callbacks, event_type_wrapped, || vec![]);

                            for event_callback in callback_list {
                                self_callback_list.push(event_callback.clone());
                            }
                        }
                    });
                });
            }

            for event_callback in self.get_event_callbacks() {
                event_callback.process(context);
            }
        });
    }

    fn process<'a, F : FnMut(Link<TypeBlueprint>, &mut ProgramContext)>(&self, context: &mut ProgramContext, mut f: F) {
        let type_blueprint = context.types.get_by_location(&self.name, None);

        context.push_scope(ScopeKind::Type(type_blueprint.clone()));
        f(type_blueprint, context);
        context.pop_scope();
    }
}