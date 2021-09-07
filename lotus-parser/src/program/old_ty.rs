use std::{collections::HashMap, fmt, hash::Hash};
use crate::{generation::{NULL_ADDR, DEREF_FLOAT_POINTER_GET_FUNC_NAME, DEREF_INT_POINTER_GET_FUNC_NAME, DEREF_FLOAT_POINTER_SET_FUNC_NAME, DEREF_INT_POINTER_SET_FUNC_NAME, ToWat, ToWatVec, Wat}, items::{FullType, Identifier, ItemType, StructDeclaration, TypeSuffix, ValueType}, program::{ARRAY_ALLOC_FUNC_NAME, ARRAY_GET_LENGTH_FUNC_NAME, STRING_ALLOC_FUNC_NAME}, wat};
use super::{ProgramContext, StructInfo, Wasm};

#[derive(Debug, Clone, PartialEq)]
pub enum TypeOld {
    Void,
    System,
    Boolean,
    Integer,
    Float,
    String,
    Null,
    Pointer(Box<TypeOld>),
    TypeRef(StructInfo),
    Struct(StructInfo),
    Array(Box<TypeOld>),
    Function(Vec<TypeOld>, Box<TypeOld>),
    Generic(String),
    Any(u32)
}

impl TypeOld {
    pub fn get_wasm_type(&self) -> Option<&'static str> {
        match self {
            TypeOld::Void => None,
            TypeOld::System => unreachable!(),
            TypeOld::Boolean => Some("i32"),
            TypeOld::Integer => Some("i32"),
            TypeOld::Float => Some("f32"),
            TypeOld::String => Some("i32"),
            TypeOld::Null => unreachable!(),
            TypeOld::Generic(_) => Some("i32"),
            TypeOld::TypeRef(_) => Some("i32"),
            TypeOld::Struct(_) => Some("i32"),
            TypeOld::Pointer(_) => Some("i32"),
            TypeOld::Function(_, _) => Some("i32"),
            TypeOld::Array(_) => Some("i32"),
            TypeOld::Any(_) => unreachable!(),
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
            TypeOld::Void => unreachable!(),
            TypeOld::System => unreachable!(),
            TypeOld::Pointer(_) => Wat::const_i32(NULL_ADDR),
            TypeOld::Boolean => Wat::const_i32(0),
            TypeOld::Integer => Wat::const_i32(0),
            TypeOld::Float => Wat::const_f32(0.),
            TypeOld::String => Wat::call(STRING_ALLOC_FUNC_NAME, vec![Wat::const_i32(0)]),
            TypeOld::Null => unreachable!(),
            TypeOld::Generic(_) => Wat::const_i32(0),
            TypeOld::TypeRef(_) => unreachable!(),
            TypeOld::Struct(_) => Wat::const_i32(NULL_ADDR),
            TypeOld::Function(_, _) => unreachable!(),
            TypeOld::Array(_) => Wat::call(ARRAY_ALLOC_FUNC_NAME, vec![Wat::const_i32(0)]),
            TypeOld::Any(_) => unreachable!(),
        };

