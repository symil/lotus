use parsable::DataLocation;
use crate::{program::{Type, FieldKind, VariableInfo, FunctionBlueprint, TypeBlueprint, TypedefBlueprint, GlobalVarBlueprint, BuiltinType, SELF_TYPE_NAME, InterfaceBlueprint}, utils::Link};
use super::{CompletionItem, CompletionItemKind, CompletionItemList};

#[derive(Debug)]
pub struct CompletionArea {
    pub location: DataLocation,
    pub details: CompletionDetails
}

#[derive(Debug)]
pub enum CompletionDetails {
    Field(Type),
    StaticField(Type),
    Event(Vec<Type>),
    Interface(Vec<Link<InterfaceBlueprint>>),
    Type(Vec<Type>, Option<Type>), // list of types, current type
    Variable(Vec<VariableInfo>, Vec<Link<GlobalVarBlueprint>>, Vec<Link<FunctionBlueprint>>, Vec<Link<TypeBlueprint>>, Vec<Link<TypedefBlueprint>>, Option<Link<TypeBlueprint>>)
}

impl CompletionArea {
    pub fn contains_cursor(&self, cursor_index: usize) -> bool {
        self.location.start <= cursor_index && self.location.end >= cursor_index
    }

    pub fn provide_completion_items(&self) -> Vec<CompletionItem> {
        let mut items = CompletionItemList::new();

        match &self.details {
            CompletionDetails::Field(parent_type) => {
                for field_info in parent_type.get_all_fields() {
                    items.add_field(field_info);
                }

                for method_info in parent_type.get_all_methods(FieldKind::Regular) {
                    items.add_method(method_info);
                }
            },
            CompletionDetails::StaticField(parent_type) => {
                for variant in parent_type.get_all_variants() {
                    items.add_enum_variant(variant);
                }

                for method_info in parent_type.get_all_methods(FieldKind::Static) {
                    items.add_method(method_info);
                }
            },
            CompletionDetails::Type(types, current_type) => {
                for ty in types {
                    items.add_type(ty.clone(), None);
                }

                if let Some(ty) = current_type {
                    items.add_type(ty.clone(), Some(SELF_TYPE_NAME));
                }
            },
            CompletionDetails::Event(types) => {
                for ty in types {
                    if ty.get_type_blueprint().borrow().name.as_str() != BuiltinType::Event.get_name() {
                        items.add_type(ty.clone(), None);
                    }
                }
            },
            CompletionDetails::Interface(interfaces) => {
                for interface in interfaces {
                    items.add_interface(interface.clone());
                }
            },
            CompletionDetails::Variable(variables, constants, functions, types, typedefs, current_type) => {
                for var_info in variables {
                    items.add_variable(var_info.clone());
                }

                for constant_wrapped in constants {
                    items.add_variable(constant_wrapped.borrow().var_info.clone());
                }

                for function_wrapped in functions {
                    items.add_function(function_wrapped.clone());
                }

                for type_wrapped in types {
                    items.add_type(type_wrapped.borrow().self_type.clone(), None);
                }

                for typedef_wrapped in typedefs {
                    items.add_type(typedef_wrapped.borrow().target.clone(), None);
                }

                if let Some(type_blueprint) = current_type {
                    items.add_type(type_blueprint.borrow().self_type.clone(), Some(SELF_TYPE_NAME));
                }
            },
        }

        items.consume()
    }
}