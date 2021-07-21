use parsable::parsable;

use super::{expr::{Expr, Operand, Operation}, identifier::Identifier};

#[parsable]
pub enum Statement {
    VarDeclaration(VarDeclaration),
    #[parsable(prefix="return")]
    Return(Expr),
    If(IfBranch),
    #[parsable(prefix="while")]
    While(Branch),
    For(ForLoop)
}

#[parsable]
pub struct ForLoop {
    #[parsable(prefix="for")]
    pub var_name: Identifier,
    #[parsable(prefix="in")]
    pub range: Operand,
    #[parsable(brackets="{}")]
    pub statements: Vec<Statement>
}

#[parsable]
pub struct IfBranch {
    #[parsable(prefix="if")]
    pub if_branch: Branch,
    #[parsable(prefix="else if", separator="else if", optional=true)]
    pub else_if_branches: Vec<Branch>,
    #[parsable(prefix="else")]
    pub else_branch: Option<Branch>
}

#[parsable]
pub struct Branch {
    pub condition: Operation,
    #[parsable(brackets="{}")]
    pub statements: Vec<Statement>
}

#[parsable]
pub struct VarDeclaration {
    pub qualifier: VarDeclarationQualifier,
    pub name: Identifier,
    #[parsable(prefix="=")]
    pub value: Operation
}

#[parsable]
#[derive(PartialEq)]
pub enum VarDeclarationQualifier {
    Let = "let",
    Const = "const"
}