        vec![item]
    }

    pub fn builtin_from_str(name: &str) -> Option<Self> {
        match name {
            "ptr" => Some(Self::Pointer(Box::new(TypeOld::Integer))),
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
                    Some(annotation) => Self::Struct(annotation.get_struct_info()),
                    None => {
                        context.errors.add(&value_type.name, format!("undefined type: {}", &value_type.name));
                        return None
                    },
                },
            },
            ItemType::Function(function_type) => {
                let mut ok = true;
                let mut arguments = vec![];
                let mut return_type = TypeOld::Void;

                for arg in &function_type.arguments {
                    if let Some(arg_type) = Self::from_parsed_type(arg, context){
                        arguments.push(arg_type);
                    } else {
                        arguments.push(TypeOld::Void);
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

                TypeOld::function(arguments, return_type)
            },
        };

        let mut final_type = item_type;

        for suffix in &ty.suffix {
            final_type = match suffix {
                TypeSuffix::Array => Self::Array(Box::new(final_type)),
            }
        }

        Some(final_type)
    }

    pub fn leaf_item_type(&self) -> &Self {
        match self {
            TypeOld::Array(sub_type) => sub_type.leaf_item_type(),
            _ => self
        }
    }

    pub fn get_item_type(&self) -> &Self {
        match self {
            TypeOld::Array(sub_type) => Box::as_ref(sub_type),
            TypeOld::Void => &TypeOld::Void,
            _ => unreachable!()
        }
    }

    pub fn int_pointer() -> Self {
        TypeOld::pointer(TypeOld::Integer)
    }

    pub fn any_pointer() -> Self {
        TypeOld::pointer(TypeOld::Any(0))
    }

    pub fn pointer(pointed_type: TypeOld) -> Self {
        TypeOld::Pointer(Box::new(pointed_type))
    }

    pub fn array(item_type: TypeOld) -> Self {
        TypeOld::Array(Box::new(item_type))
    }

    pub fn function(arguments: Vec<TypeOld>, return_type: TypeOld) -> Self {
        TypeOld::Function(arguments, Box::new(return_type))
    }

    pub fn as_function(&self) -> (&[TypeOld], &TypeOld) {
        match self {
            TypeOld::Function(arguments, return_type) => (arguments, return_type),
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

    pub fn is_type(&self) -> bool {
        match self {
            Self::TypeRef(_) => true,
            _ => false
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            TypeOld::Array(_) => true,
            _ => false
        }
    }

    fn is_pointer(&self) -> bool {
        match self {
            TypeOld::Pointer(_) => true,
            _ => false
        }
    }

    fn is_any(&self) -> bool {
        match self {
            TypeOld::Any(_) => true,
            _ => false
        }
    }

    pub fn is_assignable(&self) -> bool {
        match self {
            TypeOld::Void => true,
            TypeOld::System => false,
            TypeOld::Boolean => true,
            TypeOld::Integer => true,
            TypeOld::Float => true,
            TypeOld::String => true,
            TypeOld::Null => true,
            TypeOld::Generic(_) => true,
            TypeOld::Pointer(_) => true,
            TypeOld::TypeRef(_) => false,
            TypeOld::Struct(_) => true,
            TypeOld::Array(_) => true,
            TypeOld::Function(_, _) => true,
            TypeOld::Any(_) => true,
        }
    }

    pub fn is_compatible(&self, other: &TypeOld, context: &ProgramContext) -> bool {
        self.is_assignable_to(other, context, &mut HashMap::new()) || other.is_assignable_to(self, context, &mut HashMap::new())
    }

    pub fn is_assignable_to(&self, actual: &TypeOld, context: &ProgramContext, anonymous_types: &mut HashMap<u32, TypeOld>) -> bool {
        if actual.is_any() {
            return true;
        }

        match self {
            TypeOld::Void => actual == &TypeOld::Void,
            TypeOld::System => actual == &TypeOld::System,
            TypeOld::Boolean => actual == &TypeOld::Boolean,
            TypeOld::Integer => actual == &TypeOld::Integer,
            TypeOld::Float => actual == &TypeOld::Float,
            TypeOld::String => actual == &TypeOld::String,
            TypeOld::Null => actual == &TypeOld::Null,
            TypeOld::TypeRef(struct_info) => unreachable!(),
            TypeOld::Struct(info) => match actual {
                TypeOld::Struct(actual_info) => context.get_struct_by_id(actual_info.id).unwrap().types.contains(&info.id),
                TypeOld::Null => true,
                _ => false
            },
            TypeOld::Pointer(_) => actual.is_pointer(),
            TypeOld::Function(expected_argument_types, expected_return_type) => match actual {
                TypeOld::Function(actual_argument_types, actual_return_type) => {
                    if actual_argument_types.len() != expected_argument_types.len() {
                        false
                    } else if actual_return_type != expected_return_type {
                        false
                    } else {
                        let mut ok = true;

                        for (actual_arg_type, expected_arg_type) in actual_argument_types.iter().zip(expected_argument_types.iter()) {
                            if !expected_arg_type.is_assignable_to(actual_arg_type, context, anonymous_types) {
                                ok = false;
                            }
                        }

                        ok
                    }
                },
                _ => false
            },
            TypeOld::Array(expected_item_type) => match actual {
                TypeOld::Array(actual_item_type) => expected_item_type.is_assignable_to(actual_item_type, context, anonymous_types),
                _ => false
            },
            TypeOld::Generic(name) => {
                match actual {
                    TypeOld::Generic(actual_name) => name == actual_name,
                    _ => false
                }
            },
            TypeOld::Any(id) => {
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
            TypeOld::Boolean => wat!["nop"],
            TypeOld::Integer => wat!["i32.ne", Wat::const_i32(i32::MIN)],
            TypeOld::Float => wat!["i32.ne", wat!["i32.reinterpret_f32"], wat!["i32.reinterpret_f32", wat!["f32.const", "nan"]]],
            TypeOld::String => wat!["i32.ne", Wat::call(ARRAY_GET_LENGTH_FUNC_NAME, vec![]), Wat::const_i32(0)],
            TypeOld::Struct(_) => wat!["i32.ne", Wat::const_i32(NULL_ADDR)],
            TypeOld::Pointer(_) => wat!["i32.ne", Wat::const_i32(0)],
            TypeOld::Null => Wat::const_i32(0),
            TypeOld::Array(_) => wat!["i32.ne", Wat::call(ARRAY_GET_LENGTH_FUNC_NAME, vec![]), Wat::const_i32(0)],
            _ => return None
        };

        Some(Wasm::simple(TypeOld::Boolean, wat))
    }

    pub fn is_ambiguous(&self) -> bool {
        match self {
            TypeOld::Void => unreachable!(),
            TypeOld::System => unreachable!(),
            TypeOld::Boolean => false,
            TypeOld::Integer => false,
            TypeOld::Float => false,
            TypeOld::String => false,
            TypeOld::Null => true,
            TypeOld::Generic(_) => false,
            TypeOld::TypeRef(_) => false,
            TypeOld::Struct(_) => false,
            TypeOld::Pointer(_) => false,
            TypeOld::Array(item_type) => item_type.is_ambiguous(),
            TypeOld::Function(_, _) => todo!(),
            TypeOld::Any(_) => true,
        }
    }
}

impl fmt::Display for TypeOld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeOld::Void => write!(f, "<void>"),
            TypeOld::System => write!(f, "<system>"),
            TypeOld::Boolean => write!(f, "bool"),
            TypeOld::Integer => write!(f, "int"),
            TypeOld::Float => write!(f, "float"),
            TypeOld::String => write!(f, "string"),
            TypeOld::Null => write!(f, "<null>"),
            TypeOld::Generic(name) => write!(f, "{}", name),
            TypeOld::TypeRef(struct_info) => write!(f, "<type {}>", &struct_info.name),
            TypeOld::Struct(struct_info) => write!(f, "{}", &struct_info.name),
            TypeOld::Pointer(_) => write!(f, "ptr"),
            TypeOld::Array(item_type) => write!(f, "{}[]", item_type),
            TypeOld::Function(arguments, return_type) => {
                let args_joined = arguments.iter().map(|arg| format!("{}", arg)).collect::<Vec<String>>().join(",");
                let return_type_str = match Box::as_ref(return_type) {
                    TypeOld::Void => String::new(),
                    _ => format!("({})", return_type)
                };

                write!(f, "fn({}){}", args_joined, return_type_str)
            },
            TypeOld::Any(id) => match id {
                0 => write!(f, "<any>"),
                _ => write!(f, "<any.{}>", id),
            }
        }
    }
}

const STRUCT_OFFSET : u64 = 8;
const ARRAY_CONSTANT : u64 = 1234;

impl Eq for TypeOld {

}

impl Hash for TypeOld {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            TypeOld::Void => unreachable!(),
            TypeOld::System => unreachable!(),
            TypeOld::Boolean => 1u64.hash(state),
            TypeOld::Integer => 2u64.hash(state),
            TypeOld::Float => 3u64.hash(state),
            TypeOld::String => 4u64.hash(state),
            TypeOld::Null => unreachable!(),
            TypeOld::Generic(_) => unreachable!(),
            TypeOld::TypeRef(_) => todo!(),
            TypeOld::Struct(struct_info) => (STRUCT_OFFSET + (struct_info.id as u64)).hash(state),
            TypeOld::Pointer(_) => unreachable!(),
            TypeOld::Array(item_type) => {
                ARRAY_CONSTANT.hash(state);
                item_type.hash(state);
            },
            TypeOld::Function(_, _) => todo!(),
            TypeOld::Any(_) => unreachable!(),
        };
    }
}

impl Default for TypeOld {
    fn default() -> Self {
        Self::Void
    }
}