use parsable::parsable;
use crate::{program::{BuiltinType, FunctionCall, NamedFunctionCallDetails, ProgramContext, VI, Vasm}, vasm};
use super::{TemplateStringFragment, make_string_value_from_literal};

#[parsable]
pub struct TemplateString {
    #[parsable(brackets="``", consume_spaces_after_prefix=false, consume_spaces_between_items=false)]
    pub fragments: Vec<TemplateStringFragment>
}

impl TemplateString {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        match self.fragments.len() {
            0 => make_string_value_from_literal(None, "", context),
            1 => self.fragments.first().unwrap().process(context),
            _ => {
                let string_type = context.get_builtin_type(BuiltinType::String, vec![]);
                let string_array_type = context.get_builtin_type(BuiltinType::Array, vec![string_type.clone()]);
                let mut result = vasm![
                    VI::call_static_method(&string_array_type, "with_capacity", &[], VI::int(self.fragments.len()), context)
                ];

                for fragment in &self.fragments {
                    if let Some(vasm) = fragment.process(context) {
                        result.extend(
                            VI::call_regular_method(&string_array_type, "push", &[], vasm, context)
                        );
                    }
                }

                Some(Vasm::new(string_type, vec![], vec![
                    VI::call_function(FunctionCall::Named(NamedFunctionCallDetails {
                        caller_type: None,
                        function: context.functions.get_by_name("join_strings").unwrap(),
                        parameters: vec![]
                    }), result)
                ]))
            }
        }
    }
}