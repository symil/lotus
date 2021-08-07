use parsable::parsable;

use super::{Expression, Identifier, FullType, VarDeclarationQualifier};

#[parsable]
pub struct VarDeclaration {
    pub qualifier: Option<VarDeclarationQualifier>,
    pub var_type: FullType,
    pub var_name: Identifier,
    #[parsable(prefix="=")]
    pub init_value: Expression
}