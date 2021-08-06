use std::{collections::HashMap, fmt};

use crate::items::{Identifier, StructDeclaration, TypeSuffix, ValueType};

use super::StructAnnotation;

#[derive(Clone, Debug)]
pub enum ItemType {
    Null,
    Builtin(BuiltinType),
    Struct(Identifier),
    Function(Vec<Type>, Box<Type>)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BuiltinType {
    Pointer,
    Boolean,
    Integer,
    Float,
    String
}

#[derive(Clone, Debug)]
pub enum Type {
    Void,
    Single(ItemType),
    Array(Box<Type>),
    Any(u32)
}

impl BuiltinType {
    pub fn from_identifier(identifier: &Identifier) -> Option<Self> {
        match identifier.as_str() {
            "pointer" => Some(BuiltinType::Pointer),
            "bool" => Some(BuiltinType::Boolean),
            "int" => Some(BuiltinType::Integer),
            "float" => Some(BuiltinType::Float),
            "string" => Some(BuiltinType::String),
            _ => None
        }
    }
}

impl Type {
    pub fn from_value_type(value_type: &ValueType, structs: &HashMap<Identifier, StructDeclaration>) -> Option<Self> {
        let item_type = match BuiltinType::from_identifier(&value_type.name) {
            Some(builtin_type) => ItemType::Builtin(builtin_type),
            None => match structs.contains_key(&value_type.name) {
                true => ItemType::Struct(value_type.name.clone()),
                false => return None,
            },
        };

        let final_type = match value_type.suffix {
            Some(TypeSuffix::Array) => Self::Array(Box::new(Self::Single(item_type))),
            None => Self::Single(item_type),
        };

        Some(final_type)
    }

    pub fn item_type(&self) -> &ItemType {
        match self {
            Type::Void => unreachable!(),
            Type::Single(item_type) => item_type,
            Type::Array(sub_type) => sub_type.item_type(),
            Type::Any(_) => unreachable!(),
        }
    }

    pub fn int() -> Self {
        Type::Single(ItemType::Builtin(BuiltinType::Integer))
    }

    pub fn pointer() -> Self {
        Type::Single(ItemType::Builtin(BuiltinType::Pointer))
    }

    pub fn builtin(builtin_type: BuiltinType) -> Self {
        Type::Single(ItemType::Builtin(builtin_type))
    }

    pub fn builtin_array(builtin_type: BuiltinType) -> Self {
        Type::array(Type::builtin(builtin_type))
    }

    pub fn object(name: &Identifier) -> Self {
        Type::Single(ItemType::Struct(name.clone()))
    }

    pub fn array(item_type: Type) -> Self {
        Type::Array(Box::new(item_type))
    }

    pub fn function(arguments: Vec<Type>, return_type: Type) -> Self {
        Type::Single(ItemType::Function(arguments, Box::new(return_type)))
    }

    pub fn is_void(&self) -> bool {
        match self {
            Type::Void => true,
            _ => false
        }
    }

    pub fn is_single(&self, item_type: &ItemType) -> bool {
        match self {
            Type::Single(self_item_type) => self_item_type == item_type,
            _ => false
        }
    }

    pub fn is_array(&self, item_type: &Type) -> bool {
        match self {
            Type::Array(self_item_type) => Box::as_ref(self_item_type) == item_type,
            _ => false
        }
    }

    pub fn is_anonymous(&self, type_id: &u32) -> bool {
        match self {
            Type::Any(self_type_id) => self_type_id == type_id,
            _ => false
        }
    }

    pub fn match_actual(&self, actual: &Type, structs: &HashMap<Identifier, StructAnnotation>, anonymous_types: &mut HashMap<u32, Type>) -> bool {
        match self {
            Type::Void => actual.is_void(),
            Type::Single(expected_item_type) => match actual {
                Type::Single(actual_item_type) => expected_item_type.match_actual(actual_item_type, structs, anonymous_types),
                _ => false
            },
            Type::Array(expected_item_type) => match actual {
                Type::Array(actual_item_type) => expected_item_type.match_actual(actual_item_type, structs, anonymous_types),
                _ => false
            },
            Type::Any(id) => {
                if let Some(expected_type) = anonymous_types.get(id) {
                    actual == expected_type
                } else {
                    // TODO: not so sure about that; what happens if an anonymous type is registered?
                    anonymous_types.insert(*id, actual.clone());
                    true
                }
            },
        }
    }
}

impl Default for Type {
    fn default() -> Self {
        Type::Void
    }
}

impl ItemType {
    pub fn is_null(&self) -> bool {
        match self {
            ItemType::Null => true,
            _ => false
        }
    }

