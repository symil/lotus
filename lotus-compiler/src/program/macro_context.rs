use std::{rc::Rc, cell::Ref, fmt::Display};
use colored::Colorize;
use parsable::DataLocation;

use crate::{utils::Link, items::Identifier};
use super::{TypeBlueprint, FunctionBlueprint, FieldInfo, EnumVariantInfo, Type, ProgramContext};

pub struct MacroContext<'a, 'b> {
    location: &'a DataLocation,
    context: &'b mut ProgramContext,
    current_type: Option<Link<TypeBlueprint>>,
    current_function: Option<Link<FunctionBlueprint>>,
    field_info: Option<Rc<FieldInfo>>,
    variant_info: Option<Rc<EnumVariantInfo>>,
    ancestor_type: Option<Type>
}

impl<'a, 'b> MacroContext<'a, 'b> {
    pub fn new(location: &'a DataLocation, context: &'b mut ProgramContext) -> Self {
        let mut current_type = None;
        let mut current_function = None;
        let mut field_info = None;
        let mut variant_info = None;
        let mut ancestor_type = None;

        if let Some(type_wrapped) = context.get_current_type() {
            type_wrapped.with_ref(|type_unwrapped| {
                field_info = context.iter_fields_counter.map(|index| type_unwrapped.fields.get_index(index).unwrap().1.clone());
                variant_info = context.iter_variants_counter.map(|index| type_unwrapped.enum_variants.get_index(index).unwrap().1.clone());
                ancestor_type = context.iter_ancestors_counter.map(|index| type_unwrapped.ancestors[index].clone());
            });

            current_type = Some(type_wrapped);
        }

        if let Some(function_wrapped) = context.get_current_function() {
            current_function = Some(function_wrapped);
        }

        Self {
            location,
            context,
            current_type,
            current_function,
            field_info,
            variant_info,
            ancestor_type,
        }
    }

    pub fn access_current_type<T, F : Fn(Ref<TypeBlueprint>, &ProgramContext) -> T>(&mut self, callback: F) -> Option<T> {
        match &self.current_type {
            Some(type_wrapped) => Some(type_wrapped.with_ref(|ty| callback(ty, &self.context))),
            None => self.context.errors.add_generic_and_none(&self.location, format!("macro `{}` can only be accessed from inside type a declaration", self)),
        }
    }

    pub fn access_current_function<T, F : Fn(Ref<FunctionBlueprint>, &ProgramContext) -> T>(&mut self, callback: F) -> Option<T> {
        match &self.current_function {
            Some(function_wrapped) => Some(function_wrapped.with_ref(|function| callback(function, &self.context))),
            None => self.context.errors.add_generic_and_none(&self.location, format!("macro `{}` can only be accessed from inside a function", self)),
        }
    }

    pub fn access_current_field<T, F : Fn(&FieldInfo, &ProgramContext) -> T>(&mut self, callback: F) -> Option<T> {
        match &self.field_info {
            Some(field_info) => Some(callback(field_info, &self.context)),
            None => self.context.errors.add_generic_and_none(&self.location, format!("macro `{}` can only be accessed from inside an `iter_fields` block", self)),
        }
    }

    pub fn access_current_variant<T, F : Fn(&EnumVariantInfo, &ProgramContext) -> T>(&mut self, callback: F) -> Option<T> {
        match &self.variant_info {
            Some(variant_info) => Some(callback(variant_info, &self.context)),
            None => self.context.errors.add_generic_and_none(&self.location, format!("macro `{}` can only be accessed from inside an `iter_variants` block", self)),
        }
    }

    pub fn access_current_ancestor<T, F : Fn(&Type, &ProgramContext) -> T>(&mut self, callback: F) -> Option<T> {
        match &self.ancestor_type {
            Some(ancestor_type) => Some(callback(ancestor_type, &self.context)),
            None => self.context.errors.add_generic_and_none(&self.location, format!("macro `{}` can only be accessed from inside an `iter_ancestors` block", self)),
        }
    }
}

impl<'a, 'b> Display for MacroContext<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("#{}", self.location.as_str()).bold())
    }
}