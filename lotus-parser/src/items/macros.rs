use std::{borrow::Borrow, cell::Ref, rc::Rc};

use parsable::{DataLocation, parsable};
use colored::*;
use crate::{items::make_string_value_from_literal, program::{BuiltinType, EnumVariantInfo, FieldInfo, ProgramContext, Type, TypeBlueprint, VI, VariableInfo, Vasm}, utils::Link};
use super::{Identifier};

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
    fn check_macro_context(&self, context: &mut ProgramContext) -> Option<(MacroName, Link<TypeBlueprint>, Option<Rc<FieldInfo>>, Option<Rc<EnumVariantInfo>>, Option<Type>)> {
        let macro_name = match self.to_enum() {
            Some(m) => match context.get_current_type() {
                Some(type_wrapped) => type_wrapped.with_ref(|type_unwrapped| {
                    match m {
                        MacroName::TypeId | MacroName::TypeName | MacroName::TypeShortName | MacroName::FieldCount | MacroName::VariantCount => Some(m),
                        MacroName::FieldName | MacroName::FieldType | MacroName::FieldDefaultExpression => match context.iter_fields_counter {
                            Some(_) => Some(m),
                            None => context.errors.add_and_none(self, format!("macro `{}` can only be accessed from inside an `iter_fields` block", format!("#{}", &self.name).bold())),
                        },
                        MacroName::VariantName | MacroName::VariantValue => match context.iter_variants_counter {
                            Some(_) => Some(m),
                            None => context.errors.add_and_none(self, format!("macro `{}` can only be accessed from inside an `iter_variants` block", format!("#{}", &self.name).bold()))
                        },
                        MacroName::AncestorId | MacroName::AncestorName => match context.iter_ancestors_counter {
                            Some(_) => Some(m),
                            None => context.errors.add_and_none(self, format!("macro `{}` can only be accessed from inside an `iter_ancestors` block", format!("#{}", &self.name).bold())),
                        },
                    }
                }),
                None => context.errors.add_and_none(self, format!("macro `{}` can only be accessed from inside a method", format!("#{}", &self.name).bold())),
            },
            None => context.errors.add_and_none(self, format!("macro `{}` does not exist", format!("#{}", &self.name).bold())),
        };

        match macro_name {
            Some(m) => {
                let (field_info, variant_info, ancestor_info) = self.get_context_info(context);

                Some((m, context.get_current_type().unwrap(), field_info, variant_info, ancestor_info))
            },
            None => None,
        }
    }

    fn get_context_info(&self, context: &ProgramContext) -> (Option<Rc<FieldInfo>>, Option<Rc<EnumVariantInfo>>, Option<Type>) {
        context.get_current_type().unwrap().with_ref(|type_unwrapped| {
            let field_info = context.iter_fields_counter.and_then(|index| Some(type_unwrapped.fields.get_index(index).unwrap().1.clone()));
            let variant_info = context.iter_variants_counter.and_then(|index| Some(type_unwrapped.enum_variants.get_index(index).unwrap().1.clone()));
            let ancestor_info = context.iter_ancestors_counter.and_then(|index| Some(type_unwrapped.ancestors[index].clone()));

            (field_info, variant_info, ancestor_info)
        })
    }

    pub fn process_as_value(&self, context: &mut ProgramContext) -> Option<Vasm> {
        match self.check_macro_context(context) {
            Some((m, type_wrapped, field_info, variant_info, ancestor)) => m.process_as_value(self, type_wrapped.borrow(), field_info, variant_info, ancestor, context),
            None => context.errors.add_and_none(self, format!("macro `{}` cannot be processed as a value", format!("#{}", &self.name).bold())),
        }
    }

    pub fn process_as_type(&self, context: &mut ProgramContext) -> Option<Type> {
        match self.check_macro_context(context) {
            Some((m, type_wrapped, field_info, variant_info, ancestor)) => m.process_as_type(self, type_wrapped.borrow(), field_info, variant_info, ancestor, context),
            None => context.errors.add_and_none(self, format!("macro `{}` cannot be processed as a type", format!("#{}", &self.name).bold())),
        }
    }

    pub fn process_as_name(&self, context: &mut ProgramContext) -> Option<Identifier> {
        match self.check_macro_context(context) {
            Some((m, type_wrapped, field_info, variant_info, ancestor)) => m.process_as_name(self, type_wrapped.borrow(), field_info, variant_info, ancestor, context),
            None => context.errors.add_and_none(self, format!("macro `{}` cannot be processed as a name", format!("#{}", &self.name).bold())),
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

impl MacroName {
    fn process_as_value(&self, location: &DataLocation, type_unwrapped: Ref<TypeBlueprint>, field_info: Option<Rc<FieldInfo>>, variant_info: Option<Rc<EnumVariantInfo>>, ancestor: Option<Type>, context: &mut ProgramContext) -> Option<Vasm> {
        match self {
            MacroName::TypeId => Some(Vasm::new(context.int_type(), vec![], vec![VI::type_id(&type_unwrapped.self_type)])),
            MacroName::TypeName => Some(Vasm::new(context.get_builtin_type(BuiltinType::String, vec![]), vec![], vec![VI::type_name(&type_unwrapped.self_type)])),
            MacroName::TypeShortName => Some(make_string_value_from_literal(None, type_unwrapped.name.as_str(), context).unwrap()),
            MacroName::FieldCount => Some(Vasm::new(context.int_type(), vec![], vec![VI::int(type_unwrapped.fields.len())])),
            MacroName::FieldName => Some(make_string_value_from_literal(None, field_info.unwrap().name.as_str(), context).unwrap()),
            MacroName::FieldType => None,
            MacroName::FieldDefaultExpression => Some(field_info.unwrap().default_value.replace_type_parameters(&type_unwrapped.self_type, location.get_hash())),
            MacroName::VariantName => Some(make_string_value_from_literal(None, variant_info.unwrap().name.as_str(), context).unwrap()),
            MacroName::VariantValue => Some(Vasm::new(context.int_type(), vec![], vec![VI::int(variant_info.unwrap().value)])),
            MacroName::VariantCount => Some(Vasm::new(context.int_type(), vec![], vec![VI::int(type_unwrapped.enum_variants.len())])),
            MacroName::AncestorId => Some(Vasm::new(context.int_type(), vec![], vec![VI::type_id(&ancestor.unwrap())])),
            MacroName::AncestorName => Some(make_string_value_from_literal(None, &ancestor.unwrap().get_name(), context).unwrap()),
        }
    }

    fn process_as_type(&self, location: &DataLocation, type_unwrapped: Ref<TypeBlueprint>, field_info: Option<Rc<FieldInfo>>, variant_info: Option<Rc<EnumVariantInfo>>, ancestor: Option<Type>, context: &mut ProgramContext) -> Option<Type> {
        match self {
            MacroName::TypeId => None,
            MacroName::TypeName => None,
            MacroName::TypeShortName => None,
            MacroName::FieldCount => None,
            MacroName::FieldName => None,
            MacroName::FieldType => Some(field_info.unwrap().ty.clone()),
            MacroName::FieldDefaultExpression => None,
            MacroName::VariantName => None,
            MacroName::VariantValue => None,
            MacroName::VariantCount => None,
            MacroName::AncestorId => None,
            MacroName::AncestorName => None,
        }
    }

    fn process_as_name(&self, location: &DataLocation, type_unwrapped: Ref<TypeBlueprint>, field_info: Option<Rc<FieldInfo>>, variant_info: Option<Rc<EnumVariantInfo>>, ancestor: Option<Type>, context: &mut ProgramContext) -> Option<Identifier> {
        match self {
            MacroName::TypeId => None,
            MacroName::TypeName => None,
            MacroName::TypeShortName => None,
            MacroName::FieldCount => None,
            MacroName::FieldName => Some(Identifier::new(field_info.unwrap().name.as_str(), &location)),
            MacroName::FieldType => None,
            MacroName::FieldDefaultExpression => None,
            MacroName::VariantName => None,
            MacroName::VariantValue => None,
            MacroName::VariantCount => None,
            MacroName::AncestorId => None,
            MacroName::AncestorName => None,
        }
    }
}