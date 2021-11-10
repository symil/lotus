use parsable::parsable;
use colored::*;
use crate::{program::{ProgramContext, Vasm}, vasm};
use super::StatementList;

#[parsable]
pub struct IterVariants {
    #[parsable(prefix="iter_variants")]
    pub statements: StatementList
}

const BLOCK_NAME : &'static str = "iter_variants";

impl IterVariants {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        match context.get_current_type() {
            Some(type_wrapped) => {
                match context.iter_variants_counter {
                    Some(_) => {
                        context.errors.add(self, format!("an `{}` cannot be nested inside another one ", BLOCK_NAME.bold()));
                    },
                    None => {
                        let mut block_vasm = vasm![];
                        let variant_count = type_wrapped.borrow().enum_variants.len();

                        for i in 0..variant_count {
                            context.iter_variants_counter = Some(i);

                            if let Some(vasm) = self.statements.process(context) {
                                block_vasm.extend(vasm);
                            }
                        }

                        context.iter_variants_counter = None;

                        result = Some(block_vasm);
                    },
                }
            },
            None => {
                context.errors.add(self, format!("`{}` can only be used from inside a method", BLOCK_NAME.bold()));
            },
        };

        result
    }
}