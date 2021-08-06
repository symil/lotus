use parsable::parsable;

use super::{FunctionSignature, Identifier, Statement, AnyType};

#[parsable]
pub struct FunctionDeclaration {
    #[parsable(prefix="fn")]
    pub name: Identifier,
    pub signature: FunctionSignature,
    #[parsable(brackets="{}")]
    pub statements: Vec<Statement>
}