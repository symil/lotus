use std::collections::HashMap;
use parsable::parsable;
use crate::{generation::{Wat}, program::{AccessType, ProgramContext, Type, Wasm, process_array_field_access, process_array_method_call, process_boolean_field_access, process_boolean_method_call, process_float_field_access, process_float_method_call, process_integer_field_access, process_integer_method_call, process_pointer_field_access, process_pointer_method_call, process_string_field_access, process_string_method_call}};
use super::{ArgumentList, BracketIndexing, Expression, Identifier, VarRef};

#[parsable]
pub enum VarPathSegment {
    #[parsable(prefix=".")]
    FieldOrMethodAccess(VarRef),
    BracketIndexing(BracketIndexing),
}

impl VarPathSegment {
    pub fn process(&self, parent_type: &Type, access_type: AccessType, context: &mut ProgramContext) -> Option<Wasm> {
        match self {
            VarPathSegment::FieldOrMethodAccess(var_ref) => var_ref.process_as_field(parent_type, access_type, context),
            VarPathSegment::BracketIndexing(bracket_indexing) => bracket_indexing.process(parent_type, access_type, context),
        }
    }
}