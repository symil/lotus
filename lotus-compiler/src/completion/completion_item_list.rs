use std::{mem::take, rc::Rc};
use parsable::DataLocation;

use crate::{utils::Link, program::{FunctionBlueprint, VariableInfo, FieldInfo, EnumVariantInfo, Type}};
use super::{CompletionItem, CompletionItemKind};

const INTERNAL_METHOD_PREFIX : &'static str = "__";

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

    pub fn add_function(&mut self, function: Link<FunctionBlueprint>) {
        self
            .add(format!("{}(…)", function.borrow().name.as_str()))
            .kind(CompletionItemKind::Function)
            .description(function.borrow().get_self_type().to_string());
    }

    pub fn add_method(&mut self, method: Link<FunctionBlueprint>, location: &DataLocation) {
        // println!("{}", method.borrow().name.as_str());
        let is_internal_method = method.borrow().name.as_str().starts_with(INTERNAL_METHOD_PREFIX);
        // let should_display_internal_methods = location.as_str().starts_with(INTERNAL_METHOD_PREFIX);
        let should_display_internal_methods = false;

        if is_internal_method == should_display_internal_methods {
            self
                .add(format!("{}(…)", method.borrow().name.as_str()))
                .kind(CompletionItemKind::Method)
                .description(method.borrow().get_self_type().to_string());
        }
    }

    pub fn add_type(&mut self, ty: Type, custom_type_name: Option<&str>) {
        let type_name = match custom_type_name {
            Some(name) => name.to_string(),
            None => ty.to_string(),
        };

        self
            .add(format!("{}", type_name))
            .kind(CompletionItemKind::Class)
            .description(format!("(type) {}", ty.to_string()));
    }
}