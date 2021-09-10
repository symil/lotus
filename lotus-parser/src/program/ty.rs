use std::fmt::Display;
use parsable::DataLocation;
use crate::{generation::Wat, items::FullType, program::{PTR_SET_METHOD_NAME, THIS_TYPE_NAME, THIS_VAR_NAME}, wat};
use super::{InterfaceMethod, ProgramContext};

#[derive(Debug, Clone)]
pub enum Type {
    Void,
    Any,
    Associated(AssociatedTypeInfo),
    Generic(GenericTypeInfo),
    Actual(ActualTypeInfo),
    TypeRef(Box<Type>)
}

#[derive(Debug, Clone)]
pub struct AssociatedTypeInfo {
    pub name: String,
    pub interface_context: u64
}

#[derive(Debug, Clone)]
pub struct GenericTypeInfo {
    pub name: String,
    pub type_context: u64
}

#[derive(Debug, Clone)]
pub struct ActualTypeInfo {
    pub name: String, // only used for display
    pub type_id: u64, // blueprint id
    pub parameters: Vec<Type>
}

impl Type {
    pub fn generic(name: String, type_context: u64) -> Type {
        Type::Generic(GenericTypeInfo {
            name,
            type_context,
        })
    }

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
            Type::Generic(_) => true,
            _ => false
        }
    }

    pub fn is_actual(&self) -> bool {
        match self {
            Type::Actual(_) => true,
            _ => false
        }
    }

    pub fn get_wasm_type(&self, context: &ProgramContext) -> Option<String> {
        match self {
            Type::Void => None,
            Type::Any => unreachable!(),
            Type::Associated(info) => Some(format!("?{}", &info.name)),
            Type::Generic(info) => Some(format!("?{}", &info.name)),
            Type::Actual(info) => context.types.get_by_id(info.type_id).unwrap().get_wasm_type().and_then(|s| Some(s.to_string())),
            Type::TypeRef(_) => unreachable!(),
        }
    }

    pub fn is_assignable(&self) -> bool {
        match self {
            Type::Void => true,
            Type::Any => true,
            Type::Associated(_) => true,
            Type::Generic(_) => true,
            Type::Actual(_) => true,
            Type::TypeRef(_) => false,
        }
    }

    pub fn is_assignable_to(&self, target: &Type, context: &ProgramContext) -> bool {
        match self {
            Type::Void => false,
            Type::Any => true,
            Type::Associated(self_info) => match target {
                Type::Associated(target_info) => self_info == target_info,
                _ => false
            },
            Type::Generic(self_info) => {
                match target {
                    Type::Generic(target_info) => self_info == target_info,
                    _ => false
                }
            },
            Type::Actual(self_info) => {
                match target {
                    Type::Actual(target_info) => {
                        let self_type = context.types.get_by_id(self_info.type_id).unwrap();

                        self_type.inheritance_chain.contains(target_info)
                    },
                    _ => false
                }
            },
            Type::TypeRef(_) => false,
        }
    }

    pub fn get_placeholder(&self) -> Wat {
        match self {
            Type::Void => unreachable!(),
            Type::Any => unreachable!(),
            Type::Associated(info) => wat![&info.name],
            Type::Generic(info) => wat![&info.name],
            Type::Actual(info) => {
                let mut placeholder = wat![info.type_id];

                for ty in &info.parameters {
                    placeholder.push(ty.get_placeholder());
                }

                placeholder
            },
            Type::TypeRef(_) => unreachable!(),
        }
    }

    pub fn method_call_placeholder(&self, method_name: &str) -> Wat {
        wat!["@METHOD_CALL", self.get_placeholder()]
    }
    
    pub fn match_interface(&self, interface_id: u64, context: &ProgramContext) -> bool {
        match self {
            Type::Void => unreachable!(),
            Type::Any => unreachable!(),
            Type::Associated(info) => unreachable!(),
            Type::Generic(info) => {
                let context_type = context.types.get_by_id(info.type_context).unwrap();
                let parameter_info = context_type.parameters.get(&info.name).unwrap();

                parameter_info.required_interfaces.contains(&interface_id)
            },
            Type::Actual(info) => {
                let type_blueprint = context.types.get_by_id(info.type_id).unwrap();
                let interface_blueprint = context.interfaces.get_by_id(interface_id).unwrap();

                for associated_type in interface_blueprint.associated_types.values() {
                    if let Some(associated_type_info) = type_blueprint.associated_types.get(associated_type.name.as_str()) {
                        // later: check that associated type matches required interfaces
                    } else {
                        return false;
                    }
                }
         
                for method in interface_blueprint.methods.values() {
                    if let Some(method_info) = type_blueprint.methods.get(method.name.as_str()) {
                        let function_blueprint = context.functions.get_by_id(method_info.function_id).unwrap();

                        if function_blueprint.arguments.len() != method.arguments.len() {
                            return false;
                        }

                        for (required_type, (arg_name, arg_type)) in method.arguments.iter().zip(function_blueprint.arguments.iter()) {
                            if required_type != arg_type {
                                return false;
                            }
                        }

                        if function_blueprint.return_type != method.return_type {
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
}

impl PartialEq for ActualTypeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id && self.parameters == other.parameters
    }
}

impl PartialEq for GenericTypeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.type_context == other.type_context
    }
}

impl PartialEq for AssociatedTypeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.interface_context == other.interface_context
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Associated(l0), Self::Associated(r0)) => l0 == r0,
            (Self::Generic(l0), Self::Generic(r0)) => l0 == r0,
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
            Type::Associated(info) => write!(f, "{}", &info.name),
            Type::Generic(info) => write!(f, "{}", &info.name),
            Type::Actual(info) => write!(f, "{}", &info.name),
            Type::TypeRef(typeref) => write!(f, "<type {}>", &typeref),
        }
    }
}