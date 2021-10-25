use parsable::parsable;
use colored::*;
use crate::{items::make_string_value_from_literal, program::{MACRO_FIELD_COUNT, MACRO_FIELD_DEFAULT_EXPRESSION, MACRO_FIELD_NAME, MACRO_FIELD_TYPE, MACRO_TYPE_ID, MACRO_TYPE_NAME, ProgramContext, Type, VI, Vasm}};
use super::{Identifier, ValueOrType};

#[parsable]
pub struct Macro {
    #[parsable(prefix="#")]
    pub name: Identifier,
}

enum MacroName {
    TypeId,
    TypeName,
    FieldCount,
    FieldName,
    FieldType,
    FieldDefaultExpression
}

enum ConstantName {
    MemoryCellByteSize,
    WasmPageByteSize,
    VirtualPageByteSize,
    VirtualPageMetadataSize,
    PointerMetadataSize,
    MaxVirtualCountPerBlockSize
}

impl Macro {
    pub fn process_as_value(&self, context: &mut ProgramContext) -> Option<ValueOrType> {
        match self.to_enum() {
            Some(m) => match context.get_current_type() {
                Some(type_wrapped) => {
                    type_wrapped.with_ref(|type_unwrapped| {
                        match m {
                            MacroName::TypeId => Some(ValueOrType::Value(Vasm::new(context.int_type(), vec![], vec![VI::type_id(&type_unwrapped.self_type)]))),
                            MacroName::TypeName => Some(ValueOrType::Value(make_string_value_from_literal(None, type_unwrapped.name.as_str(), context).unwrap())),
                            MacroName::FieldCount => Some(ValueOrType::Value(Vasm::new(context.int_type(), vec![], vec![VI::int(type_unwrapped.fields.len())]))),
                            MacroName::FieldName | MacroName::FieldType | MacroName::FieldDefaultExpression => match context.iter_fields_counter {
                                Some(field_index) => {
                                    let field_info = type_unwrapped.fields.get_index(field_index).unwrap().1;

                                    match m {
                                        MacroName::FieldName => Some(ValueOrType::Value(make_string_value_from_literal(None, field_info.name.as_str(), context).unwrap())),
                                        MacroName::FieldType => Some(ValueOrType::Type(field_info.ty.clone())),
                                        MacroName::FieldDefaultExpression => Some(ValueOrType::Value(field_info.default_value.replace_type_parameters(&type_unwrapped.self_type, self.location.get_hash()))),
                                        _ => unreachable!()
                                    }
                                },
                                None => {
                                    context.errors.add(self, format!("macro `{}` can only be accessed from inside an `iter_fields` block", format!("#{}", &self.name).bold()));
                                    None
                                },
                            }
                        }
                    })
                },
                None => {
                    context.errors.add(self, format!("macro `{}` can only be accessed from inside a method", format!("#{}", &self.name).bold()));
                    None
                },
            },
            None => {
                context.errors.add(self, format!("macro `{}` does not exist", format!("#{}", &self.name).bold()));
                None
            }
        }
    }

    pub fn process_as_name(&self, context: &mut ProgramContext) -> Option<Identifier> {
        match self.to_enum() {
            Some(m) => match m {
                MacroName::FieldName => {
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
                MacroName::TypeId | MacroName::TypeName | MacroName::FieldCount | MacroName::FieldType | MacroName::FieldDefaultExpression =>  {
                    context.errors.add(self, format!("macro `{}` cannot be processed as a name", format!("#{}", &self.name).bold()));
                    None
                },
            },
            None => {
                context.errors.add(self, format!("macro `{}` does not exist", format!("#{}", &self.name).bold()));
                None
            }
        }
    }

    fn to_enum(&self) -> Option<MacroName> {
        match self.name.as_str() {
            MACRO_TYPE_ID => Some(MacroName::TypeId),
            MACRO_TYPE_NAME => Some(MacroName::TypeName),
            MACRO_FIELD_COUNT => Some(MacroName::FieldCount),
            MACRO_FIELD_TYPE => Some(MacroName::FieldType),
            MACRO_FIELD_NAME => Some(MacroName::FieldName),
            MACRO_FIELD_DEFAULT_EXPRESSION => Some(MacroName::FieldDefaultExpression),
            _ => None
        }
    }
}