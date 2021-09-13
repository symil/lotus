use std::fmt::Display;
use parsable::DataLocation;
use crate::{items::{FullType}, program::{GET_AS_PTR_METHOD_NAME, THIS_TYPE_NAME, THIS_VAR_NAME}, utils::Link, wat};
use super::{AssociatedType, FieldDetails, FunctionBlueprint, InterfaceAssociatedType, InterfaceBlueprint, ParameterType, ProgramContext, ResolvedType, TypeBlueprint};

#[derive(Debug, Clone)]
pub enum Type {
    Void,
    Any,
    Associated(Link<InterfaceAssociatedType>),
    Parameter(Link<ParameterType>),
    Actual(ActualTypeInfo),
    TypeRef(Box<Type>)
}

#[derive(Debug, Clone)]
pub struct ActualTypeInfo {
    pub type_blueprint: Link<TypeBlueprint>,
    pub parameters: Vec<Type>
}

impl Type {
    pub fn is_void(&self) -> bool {
        match self {
            Type::Void => true,
            _ => false
        }
    }

    pub fn is_any(&self) -> bool {
        match self {
            Type::Any => true,
            _ => false
        }
    }

    pub fn is_generic(&self) -> bool {
        match self {
            Type::Parameter(_) => true,
            _ => false
        }
    }

    pub fn is_actual(&self) -> bool {
        match self {
            Type::Actual(_) => true,
            _ => false
        }
    }

    pub fn is_integer(&self) -> bool {
        if let Type::Actual(info) = self {
            // TODO: do this more properly
            if info.type_blueprint.borrow().name.as_str() == "int" {
                return true;
            }
        }

        false
    }

    pub fn get_assiciated_type(&self, interface_associated_type: &Link<InterfaceAssociatedType>) -> Option<Type> {
        match self {
            Type::Void => None,
            Type::Any => None,
            Type::Associated(_) => None,
            Type::Parameter(info) => match info.borrow().required_interfaces.contains(&interface_associated_type.borrow().owner) {
                true => Some(Type::Associated(interface_associated_type.clone())),
                false => None
            },
            Type::Actual(info) => match info.type_blueprint.borrow().associated_types.get(interface_associated_type.borrow().name.as_str()) {
                Some(associated_type) => Some(associated_type.value.clone()),
                None => None,
            },
            Type::TypeRef(_) => None,
        }
    }

    pub fn is_assignable(&self) -> bool {
        match self {
            Type::Void => true,
            Type::Any => true,
            Type::Associated(_) => true,
            Type::Parameter(_) => true,
            Type::Actual(_) => true,
            Type::TypeRef(_) => false,
        }
    }

    pub fn is_assignable_to(&self, target: &Type) -> bool {
        match self {
            Type::Void => false,
            Type::Any => true,
            Type::Associated(self_info) => match target {
                Type::Associated(target_info) => self_info == target_info,
                _ => false
            },
            Type::Parameter(self_info) => {
                match target {
                    Type::Parameter(target_info) => self_info == target_info,
                    _ => false
                }
            },
            Type::Actual(self_info) => {
                match target {
                    Type::Actual(target_info) => self_info.type_blueprint.borrow().inheritance_chain.contains(target_info),
                    _ => false
                }
            },
            Type::TypeRef(_) => false,
        }
    }
    
    pub fn match_interface(&self, interface: &Link<InterfaceBlueprint>) -> bool {
        match self {
            Type::Void => unreachable!(),
            Type::Any => unreachable!(),
            Type::Associated(info) => unreachable!(),
            Type::Parameter(parameter) => parameter.borrow().required_interfaces.contains(interface),
            Type::Actual(info) => {
                let interface_blueprint = interface.borrow();
                let type_blueprint = info.type_blueprint.borrow();

                for associated_type in interface_blueprint.associated_types.values() {
                    if let Some(associated_type_info) = type_blueprint.associated_types.get(associated_type.borrow().name.as_str()) {
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

                        if actual_function_blueprint.return_value.and_then(|info| Some(&info.ty)) != expected_function_blueprint.return_value.and_then(|info| Some(&info.ty)) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                true
            },
            Type::TypeRef(_) => unreachable!(),
        }
    }

    pub fn is_ambiguous(&self) -> bool {
        match self {
            Type::Void => false,
            Type::Any => true,
            Type::Associated(_) => false,
            Type::Parameter(_) => false,
            Type::Actual(info) => info.parameters.iter().any(|ty| ty.is_ambiguous()),
            Type::TypeRef(ty) => ty.is_ambiguous(),
        }
    }

    pub fn get_maybe_static_method(&self, is_static: bool, method_name: &str) -> Option<&Link<FunctionBlueprint>> {
        match self {
            Type::Void => None,
            Type::Any => None,
            Type::Associated(associated_type) => None,
            Type::Parameter(parameter_type) => {
                for interface_blueprint in &parameter_type.borrow().required_interfaces {
                    let index_map = match is_static {
                        true => interface_blueprint.borrow().static_methods,
                        false => interface_blueprint.borrow().methods,
                    };

                    if let Some(function_blueprint) = index_map.get(method_name) {
                        return Some(&function_blueprint);
                    }
                }

                None
            },
            Type::Actual(info) => {
                let index_map = match is_static {
                    true => info.type_blueprint.borrow().static_methods,
                    false => info.type_blueprint.borrow().methods,
                };

                match index_map.get(method_name) {
                    Some(function_blueprint) => Some(&function_blueprint),
                    None => None,
                }
            },
            Type::TypeRef(ty) => ty.get_maybe_static_method(true, method_name)
        }
    }

    pub fn get_method(&self, method_name: &str) -> Option<&Link<FunctionBlueprint>> {
        self.get_maybe_static_method(false, method_name)
    }

    pub fn get_static_method(&self, method_name: &str) -> Option<&Link<FunctionBlueprint>> {
        self.get_maybe_static_method(true, method_name)
    }

    pub fn get_field(&self, field_name: &str) -> Option<&FieldDetails> {
        match self {
            Type::Void => None,
            Type::Any => None,
            Type::Associated(_) => None,
            Type::Parameter(_) => None,
            Type::Actual(info) => info.type_blueprint.borrow().fields.get(field_name),
            Type::TypeRef(_) => None,
        }
    }

    pub fn resolve(&self) -> ResolvedType {
        todo!()
    }
}

impl PartialEq for ActualTypeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.type_blueprint == other.type_blueprint && self.parameters == other.parameters
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Associated(l0), Self::Associated(r0)) => l0 == r0,
            (Self::Parameter(l0), Self::Parameter(r0)) => l0 == r0,
            (Self::Actual(l0), Self::Actual(r0)) => l0 == r0,
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
            Type::Associated(info) => write!(f, "{}", &info.borrow().name),
            Type::Parameter(info) => write!(f, "{}", &info.borrow().name),
            Type::Actual(info) => write!(f, "{}", &info.type_blueprint.borrow().name),
            Type::TypeRef(typeref) => write!(f, "<type {}>", &typeref),
        }
    }
}