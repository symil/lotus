use std::{collections::HashMap, fmt};

use crate::items::{identifier::Identifier, struct_declaration::{ParsedType, TypeSuffix}};

#[derive(Clone, Debug)]
pub enum ItemType {
    Builtin(BuiltinType),
    Struct(Identifier),
    Function(Vec<ExpressionType>, Box<ExpressionType>)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BuiltinType {
    Boolean,
    Number,
    String
}

#[derive(Clone, Debug)]
pub enum ExpressionType {
    Void,
    Single(ItemType),
    Array(Box<ExpressionType>),
    Anonymous(u32)
}

impl BuiltinType {
    pub fn from_identifier(identifier: &Identifier) -> Option<Self> {
        match identifier.as_str() {
            "bool" => Some(BuiltinType::Boolean),
            "num" => Some(BuiltinType::Number),
            "string" => Some(BuiltinType::String),
            _ => None
        }
    }
}

impl ExpressionType {
    pub fn from_value_type(parsed_type: &ParsedType) -> Self {
        let item_type = match BuiltinType::from_identifier(&parsed_type.name) {
            Some(builtin_type) => ItemType::Builtin(builtin_type),
            None => ItemType::Struct(parsed_type.name.clone()),
        };

        match parsed_type.suffix {
            Some(TypeSuffix::Array) => Self::Array(Box::new(Self::Single(item_type))),
            None => Self::Single(item_type),
        }
    }

    pub fn single_builtin(builtin_type: BuiltinType) -> Self {
        ExpressionType::Single(ItemType::Builtin(builtin_type))
    }

    pub fn array(item_type: ExpressionType) -> Self {
        ExpressionType::Array(Box::new(item_type))
    }

    pub fn function(arguments: Vec<ExpressionType>, return_type: ExpressionType) -> Self {
        ExpressionType::Single(ItemType::Function(arguments, Box::new(return_type)))
    }

    pub fn is_void(&self) -> bool {
        match self {
            ExpressionType::Void => true,
            _ => false
        }
    }

    pub fn is_single(&self, item_type: &ItemType) -> bool {
        match self {
            ExpressionType::Single(self_item_type) => self_item_type == item_type,
            _ => false
        }
    }

    pub fn is_array(&self, item_type: &ExpressionType) -> bool {
        match self {
            ExpressionType::Array(self_item_type) => Box::as_ref(self_item_type) == item_type,
            _ => false
        }
    }

    pub fn is_anonymous(&self, type_id: &u32) -> bool {
        match self {
            ExpressionType::Anonymous(self_type_id) => self_type_id == type_id,
            _ => false
        }
    }

    pub fn match_actual(&self, actual: &ExpressionType, anonymous_types: &mut HashMap<u32, ExpressionType>) -> bool {
        match self {
            ExpressionType::Void => actual.is_void(),
            ExpressionType::Single(expected_item_type) => match actual {
                ExpressionType::Single(actual_item_type) => expected_item_type.match_actual(actual_item_type, anonymous_types),
                _ => false
            },
            ExpressionType::Array(expected_item_type) => match actual {
                ExpressionType::Array(actual_item_type) => expected_item_type.match_actual(actual_item_type, anonymous_types),
                _ => false
            },
            ExpressionType::Anonymous(id) => {
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

impl Default for ExpressionType {
    fn default() -> Self {
        ExpressionType::Void
    }
}

impl ItemType {
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

    pub fn is_function(&self, argument_types: &[ExpressionType], return_type: &ExpressionType) -> bool {
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

    pub fn match_actual(&self, actual: &ItemType, anonymous_types: &mut HashMap<u32, ExpressionType>) -> bool {
        match self {
            ItemType::Builtin(expected_builtin) => actual.is_builtin(expected_builtin),
            ItemType::Struct(expected_struct) => actual.is_struct(expected_struct),
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
                                if !expected_arg_type.match_actual(actual_arg_type, anonymous_types) {
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
            ItemType::Builtin(value) => other.is_builtin(value),
            ItemType::Struct(value) => other.is_struct(value),
            ItemType::Function(args, ret) => other.is_function(args, ret),
        }
    }
}

impl PartialEq for ExpressionType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            ExpressionType::Void => other.is_void(),
            ExpressionType::Single(value) => other.is_single(value),
            ExpressionType::Array(value) => other.is_array(Box::as_ref(value)),
            ExpressionType::Anonymous(value) => other.is_anonymous(value),
        }
    }
}

impl fmt::Display for BuiltinType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuiltinType::Boolean => write!(f, "bool"),
            BuiltinType::Number => write!(f, "num"),
            BuiltinType::String => write!(f, "string"),
        }
    }
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ItemType::Builtin(builtin_type) => write!(f, "{}", builtin_type),
            ItemType::Struct(struct_name) => write!(f, "{}", struct_name),
            ItemType::Function(arguments, return_type) => {
                let args_joined = arguments.iter().map(|arg| format!("{}", arg)).collect::<Vec<String>>().join(",");
                let return_type_str = match Box::as_ref(return_type) {
                    ExpressionType::Void => String::new(),
                    _ => format!(" -> {}", return_type)
                };

                write!(f, "(fn({}){})", args_joined, return_type_str)
            },
        }
    }
}


impl fmt::Display for ExpressionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpressionType::Void => write!(f, "<void>"),
            ExpressionType::Single(item_type) => write!(f, "{}", item_type),
            ExpressionType::Array(item_type) => write!(f, "{}[]", item_type),
            ExpressionType::Anonymous(id) => write!(f, "<any.{}>", id),
        }
    }
}