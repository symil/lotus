use std::{fs, collections::HashMap};
use parsable::{ItemLocation, parsable, Token};
use crate::program::{ProgramContext, Vasm, Type, BuiltinType, PUSH_UNCHECKED_METHOD_NAME, PUSH_METHOD_NAME, NEW_METHOD_NAME, VariableInfo, OBJECT_CREATE_METHOD_NAME};
use super::{ParsedOpeningRoundBracket, ParsedClosingRoundBracket, ParsedStringLiteral, Identifier, ParsedCommaToken, unwrap_item, ParsedType, ParsedObjectInitializationItem, ParsedObjectFieldInitialization, make_string_value_from_literal_unchecked};

#[parsable(cascade = true)]
pub struct ParsedLoadDirective {
    pub load_keyword: Token<"#LOAD">,
    pub opening_bracket: Option<ParsedOpeningRoundBracket>,
    pub type_name: Option<ParsedType>,
    pub comma: Option<ParsedCommaToken>,
    pub sheet_name: Option<ParsedStringLiteral>,
    #[parsable(cascade = false)]
    pub closing_bracket: Option<ParsedClosingRoundBracket>,
}

const DEFAULT_INT_VALUE : i32 = 0;
const DEFAULT_FLOAT_VALUE : f32 = 0.;

impl ParsedLoadDirective {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let opening_bracket = unwrap_item(&self.opening_bracket, &self.load_keyword, context)?;
        let type_name = unwrap_item(&self.type_name, opening_bracket, context)?;
        let comma = unwrap_item(&self.comma, type_name, context)?;
        let sheet_name = unwrap_item(&self.sheet_name, comma, context)?;
        let closing_bracket = unwrap_item(&self.closing_bracket, self, context)?;
        let object_type = type_name.process(true, None, context)?;

        if !object_type.is_object() {
            context.errors.expected_class_type(type_name, &object_type);
            return None;
        }

        let sheet_file_name = format!("{}.tsv", sheet_name.value());
        let sheet_file_path = context.options.package.data_path.join(sheet_file_name);

        let content = fs::read_to_string(&sheet_file_path).ok().or_else(|| {
            context.errors.generic(sheet_name, format!("cannot read file `{}`", sheet_file_path.to_str().unwrap()));
            None
        })?;

        let array_type = context.get_builtin_type(BuiltinType::Array, vec![object_type.clone()]);

        if !context.options.is_compile_mode() {
            return Some(context.vasm().set_type(&array_type));
        }

        let object_var = VariableInfo::tmp("object", context.int_type());
        let mut array_vasm = context.vasm()
            .declare_variable(&object_var)
            .call_static_method(&array_type, NEW_METHOD_NAME, &[], vec![], context);

        let fields = object_type.get_all_fields();
        let (keys, values) = parse_tsv(&content);

        for data in values {
            let mut init_values = HashMap::new();

            for (i, key) in keys.iter().enumerate() {
                if let Some(value) = data.get(i) {
                    init_values.insert(key.as_str(), *value);
                } else {
                    init_values.insert(key.as_str(), "");
                }
            }

            let mut object_creation_vasm = context.vasm()
                .call_static_method(&object_type, OBJECT_CREATE_METHOD_NAME, &[], vec![], context)
                .set_tmp_var(&object_var);

            for field in &fields {
                let init_vasm = match init_values.get(field.name.as_str()) {
                    Some(string) => {
                        if field.ty.is_int() {
                            match i32::from_str_radix(string, 10) {
                                Ok(value) => Some(context.vasm().int(value)),
                                Err(_) => match string.is_empty() {
                                    true => Some(context.vasm().int(DEFAULT_INT_VALUE)),
                                    false => None,
                                },
                            }
                        } else if field.ty.is_float() {
                            match string.parse::<f32>() {
                                Ok(value) => Some(context.vasm().float(value)),
                                Err(_) => match string.is_empty() {
                                    true => Some(context.vasm().float(DEFAULT_FLOAT_VALUE)),
                                    false => None,
                                },
                            }
                        } else if field.ty.is_bool() {
                            let s = string.to_ascii_lowercase();

                            match s.as_str() {
                                "true" | "yes" => Some(context.vasm().int(1i32)),
                                "" | "false" | "no" => Some(context.vasm().int(0i32)),
                                _ => None
                            }
                        } else if field.ty.is_string() {
                            // TODO: should probably not be unchecked
                            Some(make_string_value_from_literal_unchecked(string, context))
                        } else {
                            None
                        }
                    },
                    None => {
                        if field.is_required {
                            context.errors.generic(self, format!("missing field `{}`", field.name.as_str()));
                        }

                        Some(field.get_default_vasm(&object_var, context))
                    },
                };

                if let Some(vasm) = init_vasm {
                    object_creation_vasm = object_creation_vasm
                        .get_tmp_var(&object_var)
                        .set_field(&field.ty, field.offset, vasm);
                } else if let Some(string) = init_values.get(field.name.as_str()) {
                    context.errors.generic(self, format!("cannot convert \"{}\" to `{}`", string, &field.ty));
                }
            }

            object_creation_vasm = object_creation_vasm
                .get_tmp_var(&object_var)
                .set_type(&object_type);

            array_vasm = array_vasm
                .call_regular_method(&array_type, PUSH_METHOD_NAME, &[], vec![object_creation_vasm], context);
        }

        array_vasm = array_vasm
            .set_type(&array_type);

        Some(array_vasm)
    }
}

fn parse_tsv(content: &str) -> (Vec<String>, Vec<Vec<&str>>) {
    let mut lines = content.split("\n");
    let keys : Vec<String> = lines.next().unwrap().split("\t")
        .map(|string| string.to_ascii_lowercase())
        .map(|string| string.replace(" ", "_"))
        .collect();
    
    let values = lines.map(|line| line.split("\t").collect()).collect();

    (keys, values)
}