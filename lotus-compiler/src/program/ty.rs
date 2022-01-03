use std::{convert::TryInto, fmt::{Display, write}, ops::Deref, rc::Rc, result, borrow::Borrow};
use indexmap::IndexMap;
use colored::*;
use parsable::DataLocation;
use crate::{items::{ParsedType, TypeQualifier}, program::{CompilationError, FunctionCall, ItemGenerator, NamedFunctionCallDetails, SELF_TYPE_NAME, SELF_VAR_NAME, Vasm, display_join}, utils::{Link, Wrapper}, wat};
use super::{BuiltinInterface, BuiltinType, FieldInfo, FieldKind, FuncRef, FunctionBlueprint, InterfaceAssociatedTypeInfo, InterfaceBlueprint, InterfaceList, ParameterTypeInfo, ProgramContext, Signature, TypeBlueprint, TypeIndex, TypeInstanceContent, TypeInstanceHeader, TypeInstanceParameters};

pub type Type = Wrapper<TypeContent>;

#[derive(Debug, Clone)]
pub enum TypeContent {
    Undefined,
    Any,
    This(Link<InterfaceBlueprint>),
    Actual(ActualTypeContent),
    TypeParameter(Rc<ParameterTypeInfo>),
    FunctionParameter(Rc<ParameterTypeInfo>),
    Associated(AssociatedTypeContent),
    Function(Signature)
}

#[derive(Debug, Clone)]
pub struct BuiltinTypeContent {
    pub builtin_type: BuiltinType,
    pub parameters: Vec<Type>
}

#[derive(Debug, Clone)]
pub struct ActualTypeContent {
    pub type_blueprint: Link<TypeBlueprint>,
    pub parameters: Vec<Type>,
    pub location: DataLocation
}

#[derive(Debug, Clone)]
pub struct AssociatedTypeContent {
    pub root: Type,
    pub associated: Rc<InterfaceAssociatedTypeInfo>,
}

thread_local! {
    static UNDEFINED_TYPE : Type = Wrapper::new(TypeContent::Undefined);
    static ANY_TYPE : Type = Wrapper::new(TypeContent::Any);
}

impl Type {
    pub fn undefined() -> Type {
        unsafe { UNDEFINED_TYPE.with(|wrapper| wrapper.clone()) }
    }

    pub fn any() -> Type {
        unsafe { ANY_TYPE.with(|wrapper| wrapper.clone()) }
    }

    pub fn this<T : Borrow<Link<InterfaceBlueprint>>>(interface: T) -> Type {
        Self::new(TypeContent::This(interface.borrow().clone()))
    }

    pub fn actual<T : Borrow<Link<TypeBlueprint>>>(type_blueprint: T, parameters: Vec<Type>, location: &DataLocation) -> Type {
        Self::new(TypeContent::Actual(ActualTypeContent {
            type_blueprint: type_blueprint.borrow().clone(),
            parameters,
            location: location.clone(),
        }))
    }

    pub fn type_parameter(parameter_info: &Rc<ParameterTypeInfo>) -> Type {
        Self::new(TypeContent::TypeParameter(parameter_info.clone()))
    }

    pub fn function_parameter(parameter_info: &Rc<ParameterTypeInfo>) -> Type {
        Self::new(TypeContent::FunctionParameter(parameter_info.clone()))
    }

    pub fn associated<T : Borrow<Type>>(root: T, associated: &Rc<InterfaceAssociatedTypeInfo>) -> Type {
        Self::new(TypeContent::Associated(AssociatedTypeContent {
            root: root.borrow().clone(),
            associated: associated.clone(),
        }))
    }

    pub fn function<S : Borrow<Signature>>(signature: S) -> Type {
        Self::new(TypeContent::Function(signature.borrow().clone()))
    }

    pub fn is_undefined(&self) -> bool {
        match self.content() {
            TypeContent::Undefined => true,
            _ => false
        }
    }

    pub fn is_any(&self) -> bool {
        match self.content() {
            TypeContent::Any => true,
            _ => false
        }
    }

    pub fn is_object(&self) -> bool {
        match self.content() {
            TypeContent::Actual(info) => info.type_blueprint.borrow().is_class(),
            _ => false
        }
    }

