use parsable::parsable;

use crate::{generation::Wat, program::{BuiltinType, ItemType, ProgramContext, Type, Wasm, process_array_field_access, process_array_method_call, process_boolean_field_access, process_float_field_access, process_integer_field_access, process_pointer_field_access, process_string_field_access}};

use super::{ArgumentList, Expression, Identifier};

#[parsable]
pub enum VarPathSegment {
    #[parsable(prefix=".")]
    FieldAccess(Identifier),
    #[parsable(brackets="[]")]
    BracketIndexing(Expression),
    #[parsable(brackets="()", sep=",")]
    FunctionCall(ArgumentList)
}

impl VarPathSegment {
    pub fn is_function_call(&self) -> bool {
        match self {
            VarPathSegment::FunctionCall(_) => true,
            _ => false
        }
    }
}

pub fn process_field_access(parent_type: &Type, field_name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    let result = match parent_type {
        Type::Void => None,
        Type::Single(item_type) => match item_type {
            ItemType::Null => None,
            ItemType::Builtin(builtin_type) => match builtin_type {
                BuiltinType::Pointer => process_pointer_field_access(field_name, context),
                BuiltinType::Boolean => process_boolean_field_access(field_name, context),
                BuiltinType::Integer => process_integer_field_access(field_name, context),
                BuiltinType::Float => process_float_field_access(field_name, context),
                BuiltinType::String => process_string_field_access(field_name, context),
            },
            ItemType::Struct(struct_name) => {
                if field_name.is("_") {
                    // special case: `_` refers to the value itself rather than a field
                    // e.g `#foo` means `self.foo`, but `#_` means `self`
                    Some(Wasm::typed(parent_type.clone(), vec![]))
                } else if let Some(struct_annotation) = context.structs.get(struct_name) {
                    if let Some(field) = struct_annotation.fields.get(field_name) {
                        Some(field.get_expr_type())
                    } else if let Some(method) = struct_annotation.methods.get(field_name) {
                        Some(method.get_expr_type())
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            ItemType::Function(_, _) => None,
        },
        Type::Array(item_type) => process_array_field_access(item_type, field_name, context),
        Type::Any(_) => None,
        
    };

    if result.is_none() {
        context.error(field_name, format!("type `{}` has no field `{}`", parent_type, field_name));
    }

    result
}