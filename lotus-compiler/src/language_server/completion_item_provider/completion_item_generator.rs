use std::borrow::Borrow;

use enum_iterator::IntoEnumIterator;
use parsable::{ItemLocation, Parsable};
use crate::{program::{Type, InterfaceBlueprint, VariableInfo, GlobalVarBlueprint, TypeBlueprint, TypedefBlueprint, FunctionBlueprint, FieldKind, SELF_TYPE_NAME, BuiltinType, NONE_LITERAL, EXPRESSION_KEYWORDS}, utils::Link, items::{ParsedBooleanLiteralToken, ParsedActionKeywordToken}};
use super::{CompletionItem, CompletionItemList, FieldCompletionOptions};

#[derive(Debug)]
pub enum CompletionItemGenerator {
    Keyword(KeywordCompletionDetails),
    FieldOrMethod(FieldCompletionDetails),
    StaticFieldOrMethod(FieldCompletionDetails),
    Event(EventCompletionDetails),
    Interface(InterfaceCompletionDetails),
    Type(TypeCompletionDetails),
    Variable(VariableCompletionDetails),
    MatchItem(MatchItemCompletionDetails),
    Enum(Type)
}

#[derive(Debug)]
pub struct KeywordCompletionDetails {
    pub available_keywords: Vec<&'static str>
}

#[derive(Debug)]
pub struct InterfaceCompletionDetails {
    pub available_interfaces: Vec<Link<InterfaceBlueprint>>
}

#[derive(Debug)]
pub struct MatchItemCompletionDetails {
    pub matched_type: Type,
    pub available_types: Vec<Type>,
}

#[derive(Debug)]
pub struct EventCompletionDetails {
    pub current_type: Type,
    pub self_event_type: Option<Type>,
    pub available_events: Vec<Type>,
    pub insert_brackets: bool
}

#[derive(Debug)]
pub struct FieldCompletionDetails {
    pub parent_type: Type,
    pub expected_type: Option<Type>,
    pub options: FieldCompletionOptions
}

#[derive(Debug)]
pub struct TypeCompletionDetails {
    pub available_types: Vec<Type>,
    pub expected_type: Option<Type>,
    pub self_type: Option<Type>
}

#[derive(Debug)]
pub struct VariableCompletionDetails {
    pub available_variables: Vec<VariableInfo>,
    pub available_globals: Vec<Link<GlobalVarBlueprint>>,
    pub available_functions: Vec<Link<FunctionBlueprint>>,
    pub available_types: Vec<Link<TypeBlueprint>>,
    pub available_typedefs: Vec<Link<TypedefBlueprint>>,
    pub self_type: Option<Link<TypeBlueprint>>,
    pub insert_arguments: bool,
    pub expected_type: Option<Type>
}

