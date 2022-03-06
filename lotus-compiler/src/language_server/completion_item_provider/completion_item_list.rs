use std::{mem::take, rc::Rc, fmt::format};
use parsable::ItemLocation;
use crate::{utils::Link, program::{FunctionBlueprint, VariableInfo, FieldInfo, EnumVariantInfo, Type, InterfaceBlueprint, NONE_LITERAL}, language_server::Range};
use super::{CompletionItem, CompletionItemKind, CompletionItemPosition, CompletionItemVisibility, CompletionItemCommand};

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

    pub fn position(&mut self, position: CompletionItemPosition) -> &mut Self {
        self.with_current(|item| item.position = Some(position))
    }

    pub fn kind(&mut self, kind: CompletionItemKind) -> &mut Self {
        self.with_current(|item| item.kind = Some(kind))
    }

    pub fn range(&mut self, range: Range) -> &mut Self {
        self.with_current(|item| item.range = Some(range))
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

    pub fn filter_text(&mut self, filter_text: String) -> &mut Self {
        self.with_current(|item| item.filter_text = Some(filter_text))
    }

    pub fn command(&mut self, command: CompletionItemCommand) -> &mut Self {
        self.with_current(|item| item.command = Some(command))
    }
    
    pub fn consume(mut self) -> Vec<CompletionItem> {
        self.store();
        
        self.list
    }

    pub fn add_variable(&mut self, variable: VariableInfo) {
        let variable_name = variable.name().as_str().to_string();

        self
            .add(&variable_name)
            .position(CompletionItemPosition::from_visibility(CompletionItemVisibility::from_str(&variable_name), false))
            .kind(CompletionItemKind::Variable)
            .description(variable.ty().to_string());
    }

    pub fn add_field(&mut self, field: Rc<FieldInfo>, prefix: &'static str, suffix: &'static str) {
        let field_name = field.name.as_str();
        let label = format!("{}{}", prefix, field_name);
        let insert_text = format!("{}{}", label, suffix);

        self
            .add(label)
            .insert_text(insert_text)
            .position(CompletionItemPosition::from_visibility(CompletionItemVisibility::from_str(field_name), false))
            .kind(CompletionItemKind::Field)
            .description(field.ty.to_string());
    }

    pub fn add_enum_variant(&mut self, variant: Rc<EnumVariantInfo>, show_owner: bool) {
        let variant_name = variant.name.as_str();
        let owner_type = variant.owner.borrow().self_type.clone();
        let label = match show_owner {
            true => format!("{}::{}", owner_type.to_string(), variant_name),
            false => variant_name.to_string(),
        };

        self
            .add(label)
            .position(CompletionItemPosition::EnumMember)
            .kind(CompletionItemKind::EnumMember)
            .description(owner_type.to_string())
            .filter_text(variant_name.to_string());
    }

    pub fn add_literal(&mut self, literal: &str) {
        self
            .add(literal)
            .position(CompletionItemPosition::Literal)
            .kind(CompletionItemKind::EnumMember);
    }

    pub fn add_keyword(&mut self, keyword: &str) {
        self
            .add(keyword)
            .position(CompletionItemPosition::Keyword)
            .kind(CompletionItemKind::Keyword);
    }

    fn add_function_or_method(&mut self, function: Link<FunctionBlueprint>, insert_arguments: bool, show_owner: bool, show_internals: bool) {
        function.with_ref(|function_unwrapped| {
            let function_name = function_unwrapped.name.as_str();
            let visibility = CompletionItemVisibility::from_str(function_name);

            if visibility.is_internal() && !show_internals {
                return;
            }

            let kind = match function_unwrapped.owner_type.is_some() || function_unwrapped.owner_interface.is_some() {
                true => CompletionItemKind::Method,
                false => CompletionItemKind::Function,
            };

            let mut insert_text = match insert_arguments {
                true => {
                    let mut text = format!("{}(", function_name);

                    for (i, arg) in function_unwrapped.arguments.iter().enumerate() {
                        text.push_str(&format!("${{{}:{}}}", i + 1, arg.name.as_str()));

                        if i != function_unwrapped.arguments.len() - 1 {
                            text.push_str(", ");
                        }
                    }

                    text.push_str(")");
                    text
                },
                false => function_name.to_string(),
            };

            let has_arguments = !function_unwrapped.arguments.is_empty();
            let parenthesis_content = match has_arguments {
                true => "…",
                false => "",
            };

            let mut label = match insert_arguments {
                true => format!("{}({})", function_name, parenthesis_content),
                false => function_name.to_string()
            };

            if show_owner {
                let prefix = if let Some(owner_type) = &function_unwrapped.owner_type {
                    Some(owner_type.borrow().self_type.to_string())
                } else if let Some(owner_interface) = &function_unwrapped.owner_interface {
                    Some(owner_interface.borrow().name.to_string())
                } else {
                    None
                };

                if let Some(string) = prefix {
                    label = format!("{}::{}", string, label);
                    insert_text = format!("{}::{}", string, insert_text);
                }
            }

            self
                .add(label)
                .position(CompletionItemPosition::from_visibility(visibility, true))
                .kind(kind)
                .description(function_unwrapped.get_self_type().to_string())
                .insert_text(insert_text)
                .filter_text(function_name.to_string());
            
            if insert_arguments {
                self.command(CompletionItemCommand::TriggerSignatureHelp);
            }
        });
    }

    pub fn add_function(&mut self, method: Link<FunctionBlueprint>, insert_arguments: bool) {
        self.add_function_or_method(method, insert_arguments, false, false);
    }

    pub fn add_method(&mut self, method: Link<FunctionBlueprint>, insert_arguments: bool, show_owner: bool, show_internals: bool) {
        self.add_function_or_method(method, insert_arguments, show_owner, show_internals);
    }

    pub fn add_dynamic_method_body(&mut self, method: Link<FunctionBlueprint>, show_internals: bool) {
        method.with_ref(|function_unwrapped| {
            let function_name = function_unwrapped.name.as_str();
            let visibility = CompletionItemVisibility::from_str(function_name);

            if visibility.is_internal() && !show_internals {
                return;
            }

            let content = match function_unwrapped.arguments.is_empty() {
                true => "",
                false => "…",
            };
            let arguments : Vec<String> = function_unwrapped.arguments.iter().map(|arg| format!("{}: {}", arg.name.as_str(), &arg.ty)).collect();
            let return_type = match function_unwrapped.signature.return_type.is_void() {
                true => String::new(),
                false => format!(" -> {}", &function_unwrapped.signature.return_type)
            };
            let label = format!("dyn {}({}) {{}}", function_name, content);
            let insert_text = format!("dyn {}({}){}{{\n\t$0\n}}", function_name, arguments.join(", "), return_type);

            self
                .add(label)
                .position(CompletionItemPosition::from_visibility(visibility, true))
                .kind(CompletionItemKind::Method)
                .description(function_unwrapped.get_self_type().to_string())
                .insert_text(insert_text)
                .filter_text(function_name.to_string());
        });
    }

    pub fn add_event(&mut self, event_type: Type, insert_brackets: bool) {
        let mut label = event_type.to_string();
        let mut insert_text = label.clone();

        if insert_brackets {
            label.push_str(" {}");
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

        let mut command = CompletionItemCommand::None;

        if ty.is_enum() && insert_double_colon_if_enum {
            insert_text.push_str("::");
            command = CompletionItemCommand::TriggerCompletion;
        }

        self
            .add(format!("{}", label))
            .position(CompletionItemPosition::PublicType)
            .kind(CompletionItemKind::Class)
            .description(format!("(type) {}", &type_name))
            .insert_text(insert_text)
            .command(command);
    }

    pub fn add_interface(&mut self, interface: Link<InterfaceBlueprint>) {
        interface.with_ref(|interface_unwrapped| {
            let interface_name = interface_unwrapped.name.as_str();
            let visibility = CompletionItemVisibility::from_str(interface_name);
            let should_display_internal = false;

            if visibility.is_internal() && !should_display_internal {
                return;
            }

            self
                .add(format!("{}", interface_name))
                .kind(CompletionItemKind::Interface);
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