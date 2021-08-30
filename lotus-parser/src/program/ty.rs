use std::{collections::HashMap, fmt, hash::Hash};
use crate::{generation::{NULL_ADDR, DEREF_FLOAT_POINTER_GET_FUNC_NAME, DEREF_INT_POINTER_GET_FUNC_NAME, DEREF_FLOAT_POINTER_SET_FUNC_NAME, DEREF_INT_POINTER_SET_FUNC_NAME, ToWat, ToWatVec, Wat}, items::{FullType, Identifier, ItemType, StructDeclaration, TypeSuffix, ValueType}, program::{ARRAY_ALLOC_FUNC_NAME, ARRAY_GET_LENGTH_FUNC_NAME, STRING_ALLOC_FUNC_NAME}, wat};
use super::{ProgramContext, StructAnnotation, StructInfo, Wasm};

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
    Pointer(Box<Type>),
    Struct(StructInfo),
    Array(Box<Type>),
    Function(Vec<Type>, Box<Type>),
    Any(u32)
}

impl Type {
    pub fn get_wasm_type(&self) -> Option<&'static str> {
        match self {
            Type::Void => None,
            Type::System => unreachable!(),
            Type::Boolean => Some("i32"),
            Type::Integer => Some("i32"),
            Type::Float => Some("f32"),
            Type::String => Some("i32"),
            Type::Null => unreachable!(),
            Type::TypeId => Some("i32"),
            Type::Struct(_) => Some("i32"),
            Type::Pointer(_) => Some("i32"),
            Type::Function(_, _) => Some("i32"),
            Type::Array(_) => Some("i32"),
            Type::Any(_) => unreachable!(),
        }
    }

    pub fn pointer_get_function_name(&self) -> &'static str {
        match self.get_wasm_type() {
            Some("i32") => DEREF_INT_POINTER_GET_FUNC_NAME,
            Some("f32") => DEREF_FLOAT_POINTER_GET_FUNC_NAME,
            _ => unreachable!()
        }
    }

    pub fn pointer_set_function_name(&self) -> &'static str {
        match self.get_wasm_type() {
            Some("i32") => DEREF_INT_POINTER_SET_FUNC_NAME,
            Some("f32") => DEREF_FLOAT_POINTER_SET_FUNC_NAME,
            _ => unreachable!()
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
            Type::String => Wat::call(STRING_ALLOC_FUNC_NAME, vec![Wat::const_i32(0)]),
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
                None => match context.get_struct_by_name(&value_type.name) {
                    Some(annotation) => Self::Struct(annotation.to_struct_info()),
                    None => {
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
            Type::Void => &Type::Void,
            _ => unreachable!()
        }
    }

    pub fn int_pointer() -> Self {
        Type::pointer(Type::Integer)
    }

    pub fn pointer(pointed_type: Type) -> Self {
        Type::Pointer(Box::new(pointed_type))
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

    pub fn is_float(&self) -> bool {
        match self {
            Self::Float => true,
            _ => false
        }
    }

    pub fn is_boolean(&self) -> bool {
        match self {
            Self::Boolean => true,
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

    fn is_any(&self) -> bool {
        match self {
            Type::Any(_) => true,
            _ => false
        }
    }

    pub fn is_compatible(&self, other: &Type, context: &ProgramContext) -> bool {
        self.is_assignable(other, context, &mut HashMap::new()) || other.is_assignable(self, context, &mut HashMap::new())
    }

    pub fn is_assignable(&self, actual: &Type, context: &ProgramContext, anonymous_types: &mut HashMap<u32, Type>) -> bool {
        if actual.is_any() {
            return true;
        }

        match self {
            Type::Void => actual == &Type::Void,
            Type::System => actual == &Type::System,
            Type::Boolean => actual == &Type::Boolean,
            Type::Integer => actual == &Type::Integer,
            Type::Float => actual == &Type::Float,
            Type::String => actual == &Type::String,
            Type::Null => actual == &Type::Null,
            Type::TypeId => actual == &Type::TypeId,
            Type::Struct(info) => match actual {
                Type::Struct(actual_info) => context.get_struct_by_id(actual_info.id).unwrap().types.contains(&info.id),
                Type::Null => true,
                _ => false
            },
            Type::Pointer(_) => actual.is_pointer(),
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

    pub fn to_bool(&self) -> Option<Wasm> {
        let wat = match self {
            Type::Boolean => wat!["nop"],
            Type::Integer => wat!["i32.ne", Wat::const_i32(i32::MIN)],
            Type::Float => wat!["i32.ne", wat!["i32.reinterpret_f32"], wat!["i32.reinterpret_f32", wat!["f32.const", "nan"]]],
            Type::String => wat!["i32.ne", Wat::call(ARRAY_GET_LENGTH_FUNC_NAME, vec![]), Wat::const_i32(0)],
            Type::Struct(_) => wat!["i32.ne", Wat::const_i32(NULL_ADDR)],
            Type::Pointer(_) => wat!["i32.ne", Wat::const_i32(0)],
            Type::Null => Wat::const_i32(0),
            Type::Array(_) => wat!["i32.ne", Wat::call(ARRAY_GET_LENGTH_FUNC_NAME, vec![]), Wat::const_i32(0)],
            _ => return None
        };

        Some(Wasm::simple(Type::Boolean, wat))
    }

    pub fn is_ambiguous(&self) -> bool {
        match self {
            Type::Void => unreachable!(),
            Type::System => unreachable!(),
            Type::Boolean => false,
            Type::Integer => false,
            Type::Float => false,
            Type::String => false,
            Type::Null => true,
            Type::TypeId => false,
            Type::Struct(_) => false,
            Type::Pointer(_) => false,
            Type::Array(item_type) => item_type.is_ambiguous(),
            Type::Function(_, _) => todo!(),
            Type::Any(_) => true,
        }
    }

    pub fn get_wasm_generated_method_name(&self, suffix: &str) -> String {
        match self {
            Type::Void => unreachable!(),
            Type::System => unreachable!(),
            Type::Boolean => format!("bool_{}", suffix),
            Type::Integer => format!("int_{}", suffix),
            Type::Float => format!("float_{}", suffix),
            Type::String => format!("string_{}", suffix),
            Type::Null => unreachable!(),
            Type::TypeId => todo!(),
            Type::Struct(struct_info) => format!("{}{}_{}", struct_info.name, struct_info.id, suffix),
            Type::Pointer(_) => unreachable!(),
            Type::Array(item_type) => format!("array_{}", item_type.get_wasm_generated_method_name(suffix)),
            Type::Function(_, _) => todo!(),
            Type::Any(_) => unreachable!(),
        }
    }

    pub fn is_stored_on_heap(&self) -> bool {
        match self {
            Type::Void => unreachable!(),
            Type::System => unreachable!(),
            Type::Boolean => false,
            Type::Integer => false,
            Type::Float => false,
            Type::String => true,
            Type::Null => unreachable!(),
            Type::TypeId => false,
            Type::Struct(_) => true,
            Type::Pointer(_) => unreachable!(),
            Type::Array(_) => true,
            Type::Function(_, _) => todo!(),
            Type::Any(_) => unreachable!(),
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
            Type::Struct(struct_info) => write!(f, "{}", &struct_info.name),
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

const STRUCT_OFFSET : u64 = 8;
const ARRAY_CONSTANT : u64 = 1234;

impl Eq for Type {

}

impl Hash for Type {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Type::Void => unreachable!(),
            Type::System => unreachable!(),
            Type::Boolean => 1u64.hash(state),
            Type::Integer => 2u64.hash(state),
            Type::Float => 3u64.hash(state),
            Type::String => 4u64.hash(state),
            Type::Null => unreachable!(),
            Type::TypeId => todo!(),
            Type::Struct(struct_info) => (STRUCT_OFFSET + (struct_info.id as u64)).hash(state),
            Type::Pointer(_) => unreachable!(),
            Type::Array(item_type) => {
                ARRAY_CONSTANT.hash(state);
                item_type.hash(state);
            },
            Type::Function(_, _) => todo!(),
            Type::Any(_) => unreachable!(),
        };
    }
}

impl Default for Type {
    fn default() -> Self {
        Self::Void
    }
}