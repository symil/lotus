use crate::items::FullType;
use super::ProgramContext;

#[derive(Debug, Clone)]
pub enum Type {
    Void,
    Generic(GenericInfo),
    Actual(TypeRef),
    TypeRef(TypeRef)
}

#[derive(Debug, Clone)]
pub struct TypeRef {
    pub type_id: u64, // blueprint id
    pub type_context: Option<u64>, // blueprint id
    pub generic_values: Vec<Type>
}

pub struct GenericInfo {
    pub name: String,
    pub type_context: u64
}

impl Type {
    pub fn from_parsed_type(ty: &FullType, context: &mut ProgramContext) -> Option<Self> {
        let mut item_type = None;

        match &ty.item {
            ItemType::Value(value_type) => {
                if let Some(type_blueprint) = context.types.get_by_name(&value_type.name) {
                    let mut ok = true;
                } else {
                    context.errors.add(&value_type.name, format!("undefined type: {}", &value_type.name));
                }
            },
            // match Self::builtin_from_str(value_type.name.as_str()) {
            //     Some(builtin_type) => builtin_type,
            //     None => match context.get_struct_by_name(&value_type.name) {
            //         Some(annotation) => Self::Struct(annotation.get_struct_info()),
            //         None => {
            //             context.errors.add(&value_type.name, format!("undefined type: {}", &value_type.name));
            //             return None
            //         },
            //     },
            // },
            ItemType::Function(function_type) => {
                todo!()
                // let mut ok = true;
                // let mut arguments = vec![];
                // let mut return_type = Type::Void;

                // for arg in &function_type.arguments {
                //     if let Some(arg_type) = Self::from_parsed_type(arg, context){
                //         arguments.push(arg_type);
                //     } else {
                //         arguments.push(Type::Void);
                //         ok = false;
                //     }
                // }

                // if let Some(ret) = &function_type.return_value {
                //     if let Some(ret_type) = Self::from_parsed_type(Box::as_ref(ret), context) {
                //         return_type = ret_type;
                //     } else {
                //         ok = false;
                //     }
                // }

                // if !ok {
                //     return None;
                // }

                // Type::function(arguments, return_type)
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
}