impl CompletionItemGenerator {
    pub fn generate(&self, source_location: &ItemLocation) -> Vec<CompletionItem> {
        let show_internals = source_location.file.package_root_path.ends_with("prelude/src"); // TODO do this more properly
        let mut items = CompletionItemList::new();

        match self {
            Self::Keyword(details) => {
                for keyword in &details.available_keywords {
                    items.add_keyword(keyword);
                }
            },
            Self::FieldOrMethod(details) => {
                for field_info in details.parent_type.get_all_fields() {
                    if !details.options.hide_private || !field_info.visibility.is_private() {
                        items.add_field(field_info, details.expected_type.as_ref(), details.options.prefix, details.options.suffix);
                    }
                }

                if details.options.show_methods {
                    for method_info in details.parent_type.get_all_methods(FieldKind::Regular) {
                        if !details.options.hide_private || !method_info.borrow().method_details.as_ref().unwrap().visibility.is_private() {
                            items.add_method(method_info, details.expected_type.as_ref(), details.options.insert_arguments, false, show_internals);
                        }
                    }
                } else if details.options.insert_dynamic_methods {
                    for method_wrapped in details.parent_type.get_all_methods(FieldKind::Regular) {
                        method_wrapped.with_ref(|method_unwrapped| {
                            let method_details = method_unwrapped.method_details.as_ref().unwrap();
                            let is_dynamic = method_unwrapped.is_dynamic();
                            let defined_by_parent = true;

                            if defined_by_parent && is_dynamic && (!details.options.hide_private || !method_details.visibility.is_private()) {
                                items.add_dynamic_method_body(method_wrapped.clone(), show_internals);
                            }
                        });
                    }
                }
            },
            Self::StaticFieldOrMethod(details) => {
                for variant in details.parent_type.get_all_variants() {
                    items.add_enum_variant(variant, details.expected_type.as_ref(), false);
                }

                if details.options.show_methods {
                    for method_info in details.parent_type.get_all_methods(FieldKind::Static) {
                        items.add_method(method_info, details.expected_type.as_ref(), details.options.insert_arguments, false, show_internals);
                    }
                }
            },
            Self::Type(details) => {
                for ty in &details.available_types {
                    items.add_type(ty.clone(), details.expected_type.as_ref(), None, false);
                }

                if let Some(ty) = &details.self_type {
                    items.add_type(ty.clone(), details.expected_type.as_ref(), Some(SELF_TYPE_NAME), false);
                }
            },
            Self::Event(details) => {
                let type_wrapped = details.current_type.get_type_blueprint();

                type_wrapped.with_ref(|type_unwrapped| {
                    // let event_class_name = BuiltinType::Event.get_name();
                    let event_list = details.self_event_type.iter()
                        .chain(details.available_events.iter());

                    for (i, ty) in event_list.enumerate() {
                        let event_type_wrapped = ty.get_type_blueprint();

                        // if event_type_wrapped.borrow().name.as_str() == event_class_name {
                        //     continue;
                        // }

                        let is_self = i == 0 && details.self_event_type.is_some();
                        let last_event_callback = type_unwrapped.event_callbacks.get(&event_type_wrapped)
                            .and_then(|list| list.last())
                            .filter(|callback| callback.declarer == type_wrapped);

                        items.add_event(ty.clone(), details.insert_brackets, is_self, None, 0);

                        if let Some(callback_details) = last_event_callback {
                            if callback_details.progress.is_none() {
                                items.add_event(ty.clone(), details.insert_brackets, is_self, Some(":progress"), 1);
                            }

                            if callback_details.end.is_none() {
                                items.add_event(ty.clone(), details.insert_brackets, is_self, Some(":end"), 2);
                            }
                        }
                    }
                });
            },
            Self::Interface(details) => {
                for interface in &details.available_interfaces {
                    items.add_interface(interface.clone());
                }
            },
            Self::Variable(details) => {
                for var_info in &details.available_variables {
                    items.add_variable(var_info.clone(), details.expected_type.as_ref());
                }

                for constant_wrapped in &details.available_globals {
                    items.add_variable(constant_wrapped.borrow().var_info.clone(), details.expected_type.as_ref());
                }

                for function_wrapped in &details.available_functions {
                    let is_expected_function = details.expected_type.as_ref().map(|ty| ty.is_function()).unwrap_or(false);

                    items.add_function(function_wrapped.clone(), details.expected_type.as_ref(), details.insert_arguments && !is_expected_function);
                }

                for type_wrapped in &details.available_types {
                    items.add_type(type_wrapped.borrow().self_type.clone(), details.expected_type.as_ref(), None, false);
                }

                for typedef_wrapped in &details.available_typedefs {
                    items.add_type(typedef_wrapped.borrow().target.clone(), details.expected_type.as_ref(), None, false);
                }

                if let Some(type_blueprint) = &details.self_type {
                    items.add_type(type_blueprint.borrow().self_type.clone(), details.expected_type.as_ref(), Some(SELF_TYPE_NAME), false);
                }

                if let Some(ty) = &details.expected_type {
                    if ty.is_enum() {
                        ty.get_type_blueprint().with_ref(|type_blueprint| {
                            for variant in type_blueprint.enum_variants.values() {
                                items.add_enum_variant(variant.clone(), details.expected_type.as_ref(), true)
                            }
                        });
                    }

                    for method in ty.get_all_methods(FieldKind::Static) {
                        method.with_ref(|method_blueprint| {
                            if method_blueprint.parameters.is_empty() {
                                if method_blueprint.signature.return_type.replace_parameters(Some(ty), &[]).is_assignable_to(ty) {
                                    items.add_method(method.clone(), details.expected_type.as_ref(), details.insert_arguments, true, show_internals);
                                }
                            }
                        });
                    }
                }

                for value in ParsedBooleanLiteralToken::into_enum_iter() {
                    items.add_literal(value.as_str());
                }
                items.add_literal(NONE_LITERAL);

                for keyword in ParsedActionKeywordToken::into_enum_iter() {
                    let value = format!("{}", keyword);

                    items.add_keyword(&value);
                }

                for keyword in EXPRESSION_KEYWORDS {
                    items.add_keyword(keyword);
                }
            },
            Self::MatchItem(details) => {
                if details.matched_type.is_bool() {
                    for value in ParsedBooleanLiteralToken::into_enum_iter() {
                        items.add_literal(value.as_str());
                    }
                } else if details.matched_type.is_object() {
                    for ty in &details.available_types {
                        items.add_type(ty.clone(), Some(&details.matched_type), None, false);
                    }
                } else if details.matched_type.is_enum() {
                    details.matched_type.get_type_blueprint().with_ref(|type_blueprint| {
                        for variant in type_blueprint.enum_variants.values() {
                            items.add_enum_variant(variant.clone(), Some(&details.matched_type), true)
                        }
                    });
                }

                items.add_literal(NONE_LITERAL);
            },
            Self::Enum(enum_type) => {
                for variant in enum_type.get_all_variants() {
                    items.add_enum_variant(variant.clone(), None, false)
                }
            }
        }

        items.consume()
    }
}