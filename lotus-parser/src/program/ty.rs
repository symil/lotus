use std::{collections::HashMap, fmt};
use crate::{generation::{ARRAY_ALLOC_FUNC_NAME, ARRAY_GET_F32_FUNC_NAME, ARRAY_GET_I32_FUNC_NAME, ARRAY_LENGTH_FUNC_NAME, ARRAY_SET_F32_FUNC_NAME, ARRAY_SET_I32_FUNC_NAME, NULL_ADDR, DEREF_FLOAT_POINTER_GET_FUNC_NAME, DEREF_INT_POINTER_GET_FUNC_NAME, DEREF_FLOAT_POINTER_SET_FUNC_NAME, DEREF_INT_POINTER_SET_FUNC_NAME, ToWat, ToWatVec, Wat}, items::{FullType, Identifier, ItemType, StructDeclaration, TypeSuffix, ValueType}, wat};
use super::{ProgramContext, StructAnnotation};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Void,
    System,
    Boolean,
    Integer,
    Float,
    String,
    Null,
    TypeId,
    Struct(Identifier),
    Pointer(Box<Type>),
    Array(Box<Type>),
    Function(Vec<Type>, Box<Type>),
    Any(u32)
}

impl Type {
    pub fn get_wasm_type(&self) -> Option<&'static str> {
        match self {
            Type::Void => None,
            Type::System => None,
            Type::Boolean => Some("i32"),
            Type::Integer => Some("i32"),
            Type::Float => Some("f32"),
            Type::String => Some("i32"),
            Type::Null => Some("i32"),
            Type::TypeId => Some("i32"),
            Type::Struct(_) => Some("i32"),
            Type::Pointer(_) => Some("i32"),
            Type::Function(_, _) => Some("i32"),
            Type::Array(_) => Some("i32"),
            Type::Any(_) => unreachable!(),
        }
    }

    pub fn pointer_get_function_name(&self) -> &'static str {
        match self {
            Type::Float => DEREF_FLOAT_POINTER_GET_FUNC_NAME,
            _ => DEREF_INT_POINTER_GET_FUNC_NAME
        }
    }

    pub fn pointer_set_function_name(&self) -> &'static str {
        match self {
            Type::Float => DEREF_FLOAT_POINTER_SET_FUNC_NAME,
            _ => DEREF_INT_POINTER_SET_FUNC_NAME
        }
    }

    pub fn get_default_wat(&self) -> Vec<Wat> {
        let item = match self {
            Type::Void => unreachable!(),
            Type::System => unreachable!(),
            Type::Pointer(_) => Wat::const_i32(NULL_ADDR),
            Type::Boolean => Wat::const_i32(0),
            Type::Integer => Wat::const_i32(0),
            Type::Float => Wat::const_f32(0.),
            Type::String => Wat::call(ARRAY_ALLOC_FUNC_NAME, vec![Wat::const_i32(0)]),
            Type::Null => unreachable!(),
            Type::TypeId => Wat::const_i32(0),
            Type::Struct(_) => Wat::const_i32(NULL_ADDR),
            Type::Function(_, _) => unreachable!(),
            Type::Array(_) => Wat::call(ARRAY_ALLOC_FUNC_NAME, vec![Wat::const_i32(0)]),
            Type::Any(_) => unreachable!(),
        };

        vec![item]
    }

    pub fn builtin_from_str(name: &str) -> Option<Self> {
        match name {
            "ptr" => Some(Self::Pointer(Box::new(Type::Integer))),
            "bool" => Some(Self::Boolean),
            "int" => Some(Self::Integer),
            "float" => Some(Self::Float),
            "string" => Some(Self::String),
            _ => None
        }
    }

    pub fn from_parsed_type(ty: &FullType, context: &mut ProgramContext) -> Option<Self> {
        let item_type = match &ty.item {
            ItemType::Value(value_type) => match Self::builtin_from_str(value_type.name.as_str()) {
                Some(builtin_type) => builtin_type,
                None => match context.structs.contains_key(&value_type.name) {
                    true => Self::Struct(value_type.name.clone()),
                    false => {
                        context.error(&value_type.name, format!("undefined type: {}", &value_type.name));
                        return None
                    },
                },
            },
            ItemType::Function(function_type) => {
                let mut ok = true;
                let mut arguments = vec![];
                let mut return_type = Type::Void;

                for arg in &function_type.arguments {
                    if let Some(arg_type) = Self::from_parsed_type(arg, context){
                        arguments.push(arg_type);
                    } else {
                        arguments.push(Type::Void);
                        ok = false;
                    }
                }

                if let Some(ret) = &function_type.return_value {
                    if let Some(ret_type) = Self::from_parsed_type(Box::as_ref(ret), context) {
                        return_type = ret_type;
                    } else {
                        ok = false;
                    }
                }

                if !ok {
                    return None;
                }

                Type::function(arguments, return_type)
            },
        };

        let final_type = match &ty.suffix {
            Some(TypeSuffix::Array) => Self::Array(Box::new(item_type)),
            None => item_type
        };

        Some(final_type)
    }

    pub fn leaf_item_type(&self) -> &Self {
        match self {
            Type::Array(sub_type) => sub_type.leaf_item_type(),
            _ => self
        }
    }

    pub fn get_item_type(&self) -> &Self {
        match self {
            Type::Array(sub_type) => Box::as_ref(sub_type),
            _ => unreachable!()
        }
    }

    pub fn int_pointer() -> Self {
        Type::pointer(Type::Integer)
    }

    pub fn pointer(pointed_type: Type) -> Self {
        Type::Pointer(Box::new(pointed_type))
    }

    pub fn object(name: &Identifier) -> Self {
        Type::Struct(name.clone())
    }

    pub fn array(item_type: Type) -> Self {
        Type::Array(Box::new(item_type))
    }

    pub fn function(arguments: Vec<Type>, return_type: Type) -> Self {
        Type::Function(arguments, Box::new(return_type))
    }

    pub fn as_function(&self) -> (&[Type], &Type) {
        match self {
            Type::Function(arguments, return_type) => (arguments, return_type),
            _ => unreachable!()
        }
    }

    pub fn is_void(&self) -> bool {
        match self {
            Self::Void => true,
            _ => false
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            Self::Integer => true,
            _ => false
        }
    }

    pub fn is_boolean(&self) -> bool {
        match self {
            Self::Boolean => true,
            _ => false
        }
    }

    pub fn is_struct(&self, struct_name: &Identifier) -> bool {
        match self {
            Type::Struct(self_struct_name) => self_struct_name == struct_name,
            _ => false
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            Type::Array(_) => true,
            _ => false
        }
    }

    fn is_pointer(&self) -> bool {
        match self {
            Type::Pointer(_) => true,
            _ => false
        }
    }

    pub fn is_compatible(&self, other: &Type, context: &ProgramContext) -> bool {
        self.is_assignable(other, context, &mut HashMap::new()) || other.is_assignable(self, context, &mut HashMap::new())
    }

    pub fn is_assignable(&self, actual: &Type, context: &ProgramContext, anonymous_types: &mut HashMap<u32, Type>) -> bool {
        match self {
            Type::Void => actual == &Type::Void,
            Type::System => actual == &Type::System,
            Type::Boolean => actual == &Type::Boolean,
            Type::Integer => actual == &Type::Integer,
            Type::Float => actual == &Type::Float,
            Type::String => actual == &Type::String,
            Type::Null => actual == &Type::Null,
            Type::TypeId => actual == &Type::TypeId,
            Type::Struct(struct_name) => match actual {
                Type::Struct(actual_struct_name) => context.structs.get(actual_struct_name).unwrap().types.contains(struct_name),
                Type::Null => true,
                _ => false
            },
            Type::Pointer(_) => actual.is_pointer() || actual == &Type::Integer,
            Type::Function(expected_argument_types, expected_return_type) => match actual {
                Type::Function(actual_argument_types, actual_return_type) => {
                    if actual_argument_types.len() != expected_argument_types.len() {
                        false
                    } else if actual_return_type != expected_return_type {
                        false
                    } else {
                        let mut ok = true;

                        for (actual_arg_type, expected_arg_type) in actual_argument_types.iter().zip(expected_argument_types.iter()) {
                            if !expected_arg_type.is_assignable(actual_arg_type, context, anonymous_types) {
                                ok = false;
                            }
                        }

                        ok
                    }
                },
                _ => false
            },
            Type::Array(expected_item_type) => match actual {
                Type::Array(actual_item_type) => expected_item_type.is_assignable(actual_item_type, context, anonymous_types),
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

    pub fn to_bool(&self) -> Option<Wat> {
        match self {
            Type::Boolean => Some(wat!["nop"]),
            Type::Integer => Some(wat!["i32.ne", Wat::const_i32(i32::MIN)]),
            Type::Float => Some(wat!["i32.ne", wat!["i32.reinterpret_f32"], wat!["i32.reinterpret_f32", wat!["f32.const", "nan"]]]),
            Type::String => Some(wat!["i32.ne", Wat::call(ARRAY_LENGTH_FUNC_NAME, vec![]), Wat::const_i32(0)]),
            Type::Struct(_) => Some(wat!["i32.ne", Wat::const_i32(NULL_ADDR)]),
            Type::Pointer(_) => Some(wat!["i32.ne", Wat::const_i32(0)]),
            Type::Null => Some(Wat::const_i32(0)),
            Type::Array(_) => Some(wat!["i32.ne", Wat::call(ARRAY_LENGTH_FUNC_NAME, vec![]), Wat::const_i32(0)]),
            _ => None
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Void => write!(f, "<void>"),
            Type::System => write!(f, "<system>"),
            Type::Boolean => write!(f, "bool"),
            Type::Integer => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::String => write!(f, "string"),
            Type::Null => write!(f, "<null>"),
            Type::TypeId => write!(f, "type"),
            Type::Struct(struct_name) => write!(f, "{}", struct_name),
            Type::Pointer(_) => write!(f, "ptr"),
            Type::Array(item_type) => write!(f, "{}[]", item_type),
            Type::Function(arguments, return_type) => {
                let args_joined = arguments.iter().map(|arg| format!("{}", arg)).collect::<Vec<String>>().join(",");
                let return_type_str = match Box::as_ref(return_type) {
                    Type::Void => String::new(),
                    _ => format!("({})", return_type)
                };

                write!(f, "fn({}){}", args_joined, return_type_str)
            },
            Type::Any(id) => match id {
                0 => write!(f, "<any>"),
                _ => write!(f, "<any.{}>", id),
            }
        }
    }
}

impl Default for Type {
    fn default() -> Self {
        Self::Void
    }
}