use std::{convert::TryInto, fmt::Display, rc::Rc, result};
use indexmap::IndexMap;
use parsable::DataLocation;
use crate::{items::{FullType}, program::{GET_AS_PTR_METHOD_NAME, THIS_TYPE_NAME, THIS_VAR_NAME}, utils::Link, wat};
use super::{FieldDetails, FunctionBlueprint, GenericTypeInfo, InterfaceAssociatedTypeInfo, InterfaceBlueprint, InterfaceList, ProgramContext, ResolvedType, TypeBlueprint, TypeIndex, TypeInstance};

#[derive(Debug, Clone)]
pub enum Type {
    Void,
    Any,
    This(Link<InterfaceBlueprint>),
    Actual(ActualTypeInfo),
    TypeParameter(Rc<GenericTypeInfo>),
    FunctionParameter(Rc<GenericTypeInfo>),
    Associated(AssociatedTypeInfo),
    TypeRef(Box<Type>)
}

#[derive(Debug, Clone)]
pub struct ActualTypeInfo {
    pub type_wrapped: Link<TypeBlueprint>,
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
    // pub fn void() -> Rc<Type> {
    //     Rc::new(Type::Void)
    // }

    // pub fn any() -> Rc<Type> {
    //     Rc::new(Type::Any)
    // }

    // pub fn actual(type_wrapped: Link<TypeBlueprint>) -> Rc<Type> {
    //     Rc::new(Type::Actual(ActualTypeInfo {
    //         type_wrapped,
    //     }))
    // }

    // pub fn interface(interface_wrapped: Link<InterfaceBlueprint>) -> Rc<Type> {
    //     Rc::new(Type::Interface(InterfaceTypeInfo {
    //         interface_wrapped,
    //     }))
    // }

    // pub fn parameter(parent: &Rc<Type>, name: &str) -> Rc<Type> {
    //     Rc::new(Type::Parameter(AssociatedTypeInfo {
    //         parent: Rc::clone(parent),
    //         name: name.to_string()
    //     }))
    // }

    // pub fn associated(parent: &Rc<Type>, name: &str) -> Rc<Type> {
    //     Rc::new(Type::Associated(AssociatedTypeInfo {
    //         parent: Rc::clone(parent),
    //         name: name.to_string()
    //     }))
    // }

    pub fn is_void(&self) -> bool {
        match self {
            Type::Void => true,
            _ => false
        }
    }

    pub fn is_integer(&self) -> bool {
        if let Type::Actual(info) = self {
            // TODO: do this more properly
            if info.type_wrapped.borrow().name.as_str() == "int" {
                return true;
            }
        }

        false
    }

    pub fn replace_generics(&self, this_type: &Type, function_parameters: &[Type]) -> Type {
        match self {
            Type::Void => Type::Void,
            Type::Any => Type::Any,
            Type::This(_) => this_type.clone(),
            Type::Actual(info) => {
                let type_wrapped = info.type_wrapped.clone();
                let parameters = info.parameters.iter().map(|p| p.replace_generics(this_type, function_parameters)).collect();

                Type::Actual(ActualTypeInfo {
                    type_wrapped,
                    parameters,
                })
            },
            Type::TypeParameter(info) => this_type.get_parameter(info.index),
            Type::FunctionParameter(info) => function_parameters[info.index].clone(),
            Type::Associated(info) => Type::Associated(AssociatedTypeInfo {
                root: Box::new(info.root.replace_generics(this_type, function_parameters)),
                associated: info.associated.clone(),
            }),
            Type::TypeRef(ty) => Type::TypeRef(Box::new(ty.replace_generics(this_type, function_parameters))),
        }
    }

    pub fn get_parameter(&self, index: usize) -> Type {
        match self {
            Type::Void => unreachable!(),
            Type::Any => unreachable!(),
            Type::This(_) => unreachable!(),
            Type::Actual(info) => info.parameters[index].clone(),
            Type::TypeParameter(_) => unreachable!(),
            Type::FunctionParameter(_) => unreachable!(),
            Type::Associated(_) => unreachable!(),
            Type::TypeRef(ty) => ty.get_parameter(index),
        }
    }

