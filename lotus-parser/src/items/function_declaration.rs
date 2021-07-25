use parsable::parsable;

use super::{identifier::Identifier, statement::Statement, struct_declaration::Type};

#[parsable]
pub struct FunctionDeclaration {
    #[parsable(prefix="fn")]
    pub name: Identifier,
    pub signature: FunctionSignature,
    #[parsable(brackets="{}")]
    pub statements: Vec<Statement>
}

#[parsable]
pub struct FunctionSignature {
    #[parsable(brackets="()", separator=",")]
    pub arguments: Vec<FunctionArgument>,
    #[parsable(prefix="->")]
    pub return_type: Option<Type>,
}

#[parsable]
pub struct FunctionArgument {
    pub name: Identifier,
    #[parsable(prefix=":")]
    pub type_: Type
}