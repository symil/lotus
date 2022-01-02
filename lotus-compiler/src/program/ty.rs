use std::{convert::TryInto, fmt::{Display, write}, ops::Deref, rc::Rc, result, borrow::Borrow};
use indexmap::IndexMap;
use colored::*;
use parsable::DataLocation;
use crate::{items::{ParsedType, TypeQualifier}, program::{CompilationError, FunctionCall, ItemGenerator, NamedFunctionCallDetails, SELF_TYPE_NAME, SELF_VAR_NAME, VI, Vasm, display_join}, utils::Link, wat};
use super::{BuiltinInterface, BuiltinType, FieldInfo, FieldKind, FuncRef, FunctionBlueprint, InterfaceAssociatedTypeInfo, InterfaceBlueprint, InterfaceList, ParameterTypeInfo, ProgramContext, Signature, TypeBlueprint, TypeIndex, TypeInstanceContent, TypeInstanceHeader, TypeInstanceParameters};

#[derive(Debug, Clone)]
pub enum Type {
    Undefined,
    Any,
    This(Link<InterfaceBlueprint>),
    Actual(ActualTypeContent),
    TypeParameter(Rc<ParameterTypeInfo>),
    FunctionParameter(Rc<ParameterTypeInfo>),
    Associated(AssociatedTypeContent),
    Function(Box<Signature>)
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
    pub root: Box<Type>,
    pub associated: Rc<InterfaceAssociatedTypeInfo>,
}

impl Type {
    pub fn is_undefined(&self) -> bool {
        match self {
            Type::Undefined => true,
            _ => false
        }
    }

    pub fn is_object(&self) -> bool {
        match self {
            Type::Actual(info) => info.type_blueprint.borrow().is_class(),
            _ => false
        }
    }

    pub fn is_function(&self) -> bool {
        match self {
            Type::Function(_) => true,
            _ => false
        }
    }

    pub fn is_enum(&self) -> bool {
        match self {
            Type::Actual(info) => info.type_blueprint.borrow().is_enum(),
            _ => false
        }
    }

