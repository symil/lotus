use parsable::DataLocation;
use crate::{program::{Type, FieldKind, VariableInfo, FunctionBlueprint, TypeBlueprint, TypedefBlueprint, GlobalVarBlueprint, BuiltinType, SELF_TYPE_NAME, InterfaceBlueprint}, utils::Link};
use super::{CompletionItem, CompletionItemKind, CompletionItemList, CompletionContent};

#[derive(Debug)]
pub struct CompletionArea {
    pub location: DataLocation,
    pub content: CompletionContent
}

impl CompletionArea {
    pub fn contains_cursor(&self, cursor_index: usize) -> bool {
        self.location.start <= cursor_index && self.location.end >= cursor_index
    }

    pub fn provide_completion_items(&self) -> Vec<CompletionItem> {
        let mut items = CompletionItemList::new();

        match &self.content {
            CompletionContent::FieldOrMethod(details) => {
                for field_info in details.parent_type.get_all_fields() {
                    items.add_field(field_info);
                }

                for method_info in details.parent_type.get_all_methods(FieldKind::Regular) {
                    items.add_method(method_info, details.insert_arguments);
                }
            },
            CompletionContent::StaticField(details) => {
                for variant in details.parent_type.get_all_variants() {
                    items.add_enum_variant(variant);
                }

                for method_info in details.parent_type.get_all_methods(FieldKind::Static) {
                    items.add_method(method_info, details.insert_arguments);
                }
            },
            CompletionContent::Type(details) => {
                for ty in &details.available_types {
                    items.add_type(ty.clone(), None, false);
                }

                if let Some(ty) = &details.self_type {
                    items.add_type(ty.clone(), Some(SELF_TYPE_NAME), false);
                }
            },
            CompletionContent::Event(details) => {
                for ty in &details.available_events {
                    if ty.get_type_blueprint().borrow().name.as_str() != BuiltinType::Event.get_name() {
                        items.add_event(ty.clone(), details.insert_brackets);
                    }
                }
            },
            CompletionContent::Interface(interfaces) => {
                for interface in interfaces {
                    items.add_interface(interface.clone());
                }
            },
            CompletionContent::Variable(details) => {
                for var_info in &details.available_variables {
                    items.add_variable(var_info.clone());
                }

                for constant_wrapped in &details.available_globals {
                    items.add_variable(constant_wrapped.borrow().var_info.clone());
                }

                for function_wrapped in &details.available_functions {
                    items.add_function(function_wrapped.clone(), details.insert_arguments);
                }

                for type_wrapped in &details.available_types {
                    items.add_type(type_wrapped.borrow().self_type.clone(), None, true);
                }

                for typedef_wrapped in &details.available_typedefs {
                    items.add_type(typedef_wrapped.borrow().target.clone(), None, true);
                }

                if let Some(type_blueprint) = &details.self_type {
                    items.add_type(type_blueprint.borrow().self_type.clone(), Some(SELF_TYPE_NAME), true);
                }
            },
        }

        items.consume()
    }
}