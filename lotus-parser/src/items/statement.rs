use parsable::parsable;
use crate::program::{ProgramContext, VariableKind, Vasm};
use super::{Action, Assignment, Expression, ForBlock, FullType, Identifier, IfBlock, IterFields, Operand, VarDeclaration, WhileBlock};

#[parsable]
pub enum Statement {
    #[parsable(suffix=";")]
    VarDeclaration(VarDeclaration),
    #[parsable(suffix=";")]
    Action(Action),
    IterFields(IterFields),
    If(IfBlock),
    While(WhileBlock),
    For(ForBlock),
    #[parsable(suffix=";")]
    Assignment(Assignment)
}

impl Statement {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        match self {
            Statement::VarDeclaration(var_declaration) => match var_declaration.process(VariableKind::Local, context) {
                Some((_, vasm)) => Some(vasm),
                None => None,
            },
            Statement::Action(action) => action.process(context),
            Statement::IterFields(iter_fields) => iter_fields.process(context),
            Statement::If(if_block) => if_block.process(context),
            Statement::While(while_block) => while_block.process(context),
            Statement::For(for_block) => for_block.process(context),
            Statement::Assignment(assignment) => assignment.process(context),
        }
    }
}