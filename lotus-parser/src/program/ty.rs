use std::{convert::TryInto, fmt::{Display, write}, ops::Deref, rc::Rc, result};
use indexmap::IndexMap;
use colored::*;
use parsable::DataLocation;
use crate::{items::{ParsedType, TypeQualifier}, program::{ItemGenerator, THIS_TYPE_NAME, THIS_VAR_NAME, VI, Vasm, display_join}, utils::Link, vasm, wat};
use super::{BuiltinInterface, BuiltinType, FieldInfo, FieldKind, FuncRef, FunctionBlueprint, InterfaceAssociatedTypeInfo, InterfaceBlueprint, InterfaceList, ParameterTypeInfo, ProgramContext, TypeBlueprint, TypeIndex, TypeInstanceContent, TypeInstanceHeader, TypeInstanceParameters};

#[derive(Debug, Clone)]
pub enum Type {
    Undefined,
    Void,
    Any,
    This(Link<InterfaceBlueprint>),
    Actual(ActualTypeContent),
    TypeParameter(Rc<ParameterTypeInfo>),
    FunctionParameter(Rc<ParameterTypeInfo>),
    Associated(AssociatedTypeContent),
    // Function(FunctionTypeContent)
}

#[derive(Debug, Clone)]
pub struct ActualTypeContent {
    pub type_blueprint: Link<TypeBlueprint>,
    pub parameters: Vec<Type>
}

#[derive(Debug, Clone)]
pub struct AssociatedTypeContent {
    pub root: Box<Type>,
    pub associated: Rc<InterfaceAssociatedTypeInfo>,
}

#[derive(Debug, Clone)]
pub struct FunctionTypeContent {
    pub function: Link<FunctionBlueprint>,
    pub parameters: Vec<Type>
}

impl Type {
    pub fn is_undefined(&self) -> bool {
        match self {
            Type::Undefined => true,
            _ => false
        }
    }

    pub fn is_void(&self) -> bool {
        match self {
            Type::Void => true,
            _ => false
        }
    }

    pub fn is_enum(&self) -> bool {
        match self {
            Type::Actual(info) => info.type_blueprint.borrow().qualifier == TypeQualifier::Enum,
            _ => false
        }
    }

    fn is_builtin_type(&self, builtin_type: BuiltinType) -> bool {
        let name = builtin_type.get_name();

        if let Type::Actual(info) = self {
            if info.type_blueprint.borrow().name.as_str() == name {
                return true;
            }
        }

        false
    }

    pub fn is_int(&self) -> bool {
        self.is_builtin_type(BuiltinType::Int)
    }

    pub fn is_bool(&self) -> bool {
        self.is_builtin_type(BuiltinType::Bool)
    }

    pub fn is_object(&self) -> bool {
        match self {
            Type::Actual(info) => info.type_blueprint.borrow().qualifier == TypeQualifier::Class,
            _ => false
        }
    }

