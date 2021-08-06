use parsable::parsable;

use super::{Expression, Identifier, AnyType, VarDeclarationQualifier};

#[parsable]
pub struct VarDeclaration {
    pub qualifier: Option<VarDeclarationQualifier>,
    pub var_type: AnyType,
    pub var_name: Identifier,
    #[parsable(prefix="=")]
    pub init_value: Expression
}