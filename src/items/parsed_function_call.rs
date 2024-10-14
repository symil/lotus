use parsable::parsable;
use crate::program::{AccessType, ProgramContext, Type, TypeContent, Vasm, FunctionCall, AnonymousFunctionCallDetails};
use super::{ParsedOpeningRoundBracket, ParsedArgumentList, process_function_call};

#[parsable]
pub struct ParsedFunctionCall {
    pub arguments: ParsedArgumentList
}

impl ParsedFunctionCall {
    pub fn process(&self, parent_type: &Type, type_hint: Option<&Type>, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
        let signature = match parent_type.content() {
            TypeContent::Function(signature) => signature.clone(),
            _ => {
                context.errors.generic(self, format!("type `{}` is not a function", parent_type));
                return None;
            }
        };

        let function_call = FunctionCall::Anonymous(AnonymousFunctionCallDetails {
            signature,
            function_offset: 0,
        });

        process_function_call(None, function_call, &self.arguments, type_hint, access_type, context)
    }
}