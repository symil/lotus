use std::{mem::take, rc::Rc};
use parsable::DataLocation;

use crate::{utils::Link, program::{FunctionBlueprint, VariableInfo, FieldInfo, EnumVariantInfo, Type, InterfaceBlueprint}};
use super::{CompletionItem, CompletionItemKind};

const INTERNAL_ITEM_PREFIX : &'static str = "__";

pub struct CompletionItemList {
    list: Vec<CompletionItem>,
    current: Option<CompletionItem>
}

impl CompletionItemList {
    pub fn new() -> Self {
        Self {
            list: vec![],
            current: None
        }
    }

    fn store(&mut self) {
        if let Some(item) = take(&mut self.current) {
            self.list.push(item);
        }
    }

    fn with_current<F : FnOnce(&mut CompletionItem)>(&mut self, callback: F) -> &mut Self {
        if let Some(item) = &mut self.current {
            callback(item);
        }
        self
    }

    pub fn add<S : ToString>(&mut self, label: S) -> &mut Self {
        self.store();
        self.current = Some(CompletionItem::new(label.to_string()));
        self
    }

    pub fn kind(&mut self, kind: CompletionItemKind) -> &mut Self {
        self.with_current(|item| item.kind = Some(kind))
    }

    pub fn description(&mut self, description: String) -> &mut Self {
        self.with_current(|item| item.description = Some(description))
    }

    pub fn detail(&mut self, detail: String) -> &mut Self {
        self.with_current(|item| item.detail = Some(detail))
    }

    pub fn documentation(&mut self, documentation: String) -> &mut Self {
        self.with_current(|item| item.documentation = Some(documentation))
    }

    pub fn insert_text(&mut self, insert_text: String) -> &mut Self {
        self.with_current(|item| item.insert_text = Some(insert_text))
    }

    pub fn consume(mut self) -> Vec<CompletionItem> {
        self.store();
        
        self.list
    }

    pub fn add_variable(&mut self, variable: VariableInfo) {
        self
            .add(variable.name().as_str())
            .kind(CompletionItemKind::Variable)
            .description(variable.ty().to_string());
    }

    pub fn add_field(&mut self, field: Rc<FieldInfo>) {
        self
            .add(field.name.as_str())
            .kind(CompletionItemKind::Field)
            .description(field.ty.to_string());
    }

    pub fn add_enum_variant(&mut self, variant: Rc<EnumVariantInfo>) {
        self
            .add(variant.name.as_str())
            .kind(CompletionItemKind::EnumMember)
            .description(variant.owner.borrow().self_type.to_string());
    }

    pub fn add_function(&mut self, function: Link<FunctionBlueprint>, insert_arguments: bool) {
        function.with_ref(|function_unwrapped| {
            let function_name = function_unwrapped.name.as_str();
            let is_internal_method = function_name.starts_with(INTERNAL_ITEM_PREFIX);
            let should_display_internal_methods = false;
            // let should_display_internal_methods = location.as_str().starts_with(INTERNAL_METHOD_PREFIX);

            if is_internal_method != should_display_internal_methods {
                return;
            }

            let kind = match function_unwrapped.owner_type.is_some() || function_unwrapped.owner_interface.is_some() {
                true => CompletionItemKind::Method,
                false => CompletionItemKind::Function,
            };

            let insert_text = match insert_arguments {
                true => {
                    let mut text = format!("{}(", function_name);

                    for (i, arg) in function_unwrapped.argument_names.iter().enumerate() {
                        text.push_str(&format!("${{{}:{}}}", i + 1, arg.as_str()));

                        if i != function_unwrapped.argument_names.len() - 1 {
                            text.push_str(", ");
                        }
                    }

                    text.push_str(")");
                    text
                },
                false => function_name.to_string(),
            };

            self
                .add(format!("{}(…)", function_name))
                .kind(kind)
                .description(function_unwrapped.get_self_type().to_string())
                .insert_text(insert_text);
        });
    }

    pub fn add_method(&mut self, method: Link<FunctionBlueprint>, insert_arguments: bool) {
        self.add_function(method, insert_arguments);
    }

    pub fn add_event(&mut self, event_type: Type, insert_brackets: bool) {
        let label = event_type.to_string();
        let mut insert_text = label.clone();

        if insert_brackets {
            insert_text.push_str(" {\n\t$0\n}");
        }

        self
            .add(label)
            .kind(CompletionItemKind::Event)
            .insert_text(insert_text);
    }

    pub fn add_type(&mut self, ty: Type, custom_type_name: Option<&str>, insert_double_colon_if_enum: bool) {
        let parameters = ty.get_parameters();
        let has_parameters = !parameters.is_empty();
        let type_name = replace_string(&ty.to_string(), '<', '>', "");

        let mut label = match has_parameters {
            true =>  format!("{}<…>", &type_name),
            false => type_name.clone()
        };

        if let Some(name) = custom_type_name {
            label = name.to_string();
        }

        let mut insert_text = label.clone();
        
        if !parameters.is_empty() {
            insert_text = format!("{}<", &type_name);

            for (i, param) in parameters.iter().enumerate() {
                insert_text.push_str(&format!("${{{}:{}}}", i + 1, param.to_string()));

                if i != parameters.len() - 1 {
                    insert_text.push_str(", ");
                }
            }

            insert_text.push_str(">");
        }

        if ty.is_enum() && insert_double_colon_if_enum {
            insert_text.push_str("::");
        }

        self
            .add(format!("{}", label))
            .kind(CompletionItemKind::Class)
            .description(format!("(type) {}", &type_name))
            .insert_text(insert_text);
    }

    pub fn add_interface(&mut self, interface: Link<InterfaceBlueprint>) {
        interface.with_ref(|interface_unwrapped| {
            let interface_name = interface_unwrapped.name.as_str();
            let is_internal = interface_name.starts_with(INTERNAL_ITEM_PREFIX);
            let should_display_internal = false;

            if is_internal == should_display_internal {
                self
                    .add(format!("{}", interface_name))
                    .kind(CompletionItemKind::Interface);
            }
        });
    }
}

fn replace_string(string: &str, start_char: char, end_char: char, replacement: &str) -> String {
    let mut result = string.to_string();
    let start = string.find('<');
    let end = string.rfind('>').map(|index| index + 1);

    if let (Some(i), Some(j)) = (start, end) {
        result.replace_range(i..j, replacement);
    }

    result
}