    pub fn is_function(&self) -> bool {
        match self.content() {
            TypeContent::Function(_) => true,
            _ => false
        }
    }

    pub fn is_enum(&self) -> bool {
        match self.content() {
            TypeContent::Actual(info) => info.type_blueprint.borrow().is_enum(),
            _ => false
        }
    }

    pub fn is_builtin_type(&self, builtin_type: BuiltinType) -> bool {
        match self.content() {
            TypeContent::Actual(info) => info.type_blueprint.borrow().name.as_str() == builtin_type.get_name(),
            _ => false
        }
    }

    pub fn is_void(&self) -> bool {
        self.is_builtin_type(BuiltinType::Void)
    }

    pub fn is_int(&self) -> bool {
        self.is_builtin_type(BuiltinType::Int)
    }

    pub fn is_bool(&self) -> bool {
        self.is_builtin_type(BuiltinType::Bool)
    }

    pub fn inherits_from(&self, parent_name: &str) -> bool {
        match self.content() {
            TypeContent::Actual(info) => info.type_blueprint.with_ref(|type_unwrapped| {
                match type_unwrapped.name.as_str() == parent_name {
                    true => true,
                    false => match &type_unwrapped.parent {
                        Some(parent_info) => parent_info.ty.inherits_from(parent_name),
                        None => false,
                    },
                }
            }),
            _ => false
        }
    }

    pub fn get_parameters(&self) -> &[Type] {
        match self.content() {
            TypeContent::Undefined => &[],
            TypeContent::Any => &[],
            TypeContent::This(_) => &[],
            TypeContent::Actual(info) => &info.parameters,
            TypeContent::TypeParameter(_) => &[],
            TypeContent::FunctionParameter(_) => &[],
            TypeContent::Associated(_) => &[],
            TypeContent::Function(_) => &[],
        }
    }

    pub fn get_builtin_type_parameter(&self, builtin_type: BuiltinType) -> Option<&Type> {
        match self.is_builtin_type(builtin_type) {
            true => self.get_parameters().first(),
            false => None
        }
    }

    pub fn get_array_item(&self) -> Option<&Type> {
        self.get_builtin_type_parameter(BuiltinType::Array)
    }

    pub fn get_type_blueprint(&self) -> Link<TypeBlueprint> {
        match self.content() {
            TypeContent::Actual(info) => info.type_blueprint.clone(),
            _ => unreachable!()
        }
    }

    pub fn get_function_signature(&self) -> &Signature {
        match self.content() {
            TypeContent::Function(info) => info,
            _ => unreachable!()
        }
    }

    pub fn get_common_type(&self, other: &Type) -> Option<Type> {
        if self.is_assignable_to(other) {
            Some(other.clone())
        } else if other.is_assignable_to(self) {
            Some(self.clone())
        } else if let (TypeContent::Actual(self_info), TypeContent::Actual(other_info)) = (self.content(), other.content()) {
            match self_info.parameters.is_empty() && other_info.parameters.is_empty() {
                true => {
                    let self_unwrapped = self_info.type_blueprint.borrow();
                    let other_unwrapped = other_info.type_blueprint.borrow();

                    for self_ancestor in &self_unwrapped.ancestors {
                        let self_ancestor_wrapped = self_ancestor.get_type_blueprint();

                        for other_ancestor in &other_unwrapped.ancestors {
                            let other_ancestor_wrapped = other_ancestor.get_type_blueprint();

                            if self_ancestor_wrapped == other_ancestor_wrapped {
                                return Some(Type::actual(&self_ancestor_wrapped, vec![], &DataLocation::default()));
                            }
                        }
                    }

                    None
                },
                false => None,
            }
        } else {
            None
        }
    }