    pub fn is_builtin(&self, builtin_type: &BuiltinType) -> bool {
        match self {
            ItemType::Builtin(self_builtin_type) => self_builtin_type == builtin_type,
            _ => false
        }
    }

    pub fn is_struct(&self, struct_name: &Identifier) -> bool {
        match self {
            ItemType::Struct(self_struct_name) => self_struct_name == struct_name,
            _ => false
        }
    }

    pub fn is_function(&self, argument_types: &[Type], return_type: &Type) -> bool {
        match self {
            ItemType::Function(self_argument_types, self_return_type) => {
                if self_argument_types.len() != argument_types.len() {
                    false
                } else if Box::as_ref(self_return_type) != return_type {
                    false
                } else {
                    let mut ok = true;
        
                    for (actual_arg_type, expected_arg_type) in self_argument_types.iter().zip(argument_types.iter()) {
                        if actual_arg_type != expected_arg_type {
                            ok = false
                        }
                    }
        
                    ok
                }
            },
            _ => false
        }
    }

    pub fn match_actual(&self, actual: &ItemType, structs: &HashMap<Identifier, StructAnnotation>, anonymous_types: &mut HashMap<u32, Type>) -> bool {
        match self {
            ItemType::Null => actual.is_null(),
            ItemType::Builtin(expected_builtin) => actual.is_builtin(expected_builtin),
            ItemType::Struct(expected_struct) => match actual {
                ItemType::Null => true,
                ItemType::Struct(actual_struct) => structs.get(actual_struct).unwrap().types.contains(expected_struct),
                _ => false
            },
            ItemType::Function(expected_argument_types, expected_return_type) => {
                match actual {
                    ItemType::Function(actual_argument_types, actual_return_type) => {
                        if actual_argument_types.len() != expected_argument_types.len() {
                            false
                        } else if actual_return_type != expected_return_type {
                            false
                        } else {
                            let mut ok = true;

                            for (actual_arg_type, expected_arg_type) in actual_argument_types.iter().zip(expected_argument_types.iter()) {
                                if !expected_arg_type.match_actual(actual_arg_type, structs, anonymous_types) {
                                    ok = false;
                                }
                            }

                            ok
                        }
                    },
                    _ => false
                }
            },
        }
    }
}

impl PartialEq for ItemType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            ItemType::Null => other.is_null(),
            ItemType::Builtin(value) => other.is_builtin(value),
            ItemType::Struct(value) => other.is_struct(value),
            ItemType::Function(args, ret) => other.is_function(args, ret),
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Type::Void => other.is_void(),
            Type::Single(value) => other.is_single(value),
            Type::Array(value) => other.is_array(Box::as_ref(value)),
            Type::Any(value) => other.is_anonymous(value),
        }
    }
}

impl fmt::Display for BuiltinType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuiltinType::Pointer => write!(f, "pointer"),
            BuiltinType::Boolean => write!(f, "bool"),
            BuiltinType::Integer => write!(f, "int"),
            BuiltinType::Float => write!(f, "float"),
            BuiltinType::String => write!(f, "string"),
        }
    }
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ItemType::Null => write!(f, "null"),
            ItemType::Builtin(builtin_type) => write!(f, "{}", builtin_type),
            ItemType::Struct(struct_name) => write!(f, "{}", struct_name),
            ItemType::Function(arguments, return_type) => {
                let args_joined = arguments.iter().map(|arg| format!("{}", arg)).collect::<Vec<String>>().join(",");
                let return_type_str = match Box::as_ref(return_type) {
                    Type::Void => String::new(),
                    _ => format!(" -> {}", return_type)
                };

                write!(f, "(fn({}){})", args_joined, return_type_str)
            },
        }
    }
}


impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Void => write!(f, "<void>"),
            Type::Single(item_type) => write!(f, "{}", item_type),
            Type::Array(item_type) => write!(f, "{}[]", item_type),
            Type::Any(id) => write!(f, "<any.{}>", id),
        }
    }
}