use parsable::parsable;

use super::{Expression, Identifier, Type, VarDeclarationQualifier};

#[parsable]
pub struct VarDeclaration {
    pub qualifier: Option<VarDeclarationQualifier>,
    pub var_type: Type,
    pub var_name: Identifier,
    #[parsable(prefix="=")]
    pub init_value: Expression
}