    pub fn replace_parameters(&self, this_type: Option<&Type>, function_parameters: &[Type]) -> Type {
        match self.content() {
            TypeContent::Undefined => Type::undefined(),
            TypeContent::Any => Type::any(),
            TypeContent::This(_) => this_type.unwrap().clone(),
            TypeContent::Actual(info) => {
                let type_blueprint = &info.type_blueprint;
                let parameters = info.parameters.iter().map(|p| p.replace_parameters(this_type, function_parameters)).collect();
                let location = &info.location;

                Type::actual(type_blueprint, parameters, location)
            },
            // TypeContent::TypeParameter(info) => this_type.unwrap().get_parameter(info.index),
            // TypeContent::FunctionParameter(info) => function_parameters[info.index].clone(),
            TypeContent::TypeParameter(info) => match this_type {
                Some(ty) => ty.get_parameter(info.index),
                None => self.clone(),
            },
            TypeContent::FunctionParameter(info) => match function_parameters.get(info.index) {
                Some(ty) => ty.clone(),
                None => self.clone(),
            },
            TypeContent::Associated(info) => {
                let root = info.root.replace_parameters(this_type, function_parameters);

                match root.content() {
                    TypeContent::Actual(ref actual_type) => actual_type.type_blueprint.with_ref(|type_unwrapped| {
                        type_unwrapped.associated_types.get(info.associated.name.as_str()).unwrap().ty.replace_parameters(Some(&root), &[])
                    }),
                    _ => Type::associated(root, &info.associated)
                }
            },
            TypeContent::Function(info) => Type::function(info.replace_parameters(this_type, function_parameters)),
        }
    }

    pub fn get_parameter(&self, index: usize) -> Type {
        match self.content() {
            TypeContent::Undefined => unreachable!(),
            TypeContent::Any => unreachable!(),
            TypeContent::This(_) => unreachable!(),
            TypeContent::Actual(info) => info.parameters[index].clone(),
            TypeContent::TypeParameter(_) => unreachable!(),
            TypeContent::FunctionParameter(_) => unreachable!(),
            TypeContent::Associated(_) => unreachable!(),
            TypeContent::Function(_) => unreachable!(),
        }
    }

    pub fn is_assignable_to(&self, target: &Type) -> bool {
        match (self.content(), target.content()) {
            (_, TypeContent::Any) => true,
            (TypeContent::This(_), TypeContent::This(_)) => true,
            (TypeContent::Actual(self_info), TypeContent::Actual(target_info)) => match self_info.type_blueprint == target_info.type_blueprint {
                true => {
                    if self_info.parameters.len() != target_info.parameters.len() {
                        return false;
                    }

                    for (self_param, target_param) in self_info.parameters.iter().zip(target_info.parameters.iter()) {
                        if !self_param.is_assignable_to(target_param) {
                            return false;
                        }
                    }

                    true
                },
                false => match self.get_as(&target_info.type_blueprint) {
                    Some(ty) => &ty == target,
                    None => false,
                },
            },
            (TypeContent::TypeParameter(self_info), TypeContent::TypeParameter(target_info)) => Rc::ptr_eq(self_info, target_info),
            (TypeContent::FunctionParameter(self_info), TypeContent::FunctionParameter(target_info)) => Rc::ptr_eq(self_info, target_info),
            (TypeContent::Associated(self_info), TypeContent::Associated(target_info)) => self_info == target_info,
            (TypeContent::Function(self_signature), TypeContent::Function(target_signature)) => self_signature.is_assignable_to(target_signature),
            _ => false
        }
    }

    pub fn get_as(&self, target: &Link<TypeBlueprint>) -> Option<Type> {
        match self.content() {
            TypeContent::Actual(self_info) => {
                self_info.type_blueprint.with_ref(|self_type_unwrapped| {
                    match self_type_unwrapped.ancestors.iter().find(|ty| &ty.get_type_blueprint() == target) {
                        Some(ancestor_type) => Some(ancestor_type.replace_parameters(Some(self), &[])),
                        None => None
                    }
                })
            },
            _ => None
        }
    }

    pub fn is_function_parameter(&self, parameter: &Rc<ParameterTypeInfo>) -> bool {
        match self.content() {
            TypeContent::FunctionParameter(info) => Rc::ptr_eq(info, parameter),
            _ => false
        }
    }

    pub fn contains_function_parameter(&self) -> bool {
        match self.content() {
            TypeContent::Undefined => false,
            TypeContent::Any => false,
            TypeContent::This(_) => false,
            TypeContent::Actual(info) => info.parameters.iter().any(|param| param.contains_function_parameter()),
            TypeContent::TypeParameter(_) => false,
            TypeContent::FunctionParameter(_) => true,
            TypeContent::Associated(info) => info.root.contains_function_parameter(),
            TypeContent::Function(info) => info.argument_types.iter().any(|param| param.contains_function_parameter()) || info.return_type.contains_function_parameter(),
        }
    }

