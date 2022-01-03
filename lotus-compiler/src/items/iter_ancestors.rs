use parsable::parsable;
use colored::*;
use crate::{program::{ProgramContext, Type, Vasm}};
use super::BlockExpression;

#[parsable]
pub struct IterAncestors {
    #[parsable(prefix="iter_ancestors")]
    pub block: BlockExpression
}

const BLOCK_NAME : &'static str = "iter_ancestors";

impl IterAncestors {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        match context.get_current_type() {
            Some(type_wrapped) => {
                match context.iter_ancestors_counter {
                    Some(_) => {
                        context.errors.generic(self, format!("an `{}` cannot be nested inside another one ", BLOCK_NAME.bold()));
                    },
                    None => {
                        let mut block_vasm = context.vasm().set_void(context);
                        let ancestor_count = type_wrapped.borrow().ancestors.len();

                        for i in 0..ancestor_count {
                            context.iter_ancestors_counter = Some(i);

                            if let Some(vasm) = self.block.process(None, context) {
                                if !vasm.ty.is_void() {
                                    context.errors.type_mismatch(&self.block, &context.void_type(), &vasm.ty);
                                }
                                
                                block_vasm = block_vasm.append(vasm);
                            }
                        }

                        context.iter_ancestors_counter = None;

                        result = Some(block_vasm);
                    },
                }
            },
            None => {
                context.errors.generic(self, format!("`{}` can only be used from inside a method", BLOCK_NAME.bold()));
            },
        };

        result
    }
}