use parsable::parsable;
use crate::{program::{ProgramContext, ScopeKind, Type, VI, Vasm}, vasm, wat};
use super::Branch;

#[parsable]
pub struct WhileBlock {
    #[parsable(prefix="while")]
    pub while_branch: Branch
}

impl WhileBlock {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = Vasm::empty();
        let return_found = context.return_found;

        context.push_scope(ScopeKind::Loop);

        if let (Some(condition_vasm), Some(block_vasm)) = (self.while_branch.process_condition(context), self.while_branch.process_body(context)) {
            result.extend(VI::block(
                VI::loop_(vasm![
                    condition_vasm,
                    VI::jump_if(1, VI::raw(wat!["i32.eqz"])),
                    block_vasm,
                    VI::jump(0)
                ])
            ));
        }

        context.pop_scope();
        context.return_found = return_found;

        Some(result)
    }
}