    pub fn infer_function_parameter(&self, function_param_to_infer: &Rc<ParameterTypeInfo>, actual_type: &Type) -> Option<Type> {
        match self.content() {
            TypeContent::Undefined => None,
            TypeContent::Any => None,
            TypeContent::This(_) => None,
            TypeContent::Actual(info) => match actual_type.get_as(&info.type_blueprint) {
                Some(actual_type) => {
                    if let TypeContent::Actual(actual_info) = actual_type.content() {
                        for (self_param, actual_param) in info.parameters.iter().zip(actual_info.parameters.iter()) {
                            if let Some(inferred_type) = self_param.infer_function_parameter(function_param_to_infer, actual_param) {
                                return Some(inferred_type);
                            }
                        }
                    }

                    None
                },
                None => None,
            },
            TypeContent::TypeParameter(_) => None,
            TypeContent::FunctionParameter(info) => match Rc::ptr_eq(info, function_param_to_infer) {
                true => Some(actual_type.clone()),
                false => None
            },
            TypeContent::Associated(_) => todo!(),
            TypeContent::Function(info) => match actual_type.content() {
                TypeContent::Function(actual_type_info) => {
                    if let Some(inferred_type) = info.return_type.infer_function_parameter(function_param_to_infer, &actual_type_info.return_type) {
                        return Some(inferred_type);
                    }

                    for (argument_type, actual_argument_type) in info.argument_types.iter().zip(actual_type_info.argument_types.iter()) {
                        if let Some(inferred_type) = argument_type.infer_function_parameter(function_param_to_infer, actual_argument_type) {
                            return Some(inferred_type);
                        }
                    }

                    None
                },
                _ => None
            }
        }
    }

    pub fn check_parameters(&self, context: &mut ProgramContext) -> bool {
        match self.content() {
            TypeContent::Actual(info) => {
                info.type_blueprint.with_ref(|type_unwrapped| {
                    let mut ok = true;

                    for (actual, expected) in info.parameters.iter().zip(type_unwrapped.parameters.values()) {
                        match actual.check_parameters(context) {
                            true => {
                                for interface in expected.required_interfaces.list.iter() {
                                    if !actual.check_match_interface(interface, &info.location, context) {
                                        ok = false;
                                    }
                                }
                            },
                            false => {
                                ok = false;
                            }
                        }
                    }

                    ok
                })
            },
            _ => true
        }
    }

    pub fn match_builtin_interface(&self, interface: BuiltinInterface, context: &mut ProgramContext) -> bool {
        context.errors.set_enabled(false);
        let result = self.check_match_interface(&context.get_builtin_interface(interface), &DataLocation::default(), context);
        context.errors.set_enabled(true);

        result
    }

    pub fn check_match_interface_list(&self, interface_list: &InterfaceList, location: &DataLocation, context: &mut ProgramContext) -> bool {
        let mut ok = true;
        
        for interface in interface_list.list.iter() {
            if !self.check_match_interface(interface, location, context) {
                ok = false;
            }
        }

        ok
    }

