use parsable::parsable;
use colored::*;
use crate::{program::{ProgramContext, Type, Vasm}, vasm};
use super::BlockExpression;

#[parsable]
pub struct IterVariants {
    #[parsable(prefix="iter_variants")]
    pub block: BlockExpression
}

const BLOCK_NAME : &'static str = "iter_variants";

impl IterVariants {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        match context.get_current_type() {
            Some(type_wrapped) => {
                match context.iter_variants_counter {
                    Some(_) => {
                        context.errors.generic(self, format!("an `{}` cannot be nested inside another one ", BLOCK_NAME.bold()));
                    },
                    None => {
                        let mut block_vasm = vasm![];
                        let variant_count = type_wrapped.borrow().enum_variants.len();

                        for i in 0..variant_count {
                            context.iter_variants_counter = Some(i);

                            if let Some(vasm) = self.block.process(None, context) {
                                if !vasm.ty.is_void() {
                                    context.errors.generic(&self.block, format!("expected `{}`, got `{}`", Type::Void, &vasm.ty));
                                }

                                block_vasm.extend(vasm);

                            }
                        }

                        context.iter_variants_counter = None;

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