use parsable::parsable;

use super::{identifier::Identifier, statement::Statement};

#[parsable(located)]
pub struct FunctionDeclaration {
    #[parsable(prefix="fn")]
    pub name: Identifier,
    #[parsable(brackets="()", separator=",")]
    pub arguments: Vec<FunctionArgument>,
    #[parsable(prefix="->")]
    pub return_type: Option<Identifier>,
    #[parsable(brackets="{}")]
    pub statements: Vec<Statement>
}

#[parsable(located)]
pub struct FunctionArgument {
    pub name: Identifier,
    #[parsable(prefix=":")]
    pub ty: Identifier
}