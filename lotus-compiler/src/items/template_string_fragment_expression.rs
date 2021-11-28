use parsable::parsable;
use crate::{program::{BuiltinType, ProgramContext, TO_STRING_METHKD_NAME, VI, Vasm}, vasm};
use super::Expression;

#[parsable]
pub struct TemplateStringFragmentExpression {
    #[parsable(prefix="${", suffix="}")]
    pub expression: Expression
}

impl TemplateStringFragmentExpression {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        match self.expression.process(None, context) {
            Some(vasm) => {
                let to_string_instruction = VI::call_regular_method(&vasm.ty, TO_STRING_METHKD_NAME, &[], vec![], context);

                Some(Vasm::new(context.get_builtin_type(BuiltinType::String, vec![]), vec![], vasm![
                    vasm,
                    to_string_instruction
                ]))
            },
            None => None,
        }
    }
}