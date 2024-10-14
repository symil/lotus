use parsable::parsable;
use crate::{items::Identifier, program::{ProgramContext, ScopeKind, Type, VariableInfo, VariableKind, Vasm}, wat};
use super::{ParsedBranch, ParsedBlockExpression};

#[parsable]
pub struct ParsedIfBlock {
    #[parsable(prefix="if")]
    pub if_branch: ParsedBranch,
    #[parsable(prefix="else if", separator="else if", optional=true)]
    pub else_if_branches: Vec<ParsedBranch>,
    #[parsable(prefix="else")]
    pub else_branch: Option<ParsedBlockExpression>
}

impl ParsedIfBlock {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = context.vasm().set_void(context);
        let mut required_branch_type = context.void_type();
        let result_var = VariableInfo::tmp("tmp_result", context.void_type());

        context.push_scope(ScopeKind::Branch);
        if let (Some(condition_vasm), Some(block_vasm)) = (self.if_branch.process_condition(context), self.if_branch.process_body(type_hint, context)) {
            required_branch_type = block_vasm.ty.clone();

            result = result.block(context.vasm()
                .append(condition_vasm)
                .jump_if(0, context.vasm().eqz())
                .append(block_vasm)
                .set_tmp_var(&result_var)
                .jump(1)
            );
        }
        context.pop_scope();

        for else_if_branch in &self.else_if_branches {
            context.push_scope(ScopeKind::Branch);

            if let (Some(condition_vasm), Some(block_vasm)) = (else_if_branch.process_condition(context), else_if_branch.process_body(Some(&required_branch_type), context)) {
                match block_vasm.ty.get_common_type(&required_branch_type) {
                    Some(ty) => required_branch_type = ty.clone(),
                    None => context.errors.type_mismatch(&else_if_branch, &required_branch_type, &block_vasm.ty).void(),
                }

                result = result.block(context.vasm()
                    .append(condition_vasm)
                    .jump_if(0, context.vasm().eqz())
                    .append(block_vasm)
                    .set_tmp_var(&result_var)
                    .jump(1)
                );
            }
            context.pop_scope();
        }

        if let Some(else_branch) = &self.else_branch {
            context.push_scope(ScopeKind::Branch);

            if let Some(block_vasm) = else_branch.process(Some(&required_branch_type), context) {
                match block_vasm.ty.get_common_type(&required_branch_type) {
                    Some(ty) => {},
                    None => context.errors.type_mismatch(&else_branch, &required_branch_type, &block_vasm.ty).void(),
                }

                result = result.block(context.vasm()
                    .append(block_vasm)
                    .set_tmp_var(&result_var)
                    .jump(1)
                );
            }

            context.pop_scope();
        } else if !required_branch_type.is_void() {
            result = result.set_void(context);
            required_branch_type = context.void_type();
            // context.errors.generic(self, format!("missing `else` branch (because the `if` branch returns a non-void type)"));
        }

        result_var.set_type(required_branch_type.clone());

        Some(context.vasm()
            .declare_variable(&result_var)
            .block(result)
            .get_tmp_var(&result_var)
            .set_type(required_branch_type)
        )
    }
}