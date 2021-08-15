use parsable::parsable;

use crate::program::{ProgramContext, VariableKind, Wasm};

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
            Statement::VarDeclaration(var_declaration) => var_declaration.process(VariableKind::Local, context),
            Statement::Action(action) => action.process(context),
            Statement::If(if_block) => if_block.process(context),
            Statement::While(while_block) => while_block.process(context),
            Statement::For(for_block) => for_block.process(context),
            Statement::Assignment(assignment) => assignment.process(context),
        }
    }
}