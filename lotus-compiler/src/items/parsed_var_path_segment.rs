use std::collections::HashMap;
use parsable::parsable;
use crate::{program::{AccessType, FieldKind, ProgramContext, Type, Vasm}};
use super::{ParsedArgumentList, ParsedBracketIndexing, ParsedExpression, ParsedFieldOrMethodAccess, Identifier};

#[parsable]
pub enum ParsedVarPathSegment {
    FieldOrMethodAccess(ParsedFieldOrMethodAccess),
    BracketIndexing(ParsedBracketIndexing),
}

impl ParsedVarPathSegment {
    pub fn has_side_effects(&self) -> bool {
        match self {
            ParsedVarPathSegment::FieldOrMethodAccess(var_ref) => var_ref.has_side_effects(),
            ParsedVarPathSegment::BracketIndexing(_) => true,
        }
    }

    pub fn process(&self, parent_type: &Type, type_hint: Option<&Type>, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
        match self {
            ParsedVarPathSegment::FieldOrMethodAccess(var_ref) => var_ref.process(parent_type, FieldKind::Regular, type_hint, access_type, context),
            ParsedVarPathSegment::BracketIndexing(bracket_indexing) => bracket_indexing.process(parent_type, access_type, context),
        }
    }
}