    pub fn get_builtin_type_parameter(&self, builtin_type: BuiltinType) -> Option<&Type> {
        match self.is_builtin_type(builtin_type) {
            true => {
                let info = self.get_actual_type_content();

                match info.parameters.first() {
                    Some(item_type) => Some(item_type),
                    None => None,
                }
            },
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

    pub fn get_actual_type_content(&self) -> &ActualTypeContent {
        match self {
            Type::Actual(info) => info,
            _ => unreachable!()
        }
    }

    pub fn get_common_type<'a>(&'a self, other: &'a Type) -> Option<&'a Type> {
        if self.is_assignable_to(other) {
            Some(other)
        } else if other.is_assignable_to(self) {
            Some(self)
        } else {
            None
        }
    }

    pub fn replace_parameters(&self, this_type: Option<&Type>, function_parameters: &[Type]) -> Type {
        match self {
            Type::Undefined => Type::Undefined,
            Type::Void => Type::Void,
            Type::Any => Type::Any,
            Type::This(_) => this_type.unwrap().clone(),
            Type::Actual(info) => {
                let type_blueprint = info.type_blueprint.clone();
                let parameters = info.parameters.iter().map(|p| p.replace_parameters(this_type, function_parameters)).collect();

                Type::Actual(ActualTypeContent {
                    type_blueprint,
                    parameters,
                })
            },
            Type::TypeParameter(info) => this_type.unwrap().get_parameter(info.index),
            Type::FunctionParameter(info) => function_parameters[info.index].clone(),
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
            // Type::Function(info) => Type::Function(FunctionTypeContent {
            //     function: info.function.clone(),
            //     parameters: info.parameters.iter().map(|ty| ty.replace_parameters(this_type, function_parameters)).collect(),
            // }),
        }
    }

    pub fn get_parameter(&self, index: usize) -> Type {
        match self {
            Type::Undefined => unreachable!(),
            Type::Void => unreachable!(),
            Type::Any => unreachable!(),
            Type::This(_) => unreachable!(),
            Type::Actual(info) => info.parameters[index].clone(),
            Type::TypeParameter(_) => unreachable!(),
            Type::FunctionParameter(_) => unreachable!(),
            Type::Associated(_) => unreachable!(),
            // Type::Function(_) => unreachable!(),
        }
    }

    pub fn is_assignable_to(&self, target: &Type) -> bool {
        match (self, target) {
            (_, Type::Any) => true,
            (Type::Void, Type::Void) => true,
            (Type::This(_), Type::This(_)) => true,
            (Type::Actual(self_info), Type::Actual(target_info)) => match self.get_as(&target_info.type_blueprint) {
                Some(ty) => &ty == target,
                None => false,
            },
            (Type::TypeParameter(self_info), Type::TypeParameter(target_info)) => Rc::ptr_eq(self_info, target_info),
            (Type::FunctionParameter(self_info), Type::FunctionParameter(target_info)) => Rc::ptr_eq(self_info, target_info),
            (Type::Associated(self_info), Type::Associated(target_info)) => self_info == target_info,
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
            Type::Void => false,
            Type::Any => false,
            Type::This(_) => false,
            Type::Actual(info) => info.parameters.iter().any(|param| param.contains_function_parameter()),
            Type::TypeParameter(_) => false,
            Type::FunctionParameter(_) => true,
            Type::Associated(info) => info.root.contains_function_parameter(),
        }
    }

    pub fn infer_function_parameter(&self, function_param_to_infer: &Rc<ParameterTypeInfo>, actual_type: &Type) -> Option<Type> {
        match self {
            Type::Undefined => None,
            Type::Void => None,
            Type::Any => None,
            Type::This(_) => None,
            Type::Actual(info) => match actual_type.get_as(&info.type_blueprint) {
                Some(actual_type) => {
                    let actual_info = actual_type.get_actual_type_content();

                    for (self_param, actual_param) in info.parameters.iter().zip(actual_info.parameters.iter()) {
                        if let Some(inferred_type) = self_param.infer_function_parameter(function_param_to_infer, actual_param) {
                            return Some(inferred_type);
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
        }
    }

    pub fn check_parameters(&self, location: &DataLocation, context: &mut ProgramContext) -> bool {
        match self {
            Type::Actual(info) => {
                info.type_blueprint.with_ref(|type_unwrapped| {
                    let mut ok = true;

                    for (actual, expected) in info.parameters.iter().zip(type_unwrapped.parameters.values()) {
                        match actual.check_parameters(location, context) {
                            true => {
                                for interface in expected.required_interfaces.list.iter() {
                                    if !actual.check_match_interface(interface, location, context) {
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
            Type::Void => false,
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
                            let actual_argument_count = actual_method_unwrapped.arguments.len();
                            let expected_argument_count = expected_method_unwrapped.arguments.len();

                            if actual_argument_count != expected_argument_count {
                                details.push(format!("method `{}`: expected {} arguments, got `{}`", expected_method_unwrapped.name.as_str().bold(), expected_argument_count, actual_argument_count));
                            } else {
                                for (i, (expected_arg, actual_arg)) in expected_method_unwrapped.arguments.iter().zip(actual_method_unwrapped.arguments.iter()).enumerate() {
                                    let actual_type = actual_arg.ty.replace_parameters(Some(self), &[]);
                                    let expected_type = expected_arg.ty.replace_parameters(Some(self), &[]);

                                    if !actual_type.is_assignable_to(&expected_type) && !actual_type.is_undefined() {
                                        details.push(format!("method `{}`, argument #{}: expected {}, got `{}`", expected_method_unwrapped.name.as_str().bold(), i + 1, expected_type, &actual_type));
                                    }
                                }
                            }

                            let actual_return_value = actual_method_unwrapped.return_value.as_ref().and_then(|info| Some(info.ty.replace_parameters(Some(self), &[]))).unwrap_or(Type::Void);
                            let expected_return_type = expected_method_unwrapped.return_value.as_ref().and_then(|info| Some(info.ty.replace_parameters(Some(self), &[]))).unwrap_or(Type::Void);

                            if !actual_return_value.is_assignable_to(&expected_return_type) {
                                details.push(format!("method `{}`, return type: expected {}, got `{}`", expected_method_unwrapped.name.as_str().bold(), &expected_return_type, actual_return_value));
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
        };

        if !ok && !self.is_undefined() {
            context.errors.add_detailed(location, format!("type `{}` does not match interface `{}`:", self, interface.borrow().name.as_str().bold()), details);
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
                        let return_type = function_unwrapped.return_value.as_ref().and_then(|ret| Some(ret.ty.replace_parameters(Some(self), &[]))).unwrap_or(Type::Void);

                        for (expected_arg, (actual_type, arg_location)) in function_unwrapped.arguments.iter().zip(argument_types.iter()) {
                            let expected_type = expected_arg.ty.replace_parameters(Some(self), &[]);

                            if !actual_type.is_assignable_to(&expected_type) && !actual_type.is_undefined() {
                                context.errors.add(arg_location, format!("expected `{}`, got `{}`", &expected_type, actual_type));
                            }
                        }

                        let method_instruction = VI::call_method(self, method_wrapped.clone(), &[], None, vasm![]);
                        let result = Vasm::new(return_type, vec![], vec![method_instruction]);

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
            Type::Void => false,
            Type::Any => true,
            Type::This(_) => false,
            Type::Actual(info) => info.parameters.iter().any(|ty| ty.is_ambiguous()),
            Type::TypeParameter(_) => false,
            Type::FunctionParameter(_) => false,
            Type::Associated(info) => info.root.is_ambiguous(),
        }
    }

    pub fn get_field(&self, name: &str) -> Option<Rc<FieldInfo>> {
        match self {
            Type::Undefined => None,
            Type::Void => None,
            Type::Any => None,
            Type::This(_) => None,
            Type::Actual(info) => info.type_blueprint.with_ref(|type_unwrapped| {
                type_unwrapped.fields.get(name).cloned()
            }),
            Type::TypeParameter(info) => None,
            Type::FunctionParameter(info) => None,
            Type::Associated(info) => None,
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
            Type::Void => None,
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
        }
    }

    pub fn get_associated_type(&self, name: &str) -> Option<Type> {
        match self {
            Type::Undefined => None,
            Type::Void => None,
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
                type_unwrapped.associated_types.get(name).and_then(|t| Some(t.ty.replace_parameters(Some(self), &[])))
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
            // Type::Function(info) => None,
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
            Type::Void => unreachable!(),
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
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            Type::Undefined => format!("<undefined>"),
            Type::Void => format!("void"),
            Type::Any => format!("any"),
            Type::This(_) => format!("{}", THIS_TYPE_NAME),
            Type::Actual(info) => {
                match info.parameters.is_empty() {
                    true => format!("{}", &info.type_blueprint.borrow().name),
                    false => {
                        let params = info.parameters.iter().map(|value| value.get_name()).collect::<Vec<String>>().join(", ");

                        format!("{}<{}>", &info.type_blueprint.borrow().name, params)
                    }
                }
            },
            Type::TypeParameter(info) => format!("{}", &info.name),
            Type::FunctionParameter(info) => format!("{}", &info.name),
            Type::Associated(info) => format!("{}", &info.associated.name),
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
        write!(f, "{}", self.get_name().bold())
    }
}