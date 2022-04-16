use std::{collections::{HashMap, HashSet}, rc::Rc, iter::FromIterator};
use colored::Colorize;
use indexmap::{IndexMap, IndexSet};
use parsable::{parsable, ItemLocation};
use crate::{items::ParsedTypeQualifier, program::{OBJECT_CREATE_METHOD_NAME, ProgramContext, Type, VariableInfo, VariableKind, Vasm, TypeContent, NONE_METHOD_NAME}};
use super::{ParsedExpression, Identifier, ParsedObjectInitializationItem, ParsedType, ParsedOpeningCurlyBracket, ParsedClosingCurlyBracket};

#[parsable]
pub struct ParsedObjectLiteral {
    pub object_type: ParsedType,
    pub opening_bracket: ParsedOpeningCurlyBracket,
    pub body: ParsedObjectLiteralInitializationBody,
    pub closing_bracket: ParsedClosingCurlyBracket,
}

#[parsable]
pub struct ParsedObjectLiteralInitializationBody {
    pub items: Vec<ParsedObjectInitializationItem>,
}

impl ParsedObjectLiteral {
    pub fn collect_instancied_type_names(&self, list: &mut Vec<String>, context: &mut ProgramContext) {
        self.object_type.collect_instancied_type_names(list, context);
    }

    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        if let Some(object_type) = self.object_type.process(true, type_hint, context) {
            let first_half_location = self.opening_bracket.location.until(&self.body);
            let second_half_location = self.body.location.until(&self.closing_bracket);

            let fill_required_fields = || fill_class_fields(&object_type, &self.body.items, true);
            let fill_all_fields = || fill_class_fields(&object_type, &self.body.items, false);

            for location in &[first_half_location, second_half_location] {
                context.completion_provider.add_field_completion(&location, &object_type, None, None);
                context.code_actions_provider.add_replace_action(&location, "Fill required fields", None, fill_required_fields);
                context.code_actions_provider.add_replace_action(&location, "Fill all fields", None, fill_all_fields);
            }

            result = instanciate_object(&self.object_type, &self.body.items, type_hint, context);
        }

        result
    }
}

fn fill_class_fields(ty: &Type, initialization_items: &[ParsedObjectInitializationItem], exclude_non_required: bool) -> Option<String> {
    let mut fields = ty.get_all_fields();
    let mut field_names : IndexSet<&str> = match exclude_non_required {
        true => IndexSet::from_iter(fields.iter().filter(|field_info| field_info.is_required).map(|field_info| field_info.name.as_str())),
        false => IndexSet::from_iter(fields.iter().map(|field_info| field_info.name.as_str())),
    };
    let mut result = None;

    for field in initialization_items {
        if let ParsedObjectInitializationItem::FieldInitialization(field_initialization) = field {
            field_names.remove(field_initialization.name.as_str());
        }
    }

    if !field_names.is_empty() {
        result = Some(field_names.iter()
            .map(|name| format!("{}: none,", name))
            .collect::<Vec<String>>()
            .join("\n")
        );
    }

    result
}

pub fn instanciate_object(parsed_object_type: &ParsedType, initialization_items: &[ParsedObjectInitializationItem], type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
    let object_type = parsed_object_type.process(true, type_hint, context)?;

    let mut result = context.vasm()
        .set_type(&object_type);

    if let TypeContent::Actual(info) = object_type.content() {
        let object_var = VariableInfo::tmp("object", object_type.clone());
        let type_unwrapped = info.type_blueprint.borrow();

        if type_unwrapped.is_class() {
            let mut fields_init = HashMap::new();

            result = result
                .declare_variable(&object_var)
                .call_static_method(&object_type, OBJECT_CREATE_METHOD_NAME, &[], vec![], context)
                .set_tmp_var(&object_var);

            for (i, item) in initialization_items.iter().enumerate() {
                let is_last = i == initialization_items.len() - 1;
                let init_info = item.process(&object_type, is_last, context);

                for (field_name, vasm) in init_info.fields {
                    fields_init.insert(field_name, vasm);
                }

                if let Some(vasm) = init_info.init {
                    result = result.append(vasm);
                }
            }

            for (i, field_info) in type_unwrapped.fields.values().enumerate() {
                let field_name = field_info.name.as_str();
                let field_type = field_info.ty.replace_parameters(Some(&object_type), &[]);
                let specified_value = fields_init.remove(field_name);
                let is_value_specified = specified_value.is_some();
                let init_vasm = match specified_value {
                    Some(field_vasm) => field_vasm,
                    None => field_info.get_default_vasm(&object_var, context)
                };

                if field_info.is_required && !is_value_specified {
                    context.errors.generic(parsed_object_type, format!("missing field `{}`", field_name));
                }

                result = result
                    .get_tmp_var(&object_var)
                    .set_field(&field_type, field_info.offset, None, init_vasm);
            }

            result = result
                .get_tmp_var(&object_var)
                .set_type(&object_type);
        } else {
            context.errors.generic(parsed_object_type, format!("type `{}` is not a class", &object_type));
        }
    } else {
        context.errors.generic(parsed_object_type, format!("cannot instanciate type `{}` this way", &object_type));
    }

    Some(result)
}