use parsable::parsable;
use crate::{generation::{Wat, ToWat, ToWatVec}, program::{ProgramContext, ScopeKind, Type}, wat};
use super::Branch;

#[parsable]
pub struct WhileBlock {
    #[parsable(prefix="while")]
    pub while_branch: Branch
}

impl WhileBlock {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;
        let return_found = context.return_found;

        context.push_scope(ScopeKind::Loop);

        if let (Some(condition_wasm), Some(block_wasm)) = (self.while_branch.process_condition(context), self.while_branch.process_body(context)) {
            let content = wat!["block",
                wat!["loop",
                    condition_wasm.wat,
                    wat!["br_if", 1, wat!["i32.eqz"]],
                    block_wasm.wat,
                    wat!["br", 0]
                ]
            ];

            result = Some(IrFragment::new(Type::Void, content, block_wasm.variables));
        }

        context.pop_scope();
        context.return_found = return_found;

        result
    }
}