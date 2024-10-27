use parsable::{create_token_struct, parsable};
use crate::{program::{ProgramContext, ScopeKind, Type, Vasm, WHILE_KEYWORD}, wat};
use super::ParsedBranch;

create_token_struct!(WhileKeyword, WHILE_KEYWORD);

#[parsable]
pub struct ParsedWhileBlock {
    pub while_keyword: WhileKeyword,
    pub while_branch: ParsedBranch
}

impl ParsedWhileBlock {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = context.vasm().set_void(context);
        let void_type = context.void_type();

        context.push_scope(ScopeKind::Loop);

        if let (Some(condition_vasm), Some(block_vasm)) = (self.while_branch.process_condition(context), self.while_branch.process_body(Some(&void_type), context)) {
            if !block_vasm.ty.is_void() {
                context.errors.generic(&self, format!("expected `{}`, got `{}`", context.void_type(), &block_vasm.ty));
            }

            result = result.block(context.vasm()
                .loop_(context.vasm()
                    .append(condition_vasm)
                    .jump_if(1, context.vasm().eqz())
                    .append(block_vasm)
                    .jump(0)
                )
            );
        }

        context.pop_scope();

        Some(result)
    }
}