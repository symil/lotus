use parsable::parsable;
use colored::*;
use crate::{program::{ProgramContext, Vasm}, vasm};
use super::StatementList;

#[parsable]
pub struct IterFields {
    #[parsable(prefix="iter_fields")]
    pub statements: StatementList
}

impl IterFields {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        match context.get_current_type() {
            Some(type_wrapped) => {
                match context.iter_fields_counter {
                    Some(_) => {
                        context.errors.add(self, format!("an `{}` cannot be nested inside another one ", "iter_fields".bold()));
                    },
                    None => {
                        let mut block_vasm = vasm![];
                        let field_count = type_wrapped.borrow().fields.len();

                        for i in 0..field_count {
                            context.iter_fields_counter = Some(i);

                            if let Some(vasm) = self.statements.process(context) {
                                block_vasm.extend(vasm);
                            }
                        }

                        context.iter_fields_counter = None;

                        result = Some(block_vasm);
                    },
                }
            },
            None => {
                context.errors.add(self, format!("`{}` can only be used from within a method", "iter_fields".bold()));
            },
        };

        result
    }
}