use parsable::parsable;
use crate::{program::{BuiltinType, ProgramContext, TO_STRING_METHOD_NAME, Vasm}};
use super::Expression;

#[parsable]
pub struct TemplateStringFragmentExpression {
    #[parsable(prefix="${", suffix="}", consume_spaces_after_suffix=false)]
    pub expression: Expression
}

impl TemplateStringFragmentExpression {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        match self.expression.process(None, context) {
            Some(vasm) => {
                let ty = vasm.ty.clone();

                Some(context.vasm()
                    .append(vasm)
                    .call_regular_method(&ty, TO_STRING_METHOD_NAME, &[], vec![], context)
                    .set_type(context.get_builtin_type(BuiltinType::String, vec![]))
                )
            },
            None => None,
        }
    }
}