    pub fn check_match_interface(&self, interface: &Link<InterfaceBlueprint>, location: &DataLocation, context: &mut ProgramContext) -> bool {
        let mut details = vec![];

        let ok = match self.content() {
            TypeContent::Undefined => false,
            TypeContent::Any => false,
            TypeContent::This(interface_wrapped) => interface_wrapped == interface,
            TypeContent::Actual(info) => {
                let interface_blueprint = interface.borrow();

                for associated_type in interface_blueprint.associated_types.values() {
                    if let Some(_) = self.get_associated_type(associated_type.name.as_str()) {
                        // later: check that the associated type matches the interfaces required by the interface
                    } else {
                        details.push(format!("missing associated type `{}`", associated_type.name.as_str().bold()));
                    }
                }

                for method_kind in &[FieldKind::Regular, FieldKind::Static] {
                    let interface_methods = match method_kind {
                        FieldKind::Regular => &interface_blueprint.regular_methods,
                        FieldKind::Static => &interface_blueprint.static_methods,
                    };

                    for expected_method_wrapped in interface_methods.values() {
                        let expected_method_unwrapped = expected_method_wrapped.function.borrow();
                        
                        if let Some(actual_method_wrapped) = self.get_method(method_kind.clone(), expected_method_unwrapped.name.as_str(), context) {
                            let actual_method_unwrapped = actual_method_wrapped.function.borrow();
                            let actual_arguments = &actual_method_unwrapped.signature.argument_types;
                            let expected_arguments = &expected_method_unwrapped.signature.argument_types;

                            let actual_argument_count = actual_arguments.len();
                            let expected_argument_count = expected_arguments.len();

                            if actual_argument_count != expected_argument_count {
                                details.push(format!("method `{}`: expected {} arguments, got `{}`", expected_method_unwrapped.name.as_str().bold(), expected_argument_count, actual_argument_count));
                            } else {
                                for (i, (expected_arg_type, actual_arg_type)) in expected_arguments.iter().zip(actual_arguments.iter()).enumerate() {
                                    let expected_type = expected_arg_type.replace_parameters(Some(self), &[]);
                                    let actual_type = actual_arg_type.replace_parameters(Some(self), &[]);

                                    if !actual_type.is_assignable_to(&expected_type) && !actual_type.is_undefined() {
                                        details.push(format!("method `{}`, argument #{}: expected {}, got `{}`", expected_method_unwrapped.name.as_str().bold(), i + 1, expected_type, &actual_type));
                                    }
                                }
                            }

                            let actual_return_type = actual_method_unwrapped.signature.return_type.replace_parameters(Some(self), &[]);
                            let expected_return_type = expected_method_unwrapped.signature.return_type.replace_parameters(Some(self), &[]);

                            if !actual_return_type.is_assignable_to(&expected_return_type) {
                                details.push(format!("method `{}`, return type: expected {}, got `{}`", expected_method_unwrapped.name.as_str().bold(), &expected_return_type, actual_return_type));
                            }
                        } else {
                            details.push(format!("missing method `{}`", expected_method_unwrapped.name.as_str().bold()));
                        }
                    }
                }

                details.is_empty()
            },
            TypeContent::TypeParameter(info) | TypeContent::FunctionParameter(info) => info.required_interfaces.contains(interface),
            TypeContent::Associated(info) => info.associated.required_interfaces.contains(interface),
            TypeContent::Function(_) => false,
        };

        if !ok {
            context.errors.interface_mismatch(location, interface, self);
        }

        ok
    }


    pub fn call_builtin_interface<L, F>(&self, location: &L, interface: BuiltinInterface, argument_types: &[(&Type, &DataLocation)], context: &mut ProgramContext, make_error_prefix: F) -> Option<Vasm>
        where
            L : Deref<Target=DataLocation>,
            F : Fn() -> String
    {
        let interface_wrapped = context.get_builtin_interface(interface);

        interface_wrapped.with_ref(|interface_unwrapped| {
            let (_, func_ref) = interface_unwrapped.regular_methods.first().unwrap_or_else(|| interface_unwrapped.static_methods.first().unwrap());
            let method_wrapped = &func_ref.function;

            method_wrapped.with_ref(|function_unwrapped| {
                let method_name = function_unwrapped.name.as_str();

                match self.check_match_interface(&interface_wrapped, location, context) {
                    true => {
                        let return_type = function_unwrapped.signature.return_type.replace_parameters(Some(self), &[]);

                        for (expected_arg_type, (actual_type, arg_location)) in function_unwrapped.signature.argument_types.iter().zip(argument_types.iter()) {
                            let expected_type = expected_arg_type.replace_parameters(Some(self), &[]);

                            if !actual_type.is_assignable_to(&expected_type) && !actual_type.is_undefined() {
                                context.errors.type_mismatch(arg_location, &expected_type, actual_type);
                            }
                        }

                        Some(context.vasm()
                            .call_function_named(Some(self), &method_wrapped, &[], vec![])
                            .set_type(return_type)
                        )
                    },
                    false => None,
                }
            })
        })
    }

