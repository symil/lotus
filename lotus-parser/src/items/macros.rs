use parsable::parsable;
use colored::*;
use crate::{items::make_string_value_from_literal, program::{BuiltinType, ProgramContext, Type, VI, Vasm}};
use super::{Identifier, ValueOrType};

#[parsable]
pub struct Macro {
    #[parsable(prefix="#")]
    pub name: Identifier,
}

enum MacroName {
    TypeId,
    TypeName,
    TypeShortName,
    FieldCount,
    FieldName,
    FieldType,
    FieldDefaultExpression,
    VariantName,
    VariantValue,
    VariantCount,
    AncestorId,
    AncestorName
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
                            MacroName::TypeName => Some(ValueOrType::Value(Vasm::new(context.get_builtin_type(BuiltinType::String, vec![]), vec![], vec![VI::type_name(&type_unwrapped.self_type)]))),
                            MacroName::TypeShortName => Some(ValueOrType::Value(make_string_value_from_literal(None, type_unwrapped.name.as_str(), context).unwrap())),
                            MacroName::FieldCount => Some(ValueOrType::Value(Vasm::new(context.int_type(), vec![], vec![VI::int(type_unwrapped.fields.len())]))),
                            MacroName::VariantCount => Some(ValueOrType::Value(Vasm::new(context.int_type(), vec![], vec![VI::int(type_unwrapped.enum_variants.len())]))),
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
                            },
                            MacroName::VariantName | MacroName::VariantValue => match context.iter_variants_counter {
                                Some(variant_index) => {
                                    let variant = &type_unwrapped.enum_variants.get_index(variant_index).unwrap().1;

                                    match m {
                                        MacroName::VariantValue => Some(ValueOrType::Value(Vasm::new(context.int_type(), vec![], vec![VI::int(variant.value)]))),
                                        MacroName::VariantName => Some(ValueOrType::Value(make_string_value_from_literal(None, variant.name.as_str(), context).unwrap())),
                                        _ => unreachable!()
                                    }
                                },
                                None => {
                                    context.errors.add(self, format!("macro `{}` can only be accessed from inside an `iter_variants` block", format!("#{}", &self.name).bold()));
                                    None
                                },
                            },
                            MacroName::AncestorId | MacroName::AncestorName => match context.iter_ancestors_counter {
                                Some(ancestor_index) => {
                                    let ancestor = &type_unwrapped.ancestors[ancestor_index];

                                    match m {
                                        MacroName::AncestorId => Some(ValueOrType::Value(Vasm::new(context.int_type(), vec![], vec![VI::type_id(ancestor)]))),
                                        MacroName::AncestorName => Some(ValueOrType::Value(make_string_value_from_literal(None, &ancestor.get_name(), context).unwrap())),
                                        _ => unreachable!()
                                    }
                                },
                                None => {
                                    context.errors.add(self, format!("macro `{}` can only be accessed from inside an `iter_ancestors` block", format!("#{}", &self.name).bold()));
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
                MacroName::TypeId | MacroName::TypeName | MacroName::TypeShortName |
                MacroName::FieldCount | MacroName::FieldType | MacroName::FieldDefaultExpression |
                MacroName::VariantCount | MacroName::VariantValue | MacroName::VariantName |
                MacroName::AncestorId | MacroName::AncestorName => {
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
            "TYPE_ID" => Some(MacroName::TypeId),
            "TYPE_NAME" => Some(MacroName::TypeName),
            "TYPE_SHORT_NAME" => Some(MacroName::TypeShortName),
            "FIELD_COUNT" => Some(MacroName::FieldCount),
            "FIELD_TYPE" => Some(MacroName::FieldType),
            "FIELD_NAME" => Some(MacroName::FieldName),
            "FIELD_DEFAULT_EXPRESSION" => Some(MacroName::FieldDefaultExpression),
            "VARIANT_NAME" => Some(MacroName::VariantName),
            "VARIANT_VALUE" => Some(MacroName::VariantValue),
            "VARIANT_COUNT" => Some(MacroName::VariantCount),
            "ANCESTOR_ID" => Some(MacroName::AncestorId),
            "ANCESTOR_NAME" => Some(MacroName::AncestorName),
            _ => None
        }
    }
}