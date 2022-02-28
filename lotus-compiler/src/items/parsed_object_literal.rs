use std::{collections::HashMap, rc::Rc};
use colored::Colorize;
use indexmap::IndexMap;
use parsable::parsable;
use crate::{items::ParsedTypeQualifier, program::{OBJECT_CREATE_METHOD_NAME, ProgramContext, Type, VariableInfo, VariableKind, Vasm, TypeContent}};
use super::{ParsedExpression, Identifier, ParsedObjectInitializationItem, ParsedType, ParsedOpeningCurlyBracket, ParsedClosingCurlyBracket};

#[parsable]
#[derive(Default)]
pub struct ParsedObjectLiteral {
    pub object_type: ParsedType,
    // pub field_list: Option<ObjectFieldInitializationList>
    pub opening_bracket: ParsedOpeningCurlyBracket,
    pub body: ParsedObjectLiteralInitializationBody,
    pub closing_bracket: ParsedClosingCurlyBracket,
}

#[parsable]
#[derive(Default)]
pub struct ParsedObjectLiteralInitializationBody {
    pub items: Vec<ParsedObjectInitializationItem>,
}

impl ParsedObjectLiteral {
    pub fn collect_instancied_type_names(&self, list: &mut Vec<String>, context: &mut ProgramContext) {
        self.object_type.collect_instancied_type_names(list, context);
    }

    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = context.vasm();

        if let Some(object_type) = self.object_type.process(true, context) {
            result = result.set_type(&object_type);

            let first_half_location = self.opening_bracket.location.until(&self.body);
            let second_half_location = self.body.location.until(&self.closing_bracket);

            context.completion_provider.add_field_completion(&first_half_location, &object_type, None);
            context.completion_provider.add_field_completion(&second_half_location, &object_type, None);

            if let TypeContent::Actual(info) = object_type.content() {
                let object_var = VariableInfo::tmp("object", context.int_type());
                let type_unwrapped = info.type_blueprint.borrow();

                if type_unwrapped.is_class() {
                    let mut fields_init = HashMap::new();

                    result = result
                        .declare_variable(&object_var)
                        .call_static_method(&object_type, OBJECT_CREATE_METHOD_NAME, &[], vec![], context)
                        .set_tmp_var(&object_var);

                    for (i, item) in self.body.items.iter().enumerate() {
                        let is_last = i == self.body.items.len() - 1;
                        let init_info = item.process(&object_type, is_last, context);

                        for (field_name, vasm) in init_info.fields {
                            fields_init.insert(field_name, vasm);
                        }

                        if let Some(vasm) = init_info.init {
                            result = result.append(vasm);
                        }
                    }

                    for (i, field_info) in type_unwrapped.fields.values().enumerate() {
                        let field_type = field_info.ty.replace_parameters(Some(&object_type), &[]);
                        let init_vasm = match fields_init.remove(field_info.name.as_str()) {
                            Some(field_vasm) => field_vasm,
                            None => field_info.default_value.clone(),
                        };

                        result = result
                            .get_tmp_var(&object_var)
                            .set_field(&field_type, field_info.offset, init_vasm);
                    }

                    result = result
                        .get_tmp_var(&object_var)
                        .set_type(&object_type);
                } else {
                    context.errors.generic(&self.object_type, format!("type `{}` is not a class", &object_type));
                }
            } else {
                context.errors.generic(&self.object_type, format!("cannot manually instanciate type `{}`", &object_type));
            }
        }

        Some(result)
    }
}