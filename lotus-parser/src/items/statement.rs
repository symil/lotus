use parsable::parsable;

use crate::program::{ProgramContext, Wasm};

use super::{Action, Assignment, Expression, ForBlock, Identifier, IfBlock, Operand, FullType, VarDeclaration, WhileBlock};

#[parsable]
pub enum Statement {
    #[parsable(suffix=";")]
    VarDeclaration(VarDeclaration),
    #[parsable(suffix=";")]
    Action(Action),
    If(IfBlock),
    While(WhileBlock),
    For(ForBlock),
    #[parsable(suffix=";")]
    Assignment(Assignment)
}

impl Statement {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        match self {
            Statement::VarDeclaration(var_declaration) => var_declaration.process(context),
            Statement::Action(_) => todo!(),
            Statement::If(_) => todo!(),
            Statement::While(_) => todo!(),
            Statement::For(_) => todo!(),
            Statement::Assignment(assignment) => assignment.process(context),
        }
    }
}