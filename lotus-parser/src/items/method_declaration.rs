use parsable::parsable;

use super::{FunctionSignature, Identifier, Statement, VarPath};

#[parsable]
pub struct MethodDeclaration {
    pub qualifier: Option<MethodQualifier>,
    pub name: Identifier,
    #[parsable(brackets="[]", separator=",")]
    pub conditions: Vec<MethodCondition>,
    pub signature: Option<FunctionSignature>,
    #[parsable(brackets="{}")]
    pub statements: Vec<Statement>
}

#[parsable]
pub enum MethodQualifier {
    Builtin = "@",
    Hook = "`",
    Before = "'",
    After= "\""
}

#[parsable]
pub struct MethodCondition {
    pub left: VarPath,
    #[parsable(prefix="=")]
    pub right: Option<VarPath>
}