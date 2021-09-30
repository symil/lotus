use std::{convert::TryInto, fmt::{Display, write}, rc::Rc, result};
use indexmap::IndexMap;
use colored::*;
use parsable::DataLocation;
use crate::{items::{FullType}, program::{GET_AS_PTR_METHOD_NAME, ItemGenerator, THIS_TYPE_NAME, THIS_VAR_NAME, display_join}, utils::Link, wat};
use super::{BuiltinType, FieldDetails, FieldKind, FunctionBlueprint, GenericTypeInfo, InterfaceAssociatedTypeInfo, InterfaceBlueprint, InterfaceList, ProgramContext, ResolvedType, TypeBlueprint, TypeIndex, TypeInstanceContent, TypeInstanceHeader, TypeInstanceParameters};

#[derive(Debug, Clone)]
pub enum Type {
    Undefined,
    Void,
    Any,
    This(Link<InterfaceBlueprint>),
    Actual(ActualTypeInfo),
    TypeParameter(Rc<GenericTypeInfo>),
    FunctionParameter(Rc<GenericTypeInfo>),
    Associated(AssociatedTypeInfo),
}

#[derive(Debug, Clone)]
pub struct ActualTypeInfo {
    pub type_blueprint: Link<TypeBlueprint>,
    pub parameters: Vec<Type>
}

#[derive(Debug, Clone)]
pub struct FunctionResultInfo {
    pub function_wrapped: Link<FunctionBlueprint>,
    pub this_type: Option<Box<Type>>,
}

#[derive(Debug, Clone)]
pub struct AssociatedTypeInfo {
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

    pub fn replace_generics(&self, this_type: Option<&Type>, function_parameters: &[Type]) -> Type {
        match self {
            Type::Undefined => Type::Undefined,
            Type::Void => Type::Void,
            Type::Any => Type::Any,
            Type::This(_) => this_type.unwrap().clone(),
            Type::Actual(info) => {
                let type_blueprint = info.type_blueprint.clone();
                let parameters = info.parameters.iter().map(|p| p.replace_generics(this_type, function_parameters)).collect();

                Type::Actual(ActualTypeInfo {
                    type_blueprint,
                    parameters,
                })
            },
            Type::TypeParameter(info) => this_type.unwrap().get_parameter(info.index),
            Type::FunctionParameter(info) => function_parameters[info.index].clone(),
            Type::Associated(info) => {
                let root = info.root.replace_generics(this_type, function_parameters);

                match root {
                    Type::Actual(ref actual_type) => actual_type.type_blueprint.with_ref(|type_unwrapped| {
                        type_unwrapped.associated_types.get(info.associated.name.as_str()).unwrap().replace_generics(Some(&root), &[])
                    }),
                    _ => Type::Associated(AssociatedTypeInfo {
                        root: Box::new(root),
                        associated: info.associated.clone(),
                    })
                }
            }
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
        }
    }

    pub fn get_associated_type(&self, name: &str) -> Option<Type> {
        match self {
            Type::Undefined => None,
            Type::Void => None,
            Type::Any => None,
            Type::This(interface_wrapped) => interface_wrapped.with_ref(|interface_unwrapped| {
                match interface_unwrapped.associated_types.get(name) {
                    Some(info) => Some(Type::Associated(AssociatedTypeInfo {
                        root: Box::new(self.clone()),
                        associated: info.clone(),
                    })),
                    None => None,
                }
            }),
            Type::Actual(info) => info.type_blueprint.with_ref(|type_unwrapped| {
                type_unwrapped.associated_types.get(name).cloned()
            }),
            Type::TypeParameter(info) | Type::FunctionParameter(info) => {
                match info.required_interfaces.get_associated_type_info(name) {
                    Some(type_info) => Some(Type::Associated(AssociatedTypeInfo {
                        root: Box::new(self.clone()),
                        associated: type_info.clone()
                    })),
                    None => None
                }
            },
            Type::Associated(info) => match info.associated.required_interfaces.get_associated_type_info(name) {
                Some(type_info) => Some(Type::Associated(AssociatedTypeInfo {
                    root: Box::new(self.clone()),
                    associated: type_info.clone()
                })),
                None => None
            },
        }
    }

    pub fn is_assignable_to(&self, target: &Type) -> bool {
        match (self, target) {
            (_, Type::Any) => true,
            (Type::This(_), Type::This(_)) => true,
            // (Type::Actual(self_info), Type::Actual(target_info)) => self_info.type_blueprint.borrow().inheritance_chain.contains(target_info),
            (Type::Actual(self_info), Type::Actual(target_info)) => self_info == target_info,
            (Type::TypeParameter(self_info), Type::TypeParameter(target_info)) => Rc::as_ptr(self_info) == Rc::as_ptr(target_info),
            (Type::FunctionParameter(self_info), Type::FunctionParameter(target_info)) => Rc::as_ptr(self_info) == Rc::as_ptr(target_info),
            (Type::Associated(self_info), Type::Associated(target_info)) => self_info == target_info,
            _ => false
        }
    }

