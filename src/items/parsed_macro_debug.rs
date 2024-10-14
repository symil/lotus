use parsable::parsable;
use crate::program::{ProgramContext, Vasm};
use super::ParsedExpression;

#[parsable]
pub struct ParsedMacroDebug {
    #[parsable(prefix="#")]
    pub token: ParsedMacroDebugToken,
    #[parsable(brackets="()")]
    pub expression: Option<ParsedExpression>
}

#[parsable]
pub enum ParsedMacroDebugToken {
    DebugType = "DEBUG_TYPE",
}

impl ParsedMacroDebug {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        match &self.token {
            ParsedMacroDebugToken::DebugType => match &self.expression {
                Some(expression) => match expression.process(None, context) {
                    Some(vasm) => {
                        println!("{}", vasm.ty.to_string());
                    },
                    None => {},
                },
                None => {
                    context.errors.expected_expression(self);
                },
            },
        };

        Some(context.vasm().set_void(context))
    }
}