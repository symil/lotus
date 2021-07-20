use parsable::parsable;

use super::{expr::{Expr, Operand, Operation}, identifier::Identifier};

#[parsable(located)]
pub enum Statement {
    VarDeclaration(VarDeclaration),
    #[parsable(prefix="return")]
    Return(Expr),
    If(IfBranch),
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
pub struct IfBranch {
    #[parsable(prefix="if")]
    pub if_branch: Branch,
    #[parsable(prefix="else if", separator="else if", optional=true)]
    pub else_if_branches: Vec<Branch>,
    #[parsable(prefix="else")]
    pub else_branch: Option<Branch>
}

#[parsable(located)]
pub struct Branch {
    pub condition: Operation,
    #[parsable(brackets="{}")]
    pub statements: Vec<Statement>
}

#[parsable(located)]
pub struct VarDeclaration {
    pub keyword: VarDeclarationKeyword,
    pub name: Identifier,
    #[parsable(prefix="=")]
    pub value: Operation
}

#[parsable(located)]
pub enum VarDeclarationKeyword {
    Let = "let",
    Const = "const"
}