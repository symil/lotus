use enum_iterator::IntoEnumIterator;
use parsable::{ItemLocation, Parsable};
use crate::{program::{Type, InterfaceBlueprint, VariableInfo, GlobalVarBlueprint, TypeBlueprint, TypedefBlueprint, FunctionBlueprint, FieldKind, SELF_TYPE_NAME, BuiltinType, NONE_LITERAL, EXPRESSION_KEYWORDS}, utils::Link, items::{ParsedBooleanLiteralToken, ParsedActionKeywordToken, ParsedMatchKeyword}};
use super::{CompletionItem, CompletionItemList};

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
    pub available_events: Vec<Type>,
    pub insert_brackets: bool
}

#[derive(Debug)]
pub struct FieldCompletionDetails {
    pub parent_type: Type,
    pub show_methods: bool,
    pub insert_arguments: bool
}

#[derive(Debug)]
pub struct TypeCompletionDetails {
    pub available_types: Vec<Type>,
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
        let show_internals = source_location.file.package_root_path.ends_with("prelude"); // TODO do this more properly
        let mut items = CompletionItemList::new();

        match self {
            Self::Keyword(details) => {
                for keyword in &details.available_keywords {
                    items.add_keyword(keyword);
                }
            },
            Self::FieldOrMethod(details) => {
                for field_info in details.parent_type.get_all_fields() {
                    items.add_field(field_info);
                }

                if details.show_methods {
                    for method_info in details.parent_type.get_all_methods(FieldKind::Regular) {
                        items.add_method(method_info, details.insert_arguments, false, show_internals);
                    }
                }
            },
            Self::StaticFieldOrMethod(details) => {
                for variant in details.parent_type.get_all_variants() {
                    items.add_enum_variant(variant, false);
                }

                if details.show_methods {
                    for method_info in details.parent_type.get_all_methods(FieldKind::Static) {
                        items.add_method(method_info, details.insert_arguments, false, show_internals);
                    }
                }
            },
            Self::Type(details) => {
                for ty in &details.available_types {
                    items.add_type(ty.clone(), None, false);
                }

                if let Some(ty) = &details.self_type {
                    items.add_type(ty.clone(), Some(SELF_TYPE_NAME), false);
                }
            },
            Self::Event(details) => {
                for ty in &details.available_events {
                    if ty.get_type_blueprint().borrow().name.as_str() != BuiltinType::Event.get_name() {
                        items.add_event(ty.clone(), details.insert_brackets);
                    }
                }
            },
            Self::Interface(details) => {
                for interface in &details.available_interfaces {
                    items.add_interface(interface.clone());
                }
            },
            Self::Variable(details) => {
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
                    items.add_type(type_wrapped.borrow().self_type.clone(), None, false);
                }

                for typedef_wrapped in &details.available_typedefs {
                    items.add_type(typedef_wrapped.borrow().target.clone(), None, false);
                }

                if let Some(type_blueprint) = &details.self_type {
                    items.add_type(type_blueprint.borrow().self_type.clone(), Some(SELF_TYPE_NAME), false);
                }

                if let Some(ty) = &details.expected_type {
                    if ty.is_enum() {
                        ty.get_type_blueprint().with_ref(|type_blueprint| {
                            for variant in type_blueprint.enum_variants.values() {
                                items.add_enum_variant(variant.clone(), true)
                            }
                        });
                    }

                    for method in ty.get_all_methods(FieldKind::Static) {
                        method.with_ref(|method_blueprint| {
                            if method_blueprint.parameters.is_empty() {
                                if method_blueprint.signature.return_type.replace_parameters(Some(ty), &[]).is_assignable_to(ty) {
                                    items.add_method(method.clone(), details.insert_arguments, true, show_internals);
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
                        items.add_type(ty.clone(), None, false);
                    }
                } else if details.matched_type.is_enum() {
                    details.matched_type.get_type_blueprint().with_ref(|type_blueprint| {
                        for variant in type_blueprint.enum_variants.values() {
                            items.add_enum_variant(variant.clone(), true)
                        }
                    });
                }

                items.add_literal(NONE_LITERAL);
            },
            Self::Enum(enum_type) => {
                for variant in enum_type.get_all_variants() {
                    items.add_enum_variant(variant.clone(), false)
                }
            }
        }

        items.consume()
    }
}