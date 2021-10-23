use std::collections::HashMap;
use parsable::parsable;
use crate::{program::{AccessType, FieldKind, ProgramContext, Type, Vasm}};
use super::{ArgumentList, BracketIndexing, Expression, FieldOrMethodAccess, Identifier, UnwrapToken};

#[parsable]
pub enum VarPathSegment {
    #[parsable(prefix=".")]
    FieldOrMethodAccess(FieldOrMethodAccess),
    BracketIndexing(BracketIndexing),
    Unwrap(UnwrapToken)
}

impl VarPathSegment {
    pub fn has_side_effects(&self) -> bool {
        match self {
            VarPathSegment::FieldOrMethodAccess(var_ref) => var_ref.has_side_effects(),
            VarPathSegment::BracketIndexing(_) => true,
            VarPathSegment::Unwrap(_) => true,
        }
    }

    pub fn process(&self, parent_type: &Type, field_kind: FieldKind, type_hint: Option<&Type>, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
        match self {
            VarPathSegment::FieldOrMethodAccess(var_ref) => var_ref.process(parent_type, field_kind, type_hint, access_type, context),
            VarPathSegment::BracketIndexing(bracket_indexing) => bracket_indexing.process(parent_type, access_type, context),
            VarPathSegment::Unwrap(unwrap_token) => unwrap_token.process(parent_type, context),
        }
    }
}