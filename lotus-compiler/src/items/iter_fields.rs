use parsable::parsable;
use colored::*;
use crate::{program::{ProgramContext, Type, Vasm}};
use super::BlockExpression;

#[parsable]
pub struct IterFields {
    #[parsable(prefix="iter_fields")]
    pub block: BlockExpression
}

impl IterFields {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        match context.get_current_type() {
            Some(type_wrapped) => {
                match context.iter_fields_counter {
                    Some(_) => {
                        context.errors.generic(self, format!("an `{}` cannot be nested inside another one ", "iter_fields".bold()));
                    },
                    None => {
                        let mut block_vasm = context.vasm().set_void(context);
                        let field_count = type_wrapped.borrow().fields.len();

                        for i in 0..field_count {
                            context.iter_fields_counter = Some(i);

                            if let Some(vasm) = self.block.process(None, context) {
                                if !vasm.ty.is_void() {
                                    context.errors.type_mismatch(&self.block, &context.void_type(), &vasm.ty);
                                }
                                
                                block_vasm = block_vasm.append(vasm);
                            }
                        }

                        context.iter_fields_counter = None;

                        result = Some(block_vasm);
                    },
                }
            },
            None => {
                context.errors.generic(self, format!("`{}` can only be used from inside a method", "iter_fields".bold()));
            },
        };

        result
    }
}