    pub fn call_builtin_interface_no_arg<L>(&self, location: &L, interface: BuiltinInterface, context: &mut ProgramContext) -> Option<Vasm>
        where
            L : Deref<Target=DataLocation>
    {
        self.call_builtin_interface(location, interface, &[], context, || String::new())
    }

    pub fn is_ambiguous(&self) -> bool {
        match self.content() {
            TypeContent::Undefined => false,
            TypeContent::Any => true,
            TypeContent::This(_) => false,
            TypeContent::Actual(info) => info.parameters.iter().any(|ty| ty.is_ambiguous()),
            TypeContent::TypeParameter(_) => false,
            TypeContent::FunctionParameter(_) => false,
            TypeContent::Associated(info) => info.root.is_ambiguous(),
            TypeContent::Function(info) => info.argument_types.iter().any(|ty| ty.is_ambiguous()) || info.return_type.is_ambiguous(),
        }
    }

    pub fn get_field(&self, name: &str) -> Option<Rc<FieldInfo>> {
        match self.content() {
            TypeContent::Undefined => None,
            TypeContent::Any => None,
            TypeContent::This(_) => None,
            TypeContent::Actual(info) => info.type_blueprint.with_ref(|type_unwrapped| {
                type_unwrapped.fields.get(name).cloned()
            }),
            TypeContent::TypeParameter(info) => None,
            TypeContent::FunctionParameter(info) => None,
            TypeContent::Associated(info) => None,
            TypeContent::Function(_) => None,
        }
    }

    pub fn get_all_fields(&self) -> Vec<Rc<FieldInfo>> {
        match self.content() {
            TypeContent::Undefined => vec![],
            TypeContent::Any => vec![],
            TypeContent::This(_) => vec![],
            TypeContent::Actual(info) => info.type_blueprint.with_ref(|type_unwrapped| {
                type_unwrapped.fields.values().map(|field_info| field_info.clone()).collect()
            }),
            TypeContent::TypeParameter(_) => vec![],
            TypeContent::FunctionParameter(_) => vec![],
            TypeContent::Associated(_) => vec![],
            TypeContent::Function(_) => vec![],
        }
    }

    pub fn get_method(&self, kind: FieldKind, name: &str, context: &ProgramContext) -> Option<FuncRef> {
        let is_static = match kind {
            FieldKind::Regular => false,
            FieldKind::Static => true,
        };
        
        match self.content() {
            TypeContent::Undefined => None,
            TypeContent::Any => None,
            TypeContent::This(interface_wrapped) => interface_wrapped.get_method(is_static, name),
            TypeContent::Actual(info) => {
                info.type_blueprint.with_ref(|type_unwrapped| {
                    let index_map = match is_static {
                        true => &type_unwrapped.static_methods,
                        false => &type_unwrapped.regular_methods,
                    };

                    index_map.get(name).cloned()
                })
            },
            TypeContent::TypeParameter(info) => {
                info.required_interfaces.get_method(is_static, name)
                    .or_else(|| context.default_interfaces.get_method(is_static, name))
            },
            TypeContent::FunctionParameter(info) => {
                info.required_interfaces.get_method(is_static, name)
                    .or_else(|| context.default_interfaces.get_method(is_static, name))
            },
            TypeContent::Associated(info) => info.associated.required_interfaces.get_method(is_static, name),
            TypeContent::Function(_) => context.function_type().get_method(kind, name, context),
        }
    }

    pub fn get_associated_type(&self, name: &str) -> Option<Type> {
        match self.content() {
            TypeContent::Undefined => None,
            TypeContent::Any => None,
            TypeContent::This(interface_wrapped) => interface_wrapped.with_ref(|interface_unwrapped| {
                match interface_unwrapped.associated_types.get(name) {
                    Some(info) => Some(Type::associated(self, info)),
                    None => None,
                }
            }),
            TypeContent::Actual(info) => info.type_blueprint.with_ref(|type_unwrapped| {
                type_unwrapped.associated_types.get(name).map(|t| t.ty.replace_parameters(Some(self), &[]))
            }),
            TypeContent::TypeParameter(info) | TypeContent::FunctionParameter(info) => {
                match info.required_interfaces.get_associated_type_info(name) {
                    Some(type_info) => Some(Type::associated(self, &type_info)),
                    None => None
                }
            },
            TypeContent::Associated(info) => match info.associated.required_interfaces.get_associated_type_info(name) {
                Some(type_info) => Some(Type::associated(self, &type_info)),
                None => None
            },
            TypeContent::Function(info) => None,
        }
    }