    pub fn is_builtin_type(&self, builtin_type: BuiltinType) -> bool {
        match self {
            Type::Actual(info) => info.type_blueprint.borrow().name.as_str() == builtin_type.get_name(),
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

    pub fn push_parameters(&mut self, parameter_types: Vec<Type>) {
        match self {
            Type::Actual(info) => info.parameters.extend(parameter_types),
            _ => unreachable!()
        }
    }

    pub fn inherits_from(&self, parent_name: &str) -> bool {
        match self {
            Type::Actual(info) => info.type_blueprint.with_ref(|type_unwrapped| {
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
        match self {
            Type::Undefined => &[],
            Type::Any => &[],
            Type::This(_) => &[],
            Type::Actual(info) => &info.parameters,
            Type::TypeParameter(_) => &[],
            Type::FunctionParameter(_) => &[],
            Type::Associated(_) => &[],
            Type::Function(_) => &[],
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
        match self {
            Type::Actual(info) => info.type_blueprint.clone(),
            _ => unreachable!()
        }
    }

    pub fn get_function_signature(&self) -> &Signature {
        match self {
            Type::Function(info) => info,
            _ => unreachable!()
        }
    }

    pub fn get_common_type(&self, other: &Type) -> Option<Type> {
        if self.is_assignable_to(other) {
            Some(other.clone())
        } else if other.is_assignable_to(self) {
            Some(self.clone())
        } else if let (Type::Actual(self_info), Type::Actual(other_info)) = (self, other) {
            match self_info.parameters.is_empty() && other_info.parameters.is_empty() {
                true => {
                    let self_unwrapped = self_info.type_blueprint.borrow();
                    let other_unwrapped = other_info.type_blueprint.borrow();

                    for self_ancestor in &self_unwrapped.ancestors {
                        let self_ancestor_wrapped = self_ancestor.get_type_blueprint();

                        for other_ancestor in &other_unwrapped.ancestors {
                            let other_ancestor_wrapped = other_ancestor.get_type_blueprint();

                            if self_ancestor_wrapped == other_ancestor_wrapped {
                                return Some(Type::Actual(ActualTypeContent {
                                    type_blueprint: self_ancestor_wrapped.clone(),
                                    parameters: vec![],
                                    location: DataLocation::default(),
                                }));
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
        match self {
            Type::Undefined => Type::Undefined,
            Type::Any => Type::Any,
            Type::This(_) => this_type.unwrap().clone(),
            Type::Actual(info) => {
                let type_blueprint = info.type_blueprint.clone();
                let parameters = info.parameters.iter().map(|p| p.replace_parameters(this_type, function_parameters)).collect();
                let location = info.location.clone();

                Type::Actual(ActualTypeContent {
                    type_blueprint,
                    parameters,
                    location
                })
            },
            // Type::TypeParameter(info) => this_type.unwrap().get_parameter(info.index),
            // Type::FunctionParameter(info) => function_parameters[info.index].clone(),
            Type::TypeParameter(info) => match this_type {
                Some(ty) => ty.get_parameter(info.index),
                None => self.clone(),
            },
            Type::FunctionParameter(info) => match function_parameters.get(info.index) {
                Some(ty) => ty.clone(),
                None => self.clone(),
            },
            Type::Associated(info) => {
                let root = info.root.replace_parameters(this_type, function_parameters);

                match root {
                    Type::Actual(ref actual_type) => actual_type.type_blueprint.with_ref(|type_unwrapped| {
                        type_unwrapped.associated_types.get(info.associated.name.as_str()).unwrap().ty.replace_parameters(Some(&root), &[])
                    }),
                    _ => Type::Associated(AssociatedTypeContent {
                        root: Box::new(root),
                        associated: info.associated.clone(),
                    })
                }
            },
            Type::Function(info) => Type::Function(Box::new(info.replace_parameters(this_type, function_parameters))),
        }
    }

    pub fn get_parameter(&self, index: usize) -> Type {
        match self {
            Type::Undefined => unreachable!(),
            Type::Any => unreachable!(),
            Type::This(_) => unreachable!(),
            Type::Actual(info) => info.parameters[index].clone(),
            Type::TypeParameter(_) => unreachable!(),
            Type::FunctionParameter(_) => unreachable!(),
            Type::Associated(_) => unreachable!(),
            Type::Function(_) => unreachable!(),
        }
    }

    pub fn is_assignable_to(&self, target: &Type) -> bool {
        match (self, target) {
            (_, Type::Any) => true,
            (Type::This(_), Type::This(_)) => true,
            (Type::Actual(self_info), Type::Actual(target_info)) => match self_info.type_blueprint == target_info.type_blueprint {
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
            (Type::TypeParameter(self_info), Type::TypeParameter(target_info)) => Rc::ptr_eq(self_info, target_info),
            (Type::FunctionParameter(self_info), Type::FunctionParameter(target_info)) => Rc::ptr_eq(self_info, target_info),
            (Type::Associated(self_info), Type::Associated(target_info)) => self_info == target_info,
            (Type::Function(self_signature), Type::Function(target_signature)) => self_signature.is_assignable_to(target_signature),
            _ => false
        }
    }

    pub fn get_as(&self, target: &Link<TypeBlueprint>) -> Option<Type> {
        match self {
            Type::Actual(self_info) => {
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
        match self {
            Type::FunctionParameter(info) => Rc::ptr_eq(info, parameter),
            _ => false
        }
    }

    pub fn contains_function_parameter(&self) -> bool {
        match self {
            Type::Undefined => false,
            Type::Any => false,
            Type::This(_) => false,
            Type::Actual(info) => info.parameters.iter().any(|param| param.contains_function_parameter()),
            Type::TypeParameter(_) => false,
            Type::FunctionParameter(_) => true,
            Type::Associated(info) => info.root.contains_function_parameter(),
            Type::Function(info) => info.argument_types.iter().any(|param| param.contains_function_parameter()) || info.return_type.contains_function_parameter(),
        }
    }

    pub fn infer_function_parameter(&self, function_param_to_infer: &Rc<ParameterTypeInfo>, actual_type: &Type) -> Option<Type> {
        match self {
            Type::Undefined => None,
            Type::Any => None,
            Type::This(_) => None,
            Type::Actual(info) => match actual_type.get_as(&info.type_blueprint) {
                Some(actual_type) => {
                    if let Type::Actual(actual_info) = actual_type {
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
            Type::TypeParameter(_) => None,
            Type::FunctionParameter(info) => match Rc::ptr_eq(info, function_param_to_infer) {
                true => Some(actual_type.clone()),
                false => None
            },
            Type::Associated(_) => todo!(),
            Type::Function(info) => match actual_type {
                Type::Function(actual_type_info) => {
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
        match self {
            Type::Actual(info) => {
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

        let ok = match self {
            Type::Undefined => false,
            Type::Any => false,
            Type::This(interface_wrapped) => interface_wrapped == interface,
            Type::Actual(info) => {
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
            Type::TypeParameter(info) | Type::FunctionParameter(info) => info.required_interfaces.contains(interface),
            Type::Associated(info) => info.associated.required_interfaces.contains(interface),
            Type::Function(_) => false,
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

                        let result = Vasm::new(return_type)
                            .call_function_named(Some(self), &method_wrapped, &[], vec![]);

                        Some(result)
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
        match self {
            Type::Undefined => false,
            Type::Any => true,
            Type::This(_) => false,
            Type::Actual(info) => info.parameters.iter().any(|ty| ty.is_ambiguous()),
            Type::TypeParameter(_) => false,
            Type::FunctionParameter(_) => false,
            Type::Associated(info) => info.root.is_ambiguous(),
            Type::Function(info) => info.argument_types.iter().any(|ty| ty.is_ambiguous()) || info.return_type.is_ambiguous(),
        }
    }

    pub fn get_field(&self, name: &str) -> Option<Rc<FieldInfo>> {
        match self {
            Type::Undefined => None,
            Type::Any => None,
            Type::This(_) => None,
            Type::Actual(info) => info.type_blueprint.with_ref(|type_unwrapped| {
                type_unwrapped.fields.get(name).cloned()
            }),
            Type::TypeParameter(info) => None,
            Type::FunctionParameter(info) => None,
            Type::Associated(info) => None,
            Type::Function(_) => None,
        }
    }

    pub fn get_all_fields(&self) -> Vec<Rc<FieldInfo>> {
        match self {
            Type::Undefined => vec![],
            Type::Any => vec![],
            Type::This(_) => vec![],
            Type::Actual(info) => info.type_blueprint.with_ref(|type_unwrapped| {
                type_unwrapped.fields.values().map(|field_info| field_info.clone()).collect()
            }),
            Type::TypeParameter(_) => vec![],
            Type::FunctionParameter(_) => vec![],
            Type::Associated(_) => vec![],
            Type::Function(_) => vec![],
        }
    }

    pub fn get_method(&self, kind: FieldKind, name: &str, context: &ProgramContext) -> Option<FuncRef> {
        let is_static = match kind {
            FieldKind::Regular => false,
            FieldKind::Static => true,
        };
        
        match self {
            Type::Undefined => None,
            Type::Any => None,
            Type::This(interface_wrapped) => interface_wrapped.get_method(is_static, name),
            Type::Actual(info) => {
                info.type_blueprint.with_ref(|type_unwrapped| {
                    let index_map = match is_static {
                        true => &type_unwrapped.static_methods,
                        false => &type_unwrapped.regular_methods,
                    };

                    index_map.get(name).cloned()
                })
            },
            Type::TypeParameter(info) => {
                info.required_interfaces.get_method(is_static, name)
                    .or_else(|| context.default_interfaces.get_method(is_static, name))
            },
            Type::FunctionParameter(info) => {
                info.required_interfaces.get_method(is_static, name)
                    .or_else(|| context.default_interfaces.get_method(is_static, name))
            },
            Type::Associated(info) => info.associated.required_interfaces.get_method(is_static, name),
            Type::Function(_) => context.function_type().get_method(kind, name, context),
        }
    }

    pub fn get_associated_type(&self, name: &str) -> Option<Type> {
        match self {
            Type::Undefined => None,
            Type::Any => None,
            Type::This(interface_wrapped) => interface_wrapped.with_ref(|interface_unwrapped| {
                match interface_unwrapped.associated_types.get(name) {
                    Some(info) => Some(Type::Associated(AssociatedTypeContent {
                        root: Box::new(self.clone()),
                        associated: info.clone(),
                    })),
                    None => None,
                }
            }),
            Type::Actual(info) => info.type_blueprint.with_ref(|type_unwrapped| {
                type_unwrapped.associated_types.get(name).map(|t| t.ty.replace_parameters(Some(self), &[]))
            }),
            Type::TypeParameter(info) | Type::FunctionParameter(info) => {
                match info.required_interfaces.get_associated_type_info(name) {
                    Some(type_info) => Some(Type::Associated(AssociatedTypeContent {
                        root: Box::new(self.clone()),
                        associated: type_info.clone()
                    })),
                    None => None
                }
            },
            Type::Associated(info) => match info.associated.required_interfaces.get_associated_type_info(name) {
                Some(type_info) => Some(Type::Associated(AssociatedTypeContent {
                    root: Box::new(self.clone()),
                    associated: type_info.clone()
                })),
                None => None
            },
            Type::Function(info) => None,
        }
    }

    pub fn get_regular_method(&self, name: &str, context: &ProgramContext) -> Option<FuncRef> {
        self.get_method(FieldKind::Regular, name, context)
    }

    pub fn get_static_method(&self, name: &str, context: &ProgramContext) -> Option<FuncRef> {
        self.get_method(FieldKind::Static, name, context)
    }

    pub fn resolve(&self, type_index: &TypeIndex, context: &mut ProgramContext) -> Rc<TypeInstanceHeader> {
        match self {
            Type::Undefined => unreachable!(),
            Type::Any => unreachable!(),
            Type::This(_) => type_index.current_type_instance.as_ref().unwrap().clone(),
            Type::Actual(info) => {
                let parameters = TypeInstanceParameters {
                    type_blueprint: info.type_blueprint.clone(),
                    type_parameters: info.parameters.iter().map(|ty| ty.resolve(type_index, context)).collect(),
                };

                context.get_type_instance(parameters)
            },
            Type::TypeParameter(info) => type_index.get_current_type_parameter(info.index),
            Type::FunctionParameter(info) => type_index.current_function_parameters[info.index].clone(),
            Type::Associated(info) => {
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
            Type::Function(_) => context.function_type().resolve(type_index, context),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Type::Undefined => format!("<undefined>"),
            Type::Any => format!("any"),
            Type::This(_) => format!("{}", SELF_TYPE_NAME),
            Type::Actual(info) => {
                match info.parameters.is_empty() {
                    true => format!("{}", &info.type_blueprint.borrow().name),
                    false => {
                        let params = info.parameters.iter().map(|value| value.to_string()).collect::<Vec<String>>().join(", ");

                        format!("{}<{}>", &info.type_blueprint.borrow().name, params)
                    }
                }
            },
            Type::TypeParameter(info) => format!("{}", &info.name),
            Type::FunctionParameter(info) => format!("{}", &info.name),
            Type::Associated(info) => format!("{}", &info.associated.name),
            Type::Function(info) => info.to_string()
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
        match (self, other) {
            (Self::This(l0), Self::This(r0)) => l0 == r0,
            (Self::Actual(l0), Self::Actual(r0)) => l0 == r0,
            (Self::TypeParameter(l0), Self::TypeParameter(r0)) => Rc::ptr_eq(l0, r0),
            (Self::FunctionParameter(l0), Self::FunctionParameter(r0)) => Rc::ptr_eq(l0, r0),
            (Self::Associated(l0), Self::Associated(r0)) => l0 == r0,
            (Self::Function(l0), Self::Function(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Default for Type {
    fn default() -> Self {
        Self::Undefined
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string().bold())
    }
}