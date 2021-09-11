use parsable::parsable;
use crate::{generation::{Wat, ToWat, ToWatVec}, program::{ProgramContext, ScopeKind, Type, Vasm}, wat};
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
        let mut ok = true;
        let mut return_found = context.return_found;
        let mut branches_return = vec![];

        context.return_found = false;

        let mut wat = wat!["block"];
        let mut variables = vec![];

        context.return_found = false;
        context.push_scope(ScopeKind::Branch);
        if let (Some(condition_wasm), Some(block_wasm)) = (self.if_branch.process_condition(context), self.if_branch.process_body(context)) {
            let branch_wat = wat!["block", condition_wasm.wat, wat!["br_if", 0, wat!["i32.eqz"]], block_wasm.wat, wat!["br", 1]];

            wat.push(branch_wat);
            variables.extend(block_wasm.variables);
        } else {
            ok = false;
        }
        context.pop_scope();
        branches_return.push(context.return_found);

        for branch in &self.else_if_branches {
            context.return_found = false;
            context.push_scope(ScopeKind::Branch);
            if let (Some(condition_wasm), Some(block_wasm)) = (branch.process_condition(context), branch.process_body(context)) {
                let branch_wat = wat!["block", condition_wasm.wat, wat!["br_if", 0, wat!["i32.eqz"]], block_wasm.wat, wat!["br", 1]];

                wat.push(branch_wat);
                variables.extend(block_wasm.variables);
            } else {
                ok = false;
            }
            context.pop_scope();
            branches_return.push(context.return_found);
        }

        context.return_found = false;
        if let Some(else_branch) = &self.else_branch {
            context.push_scope(ScopeKind::Branch);

            if let Some(wasm) = else_branch.process(context) {
                let branch_wat = wat!["block", wasm.wat, wat!["br", 1]];

                wat.push(branch_wat);
                variables.extend(wasm.variables);
            } else {
                ok = false;
            }

            context.pop_scope();
        }
        branches_return.push(context.return_found);

        context.return_found = return_found || branches_return.iter().all(|value| *value);

        match ok {
            true => Some(IrFragment::new(Type::Void, wat, variables)),
            false => None
        }
    }
}