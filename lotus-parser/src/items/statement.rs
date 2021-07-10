use parsable::parsable;

use super::{expr::{Operand, Operation}, identifier::Identifier};

#[parsable(located)]
pub enum Statement {
    VarDeclaration(VarDeclaration),
    #[parsable(prefix="return")]
    Return(Operation),
    #[parsable(prefix="if")]
    If(Branch),
    #[parsable(prefix="else if")]
    ElseIf(Branch),
    #[parsable(prefix="else")]
    Else(Branch),
    #[parsable(prefix="while")]
    While(Branch),
    For(ForLoop)
}

#[parsable(located)]
pub struct ForLoop {
    #[parsable(prefix="for")]
    pub var_name: Identifier,
    #[parsable(prefix="in")]
    pub range: Operand,
    #[parsable(brackets="{}")]
    pub statements: Vec<Statement>
}

#[parsable(located)]
pub struct Branch {
    pub condition: Operation,
    #[parsable(brackets="{}")]
    pub statements: Vec<Statement>
}

#[parsable(located)]
pub struct VarDeclaration {
    pub keyword: DeclarationKeyword,
    pub name: Identifier,
    #[parsable(prefix="=")]
    pub value: Operation
}

#[parsable(located)]
pub enum DeclarationKeyword {
    Let = "let",
    Const = "const"
}