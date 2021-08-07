use std::collections::HashMap;
use parsable::parsable;
use crate::{generation::{ARRAY_GET_I32_FUNC_NAME, Wat}, program::{ProgramContext, Type, Wasm, process_array_field_access, process_array_method_call, process_boolean_field_access, process_boolean_method_call, process_float_field_access, process_float_method_call, process_integer_field_access, process_integer_method_call, process_pointer_field_access, process_pointer_method_call, process_string_field_access, process_string_method_call}};
use super::{ArgumentList, Expression, Identifier, VarRef};

#[parsable]
pub enum VarPathSegment {
    #[parsable(prefix=".")]
    FieldOrMethodAccess(VarRef),
    #[parsable(brackets="[]")]
    BracketIndexing(Expression),
}

impl VarPathSegment {
    pub fn process(&self, parent_type: &Type, context: &mut ProgramContext) -> Option<Wasm> {
        match self {
            VarPathSegment::FieldOrMethodAccess(var_ref) => var_ref.process_as_field(parent_type, context),
            VarPathSegment::BracketIndexing(index_expr) => process_bracket_indexing(parent_type, index_expr, context),
        }
    }
}


pub fn process_bracket_indexing(parent_type: &Type, index_expr: &Expression, context: &mut ProgramContext) -> Option<Wasm> {
    let mut result = None;
    let mut indexing_ok = false;
    let mut wat = vec![];

    if let Some(index_wasm) = index_expr.process(context) {
        if &index_wasm.ty == &Type::Integer {
            indexing_ok = true;
        } else {
            context.error(index_expr, format!("bracket indexing argument: expected `{}`, got `{}`", Type::Integer, &index_wasm.ty));
        }

        wat.extend(index_wasm.wat);
    }

    if let Type::Array(item_type) = parent_type {
        wat.push(Wat::call(ARRAY_GET_I32_FUNC_NAME, vec![]));
        result = Some(Wasm::typed(Box::as_ref(item_type).clone(), wat))
    } else {
        context.error(index_expr, format!("bracket indexing target: expected array, got `{}`", parent_type));
    }

    match indexing_ok {
        true => result,
        false => None
    }
}