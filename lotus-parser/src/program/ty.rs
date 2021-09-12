use std::fmt::Display;
use parsable::DataLocation;
use crate::{generation::Wat, items::{FullType}, program::{PTR_SET_METHOD_NAME, THIS_TYPE_NAME, THIS_VAR_NAME}, utils::Link, wat};
use super::{AssociatedType, InterfaceAssociatedType, InterfaceBlueprint, InterfaceMethod, ProgramContext, ResolvedType, TypeBlueprint, TypeParameter};

#[derive(Debug, Clone)]
pub enum Type {
    Void,
    Any,
    Associated(Link<InterfaceAssociatedType>),
    Parameter(Link<TypeParameter>),
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
         
                for method in interface_blueprint.methods.values() {
                    if let Some(method_info) = type_blueprint.methods.get(method.name.as_str()) {
                        let function_blueprint = method_info.content.borrow();

                        if function_blueprint.arguments.len() != method.arguments.len() {
                            return false;
                        }

                        for (required_type, arg_info) in method.arguments.iter().zip(function_blueprint.arguments.iter()) {
                            if required_type != &arg_info.ty {
                                return false;
                            }
                        }

                        if function_blueprint.return_value.and_then(|info| Some(&info.ty)) != method.return_type.as_ref() {
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