    pub fn is_function_parameter(&self, parameter: &Rc<GenericTypeInfo>) -> bool {
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
    
    pub fn check_match_interface(&self, interface: &Link<InterfaceBlueprint>, location: &DataLocation, context: &mut ProgramContext) -> bool {
        let mut details = vec![];

        let ok = match self {
            Type::Undefined => false,
            Type::Any => false,
            Type::Void => false,
            Type::This(interface_wrapped) => interface_wrapped == interface,
            Type::Actual(info) => {
                let interface_blueprint = interface.borrow();
                let type_blueprint = info.type_blueprint.borrow();

                for associated_type in interface_blueprint.associated_types.values() {
                    if let Some(associated_type_info) = type_blueprint.associated_types.get(associated_type.name.as_str()) {
                        // later: check that the associated type matches the interfaces required by the interface
                    } else {
                        details.push(format!("missing associated type `{}`", &associated_type.name));
                    }
                }

                for method_kind in &[FieldKind::Regular, FieldKind::Static] {
                    let interface_methods = match method_kind {
                        FieldKind::Regular => &interface_blueprint.regular_methods,
                        FieldKind::Static => &interface_blueprint.static_methods,
                    };

                    for expected_method_wrapped in interface_methods.values() {
                        let expected_method_unwrapped = expected_method_wrapped.borrow();
                        let type_methods = match method_kind {
                            FieldKind::Regular => &type_blueprint.regular_methods,
                            FieldKind::Static => &type_blueprint.static_methods,
                        };
                        
                        if let Some(actual_method_wrapped) = type_methods.get(expected_method_unwrapped.name.as_str()) {
                            let actual_method_unwrapped = actual_method_wrapped.borrow();
                            let actual_argument_count = actual_method_unwrapped.arguments.len();
                            let expected_argument_count = expected_method_unwrapped.arguments.len();

                            if actual_argument_count != expected_argument_count {
                                details.push(format!("method `{}`: expected {} arguments, got `{}`", expected_method_unwrapped.name.as_str().bold(), expected_argument_count, actual_argument_count));
                            } else {
                                for (i, (expected_arg, actual_arg)) in expected_method_unwrapped.arguments.iter().zip(actual_method_unwrapped.arguments.iter()).enumerate() {
                                    let actual_type = actual_arg.ty.replace_generics(Some(self), &[]);
                                    let expected_type = expected_arg.ty.replace_generics(Some(self), &[]);

                                    if &expected_type != &actual_type {
                                        details.push(format!("method `{}`, argument #{}: expected {}, got `{}`", expected_method_unwrapped.name.as_str().bold(), i + 1, expected_type, &actual_type));
                                    }
                                }
                            }

                            let actual_return_value = actual_method_unwrapped.return_value.as_ref().and_then(|info| Some(info.ty.replace_generics(Some(self), &[]))).unwrap_or(Type::Void);
                            let expected_return_type = expected_method_unwrapped.return_value.as_ref().and_then(|info| Some(info.ty.replace_generics(Some(self), &[]))).unwrap_or(Type::Void);

                            if actual_return_value != expected_return_type {
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

        if !ok {
            context.errors.add_detailed(location, format!("type `{}` does not match interface `{}`:", self, interface.borrow().name.as_str().bold()), details);
        }

        ok
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

    pub fn get_method(&self, kind: FieldKind, name: &str) -> Option<Link<FunctionBlueprint>> {
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
            Type::TypeParameter(info) => info.required_interfaces.get_method(is_static, name),
            Type::FunctionParameter(info) => info.required_interfaces.get_method(is_static, name),
            Type::Associated(info) => info.associated.required_interfaces.get_method(is_static, name),
        }
    }

    pub fn get_regular_method(&self, name: &str) -> Option<Link<FunctionBlueprint>> {
        self.get_method(FieldKind::Regular, name)
    }

    pub fn get_static_method(&self, name: &str) -> Option<Link<FunctionBlueprint>> {
        self.get_method(FieldKind::Static, name)
    }

    pub fn get_field(&self, field_name: &str) -> Option<Rc<FieldDetails>> {
        match self {
            Type::Undefined => None,
            Type::Void => None,
            Type::Any => None,
            Type::This(_) => None,
            Type::Actual(info) => info.type_blueprint.with_ref(|type_unwrapped| type_unwrapped.fields.get(field_name).cloned()),
            Type::TypeParameter(info) => None,
            Type::FunctionParameter(info) => None,
            Type::Associated(info) => None,
        }
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

                let (type_instance, exists) = context.type_instances.get_header(&parameters);

                if !exists {
                    let content = parameters.generate_content(&type_instance, context);
                    
                    context.type_instances.set_content(&parameters, content);
                }

                type_instance
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

                    associated.resolve(&type_index, context)
                })
            },
        }
    }

    pub fn print(&self) {
        println!("{}", self);
    }
}

impl PartialEq for ActualTypeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.type_blueprint == other.type_blueprint && self.parameters == other.parameters
    }
}

impl PartialEq for AssociatedTypeInfo {
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
        let s = match self {
            Type::Undefined => format!("<undefined>"),
            Type::Void => format!("void"),
            Type::Any => format!("any"),
            Type::This(_) => format!("{}", THIS_TYPE_NAME),
            Type::Actual(info) => {
                match info.parameters.is_empty() {
                    true => format!("{}", &info.type_blueprint.borrow().name),
                    false => format!("{}<{}>", &info.type_blueprint.borrow().name, display_join(&info.parameters, ",")),
                }
            },
            Type::TypeParameter(info) => format!("{}", &info.name),
            Type::FunctionParameter(info) => format!("{}", &info.name),
            Type::Associated(info) => format!("{}", &info.associated.name),
        };

        write!(f, "{}", s.bold())
    }
}