use parsable::parsable;
use crate::{program::{BuiltinType, FunctionCall, NamedFunctionCallDetails, ProgramContext, Vasm}};
use super::{TemplateStringFragment, make_string_value_from_literal_unchecked};

#[parsable]
pub struct TemplateString {
    #[parsable(brackets="``", consume_spaces_after_prefix=false, consume_spaces_between_items=false)]
    pub fragments: Vec<TemplateStringFragment>
}

impl TemplateString {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        match self.fragments.len() {
            0 => Some(make_string_value_from_literal_unchecked("", context)),
            1 => self.fragments.first().unwrap().process(context),
            _ => {
                let string_type = context.get_builtin_type(BuiltinType::String, vec![]);
                let string_array_type = context.get_builtin_type(BuiltinType::Array, vec![string_type.clone()]);
                let mut result = context.vasm()
                    .call_static_method(&string_array_type, "with_capacity", &[], vec![context.vasm().int(self.fragments.len())], context);

                for fragment in &self.fragments {
                    if let Some(vasm) = fragment.process(context) {
                        result = result
                            .call_regular_method(&string_array_type, "push", &[], vec![vasm], context);
                    }
                }

                Some(context.vasm()
                    .call_function_named(None, &context.functions.get_by_name("join_strings").unwrap(), &[], vec![result])
                    .set_type(string_type)
                )
            }
        }
    }
}