    pub fn get_associated_type(&self, name: &str) -> Option<Type> {
        match self {
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
            Type::Actual(info) => info.type_wrapped.with_ref(|type_unwrapped| {
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
            Type::TypeRef(_) => None,
        }
    }

    pub fn is_assignable(&self) -> bool {
        match self {
            Type::TypeRef(_) => false,
            _ => true
        }
    }

    pub fn is_assignable_to(&self, target: &Type) -> bool {
        match (self, target) {
            (_, Type::Any) => true,
            (Type::This(_), Type::This(_)) => true,
            (Type::Actual(self_info), Type::Actual(target_info)) => self_info.type_wrapped.borrow().inheritance_chain.contains(target_info),
            (Type::TypeParameter(self_info), Type::TypeParameter(target_info)) => Rc::as_ptr(self_info) == Rc::as_ptr(target_info),
            (Type::FunctionParameter(self_info), Type::FunctionParameter(target_info)) => Rc::as_ptr(self_info) == Rc::as_ptr(target_info),
            (Type::Associated(self_info), Type::Associated(target_info)) => self_info == target_info,
            _ => false
        }
    }
    
    pub fn match_interface(&self, interface: &Link<InterfaceBlueprint>) -> bool {
        match self {
            Type::Void => unreachable!(),
            Type::Any => unreachable!(),
            Type::This(interface_wrapped) => interface_wrapped == interface,
            Type::Actual(info) => {
                let interface_blueprint = interface.borrow();
                let type_blueprint = info.type_wrapped.borrow();

                for associated_type in interface_blueprint.associated_types.values() {
                    if let Some(associated_type_info) = type_blueprint.associated_types.get(associated_type.name.as_str()) {
                        // later: check that the associated type matches the interfaces required by the interface
                    } else {
                        return false;
                    }
                }
         
                for required_method in interface_blueprint.methods.values() {
                    let expected_function_blueprint = required_method.borrow();

                    if let Some(function_blueprint) = type_blueprint.methods.get(expected_function_blueprint.name.as_str()) {
                        let actual_function_blueprint = function_blueprint.borrow();

                        if actual_function_blueprint.arguments.len() != expected_function_blueprint.arguments.len() {
                            return false;
                        }

                        for (expected_arg, actual_arg) in expected_function_blueprint.arguments.iter().zip(actual_function_blueprint.arguments.iter()) {
                            if &expected_arg.ty != &actual_arg.ty {
                                return false;
                            }
                        }

                        let actual_return_value = actual_function_blueprint.return_value.as_ref().and_then(|info| Some(&info.ty));
                        let expected_return_type = expected_function_blueprint.return_value.as_ref().and_then(|info| Some(&info.ty));

                        if actual_return_value != expected_return_type {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                true
            },
            Type::TypeParameter(info) | Type::FunctionParameter(info) => info.required_interfaces.contains(interface),
            Type::Associated(info) => info.associated.required_interfaces.contains(interface),
            Type::TypeRef(_) => unreachable!(),
        }
    }

    pub fn is_ambiguous(&self) -> bool {
        match self {
            Type::Void => false,
            Type::Any => true,
            Type::This(_) => false,
            Type::Actual(info) => info.parameters.iter().any(|ty| ty.is_ambiguous()),
            Type::TypeParameter(_) => false,
            Type::FunctionParameter(_) => false,
            Type::Associated(info) => info.root.is_ambiguous(),
            Type::TypeRef(ty) => ty.is_ambiguous(),
        }
    }

    pub fn get_maybe_static_method(&self, is_static: bool, name: &str) -> Option<Link<FunctionBlueprint>> {
        match self {
            Type::Void => None,
            Type::Any => None,
            Type::This(interface_wrapped) => interface_wrapped.get_method(is_static, name),
            Type::Actual(info) => {
                info.type_wrapped.with_ref(|type_unwrapped| {
                    let index_map = match is_static {
                        true => &type_unwrapped.static_methods,
                        false => &type_unwrapped.methods,
                    };

                    index_map.get(name).cloned()
                })
            },
            Type::TypeParameter(info) => info.required_interfaces.get_method(is_static, name),
            Type::FunctionParameter(info) => info.required_interfaces.get_method(is_static, name),
            Type::Associated(info) => info.associated.required_interfaces.get_method(is_static, name),
            Type::TypeRef(ty) => ty.get_maybe_static_method(true, name)
        }
    }

    pub fn get_method(&self, method_name: &str) -> Option<Link<FunctionBlueprint>> {
        self.get_maybe_static_method(false, method_name)
    }

    pub fn get_static_method(&self, method_name: &str) -> Option<Link<FunctionBlueprint>> {
        self.get_maybe_static_method(true, method_name)
    }

    pub fn get_field(&self, field_name: &str) -> Option<Rc<FieldDetails>> {
        match self {
            Type::Void => None,
            Type::Any => None,
            Type::This(_) => None,
            Type::Actual(info) => info.type_wrapped.with_ref(|type_unwrapped| type_unwrapped.fields.get(field_name).cloned()),
            Type::TypeParameter(info) => None,
            Type::FunctionParameter(info) => None,
            Type::Associated(info) => None,
            Type::TypeRef(_) => None,
        }
    }

    pub fn resolve(&self, type_index: &TypeIndex) -> Rc<TypeInstance> {
        match self {
            Type::Void => unreachable!(),
            Type::Any => unreachable!(),
            Type::Actual(info) => {
                let mut resolved_type = ResolvedType {
                    type_wrapped: info.type_wrapped.clone(),
                    parameters: Vec::with_capacity(info.parameters.len()),
                };

                for parameter in &info.parameters {
                    resolved_type.parameters.push(parameter.resolve(current_type));
                }

                resolved_type
            },
            Type::TypeParameter(info) => {

            },
            Type::TypeRef(_) => unreachable!(),
        }
    }
}

impl PartialEq for ActualTypeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.type_wrapped == other.type_wrapped && self.parameters == other.parameters
    }
}

impl PartialEq for AssociatedTypeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root && self.name == other.name
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Actual(l0), Self::Actual(r0)) => l0 == r0,
            (Self::TypeParameter(l0), Self::TypeParameter(r0)) => Rc::as_ptr(l0) == Rc::as_ptr(r0),
            (Self::TypeRef(l0), Self::TypeRef(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Default for Type {
    fn default() -> Self {
        Self::Void
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Void => write!(f, "<void>"),
            Type::Any => write!(f, "<any>"),
            Type::Actual(info) => write!(f, "{}", &info.type_wrapped.borrow().name),
            Type::TypeParameter(info) => write!(f, "{}", &info.name),
            Type::TypeRef(typeref) => write!(f, "<type {}>", &typeref),
        }
    }
}