use parsable::parsable;

use super::{Action, Assignment, Expression, ForBlock, Identifier, IfBlock, Operand, AnyType, VarDeclaration, WhileBlock};

#[parsable]
pub enum Statement {
    VarDeclaration(VarDeclaration),
    Action(Action),
    If(IfBlock),
    While(WhileBlock),
    For(ForBlock),
    Assignment(Assignment)
}