    pub fn get_regular_method(&self, name: &str, context: &ProgramContext) -> Option<FuncRef> {
        self.get_method(FieldKind::Regular, name, context)
    }

    pub fn get_static_method(&self, name: &str, context: &ProgramContext) -> Option<FuncRef> {
        self.get_method(FieldKind::Static, name, context)
    }

    pub fn resolve(&self, type_index: &TypeIndex, context: &mut ProgramContext) -> Rc<TypeInstanceHeader> {
        match self.content() {
            TypeContent::Undefined => unreachable!(),
            TypeContent::Any => unreachable!(),
            TypeContent::This(_) => type_index.current_type_instance.as_ref().unwrap().clone(),
            TypeContent::Actual(info) => {
                let parameters = TypeInstanceParameters {
                    type_blueprint: info.type_blueprint.clone(),
                    type_parameters: info.parameters.iter().map(|ty| ty.resolve(type_index, context)).collect(),
                };

                context.get_type_instance(parameters)
            },
            TypeContent::TypeParameter(info) => type_index.get_current_type_parameter(info.index),
            TypeContent::FunctionParameter(info) => type_index.current_function_parameters[info.index].clone(),
            TypeContent::Associated(info) => {
                let result = info.root.resolve(type_index, context);

                result.type_blueprint.with_ref(|type_unwrapped| {
                    let associated = type_unwrapped.associated_types.get(info.associated.name.as_str()).unwrap();
                    let type_index = TypeIndex {
                        current_type_instance: Some(result.clone()),
                        current_function_parameters: vec![],
                    };

                    associated.ty.resolve(&type_index, context)
                })
            },
            TypeContent::Function(_) => context.function_type().resolve(type_index, context),
        }
    }

    pub fn to_string(&self) -> String {
        match self.content() {
            TypeContent::Undefined => format!("{{undefined}}"),
            TypeContent::Any => format!("any"),
            TypeContent::This(_) => format!("{}", SELF_TYPE_NAME),
            TypeContent::Actual(info) => {
                match info.parameters.is_empty() {
                    true => format!("{}", &info.type_blueprint.borrow().name),
                    false => {
                        let params = info.parameters.iter().map(|value| value.to_string()).collect::<Vec<String>>().join(", ");

                        format!("{}<{}>", &info.type_blueprint.borrow().name, params)
                    }
                }
            },
            TypeContent::TypeParameter(info) => format!("{}", &info.name),
            TypeContent::FunctionParameter(info) => format!("{}", &info.name),
            TypeContent::Associated(info) => format!("{}", &info.associated.name),
            TypeContent::Function(info) => info.to_string()
        }
    }

    pub fn print(&self) {
        println!("{}", self);
    }
}

impl PartialEq for ActualTypeContent {
    fn eq(&self, other: &Self) -> bool {
        self.type_blueprint == other.type_blueprint && self.parameters == other.parameters
    }
}

impl PartialEq for AssociatedTypeContent {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root && Rc::as_ptr(&self.associated) == Rc::as_ptr(&other.associated)
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self.content(), other.content()) {
            (TypeContent::This(l0), TypeContent::This(r0)) => l0 == r0,
            (TypeContent::Actual(l0), TypeContent::Actual(r0)) => l0 == r0,
            (TypeContent::TypeParameter(l0), TypeContent::TypeParameter(r0)) => Rc::ptr_eq(l0, r0),
            (TypeContent::FunctionParameter(l0), TypeContent::FunctionParameter(r0)) => Rc::ptr_eq(l0, r0),
            (TypeContent::Associated(l0), TypeContent::Associated(r0)) => l0 == r0,
            (TypeContent::Function(l0), TypeContent::Function(r0)) => l0 == r0,
            _ => core::mem::discriminant(self.content()) == core::mem::discriminant(other.content()),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string().bold())
    }
}