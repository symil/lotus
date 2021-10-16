use parsable::parsable;
use colored::*;
use crate::program::{ProgramContext, VI, Vasm, MACRO_TYPE_ID, MACRO_FIELD_COUNT, MACRO_FIELD_NAME, MACRO_FIELD_TYPE};
use super::{Identifier, ValueOrType};

#[parsable]
pub struct Macro {
    #[parsable(prefix="#")]
    pub name: Identifier,
}

impl Macro {
    pub fn process_as_value(&self, context: &mut ProgramContext) -> Option<ValueOrType> {
        match context.get_current_type() {
            Some(type_wrapped) => {
                type_wrapped.with_ref(|type_unwrapped| {
                    match self.name.as_str() {
                        MACRO_TYPE_ID => Some(ValueOrType::Value(Vasm::new(context.int_type(), vec![], vec![VI::type_id(&type_unwrapped.self_type)]))),
                        MACRO_FIELD_COUNT => Some(ValueOrType::Value(Vasm::new(context.int_type(), vec![], vec![VI::int(type_unwrapped.fields.len())]))),
                        MACRO_FIELD_NAME => {
                            context.errors.add(self, format!("macro `{}` cannot be processed as a value", format!("#{}", &self.name).bold()));
                            None
                        },
                        MACRO_FIELD_TYPE => match context.iter_fields_counter {
                            Some(field_index) => {
                                Some(ValueOrType::Type(type_unwrapped.fields.get_index(field_index).unwrap().1.ty.clone()))
                            },
                            None => {
                                context.errors.add(self, format!("macro `{}` can only be accessed from inside an `iter_fields` block", format!("#{}", &self.name).bold()));
                                None
                            },
                        },
                        _ => {
                            context.errors.add(self, format!("macro `{}` does not exist", format!("#{}", &self.name).bold()));
                            None
                        }
                    }
                })
            },
            None => {
                context.errors.add(self, format!("macro `{}` can only be accessed from inside a method", format!("#{}", &self.name).bold()));
                None
            },
        }
    }

    pub fn process_as_name(&self, context: &mut ProgramContext) -> Option<Identifier> {
        match self.name.as_str() {
            MACRO_FIELD_NAME => {
                match context.get_current_type() {
                    Some(type_wrapped) => {
                        type_wrapped.with_ref(|type_unwrapped| {
                            match context.iter_fields_counter {
                                Some(field_index) => {
                                    let field = &type_unwrapped.fields.get_index(field_index).unwrap().1;

                                    Some(Identifier::new(field.name.as_str(), self))
                                },
                                None => {
                                    context.errors.add(self, format!("macro `{}` can only be accessed from inside an `iter_fields` block", format!("#{}", &self.name).bold()));
                                    None
                                },
                            }
                        })
                    },
                    None => {
                        context.errors.add(self, format!("macro `{}` can only be accessed from inside a method", format!("#{}", &self.name).bold()));
                        None
                    },
                }
            },
            MACRO_TYPE_ID | MACRO_FIELD_COUNT | MACRO_FIELD_TYPE =>  {
                context.errors.add(self, format!("macro `{}` cannot be processed as a name", format!("#{}", &self.name).bold()));
                None
            },
            _ => {
                context.errors.add(self, format!("macro `{}` does not exist", format!("#{}", &self.name).bold()));
                None
            }
        }
    }
}