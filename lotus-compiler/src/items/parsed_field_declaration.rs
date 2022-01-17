use parsable::parsable;
use crate::program::{Type, Vasm, ProgramContext};
use super::{ParsedExpression, ParsedType, Identifier, ParsedColon, ParsedEqual, ParsedComma, unwrap_item, ParsedVarTypeDeclaration, ParsedDefaultValueAssignment};

#[parsable]
pub struct ParsedFieldDeclaration {
    pub name: Identifier,
    pub ty: Option<ParsedVarTypeDeclaration>,
    pub default_value: Option<ParsedDefaultValueAssignment>,
    pub comma: Option<ParsedComma>,
}