use parsable::parsable;
use crate::{program::{ProgramContext, ScopeKind, Type, VI, Vasm}, vasm, wat};
use super::{Branch, StatementList};

#[parsable]
pub struct IfBlock {
    #[parsable(prefix="if")]
    pub if_branch: Branch,
    #[parsable(prefix="else if", separator="else if", optional=true)]
    pub else_if_branches: Vec<Branch>,
    #[parsable(prefix="else")]
    pub else_branch: Option<StatementList>
}

impl IfBlock {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut return_found = context.return_found;
        let mut branches_return = vec![];

        context.return_found = false;

        let mut result = Vasm::empty();

        context.return_found = false;
        context.push_scope(ScopeKind::Branch);
        if let (Some(condition_vasm), Some(block_vasm)) = (self.if_branch.process_condition(context), self.if_branch.process_body(context)) {
            result.extend(VI::block(vasm![
                condition_vasm,
                VI::jump_if(0, VI::raw(wat!["i32.eqz"])),
                block_vasm,
                VI::jump(1)
            ]));
        }
        context.pop_scope();
        branches_return.push(context.return_found);

        for branch in &self.else_if_branches {
            context.return_found = false;
            context.push_scope(ScopeKind::Branch);

            if let (Some(condition_vasm), Some(block_vasm)) = (branch.process_condition(context), branch.process_body(context)) {
                result.extend(VI::block(vasm![
                    condition_vasm,
                    VI::jump_if(0, VI::raw(wat!["i32.eqz"])),
                    block_vasm,
                    VI::jump(1)
                ]));
            }
            context.pop_scope();
            branches_return.push(context.return_found);
        }

        context.return_found = false;
        if let Some(else_branch) = &self.else_branch {
            context.push_scope(ScopeKind::Branch);

            if let Some(vasm) = else_branch.process(context) {
                result.extend(VI::block(vasm![
                    vasm,
                    VI::jump(1)
                ]));
            }

            context.pop_scope();
        }
        branches_return.push(context.return_found);

        context.return_found = return_found || branches_return.iter().all(|value| *value);

        Some(vasm![VI::block(result)])
    }
}