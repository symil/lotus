use parsable::parsable;

use super::{expression::{Expression, Operand}, identifier::Identifier, struct_declaration::{Type}};

#[parsable]
pub enum Statement {
    VarDeclaration(VarDeclaration),
    Action(Action),
    If(IfBlock),
    While(WhileBlock),
    For(ForBlock),
    Assignment(Assignment)
}

#[parsable]
pub struct Assignment {
    pub lvalue: Operand,
    #[parsable(prefix="=")]
    pub rvalue: Option<Expression>
}

#[parsable]
pub struct Action {
    pub keyword: ActionKeyword,
    pub value: Expression
}

#[parsable]
pub enum ActionKeyword {
    Return = "return"
}

#[parsable]
pub struct IfBlock {
    #[parsable(prefix="if")]
    pub if_branch: Branch,
    #[parsable(prefix="else if", separator="else if", optional=true)]
    pub else_if_branches: Vec<Branch>,
    #[parsable(prefix="else")]
    pub else_branch: Option<Branch>
}

#[parsable]
pub struct WhileBlock {
    #[parsable(prefix="while")]
    pub while_branch: Branch
}

#[parsable]
pub struct ForBlock {
    #[parsable(prefix="for")]
    pub var_name: Identifier,
    #[parsable(prefix="in")]
    pub array_expression: Expression,
    #[parsable(brackets="{}")]
    pub statements: Vec<Statement>
}

#[parsable]
pub struct Branch {
    pub condition: Expression,
    #[parsable(brackets="{}")]
    pub statements: Vec<Statement>
}

#[parsable]
pub struct VarDeclaration {
    pub qualifier: Option<VarDeclarationQualifier>,
    pub var_type: Type,
    pub var_name: Identifier,
    #[parsable(prefix="=")]
    pub init_value: Expression
}

#[parsable(impl_display=true)]
#[derive(PartialEq)]
pub enum VarDeclarationQualifier {
    Const = "const"
}