use parsable::parsable;
use crate::{program::{ProgramContext, ScopeKind, Type, VI, Vasm}, vasm, wat};
use super::{Branch, BlockExpression};

#[parsable]
pub struct IfBlock {
    #[parsable(prefix="if")]
    pub if_branch: Branch,
    #[parsable(prefix="else if", separator="else if", optional=true)]
    pub else_if_branches: Vec<Branch>,
    #[parsable(prefix="else")]
    pub else_branch: Option<BlockExpression>
}

impl IfBlock {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = Vasm::void();
        let mut required_branch_type = Type::Void;

        context.push_scope(ScopeKind::Branch);
        if let (Some(condition_vasm), Some(block_vasm)) = (self.if_branch.process_condition(context), self.if_branch.process_body(type_hint, context)) {
            required_branch_type = block_vasm.ty.clone();

            result.extend(VI::typed_block(vec![block_vasm.ty.clone()], vasm![
                condition_vasm,
                VI::jump_if(0, VI::raw(wat!["i32.eqz"])),
                block_vasm,
                VI::jump(1)
            ]));
        }
        context.pop_scope();

        for else_if_branch in &self.else_if_branches {
            context.push_scope(ScopeKind::Branch);

            if let (Some(condition_vasm), Some(block_vasm)) = (else_if_branch.process_condition(context), else_if_branch.process_body(Some(&required_branch_type), context)) {
                match block_vasm.ty.get_common_type(&required_branch_type) {
                    Some(ty) => required_branch_type = ty.clone(),
                    None => context.errors.add(&else_if_branch, format!("expected `{}`, got `{}`", &required_branch_type, &block_vasm.ty)),
                }

                result.extend(VI::typed_block(vec![block_vasm.ty.clone()], vasm![
                    condition_vasm,
                    VI::jump_if(0, VI::raw(wat!["i32.eqz"])),
                    block_vasm,
                    VI::jump(1)
                ]));
            }
            context.pop_scope();
        }

        if let Some(else_branch) = &self.else_branch {
            context.push_scope(ScopeKind::Branch);

            if let Some(block_vasm) = else_branch.process(Some(&required_branch_type), context) {
                match block_vasm.ty.get_common_type(&required_branch_type) {
                    Some(ty) => {},
                    None => context.errors.add(&else_branch, format!("expected `{}`, got `{}`", &required_branch_type, &block_vasm.ty)),
                }

                result.extend(VI::typed_block(vec![block_vasm.ty.clone()], vasm![
                    block_vasm,
                    VI::jump(1)
                ]));
            }

            context.pop_scope();
        } else if !required_branch_type.is_void() {
            context.errors.add(self, format!("missing `else` branch (because the `if` branch returns a non-void type)"));
        }

        Some(Vasm::new(required_branch_type.clone(), vec![], vec![VI::typed_block(vec![required_branch_type], result)]))
    }
}