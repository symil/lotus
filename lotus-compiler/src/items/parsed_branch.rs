use colored::Colorize;
use parsable::{ItemLocation, parsable};
use crate::{program::{BuiltinInterface, BuiltinType, CompilationError, IS_NONE_METHOD_NAME, ProgramContext, Type, Vasm}, wat};
use super::{ParsedExpression, ParsedBlockExpression};

#[parsable]
pub struct ParsedBranch {
    #[parsable(set_marker="no-object")]
    pub condition: ParsedExpression,
    pub body: Option<ParsedBlockExpression>
}

impl ParsedBranch {
    pub fn process_condition(&self, context: &mut ProgramContext) -> Option<Vasm> {
        match self.condition.process(None, context) {
            Some(condition_vasm) => convert_to_bool(&self.condition, condition_vasm, context),
            None => None,
        }
    }

    pub fn process_body(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let body = match &self.body {
            Some(body) => body,
            None => {
                context.errors.expected_block(self);
                return None;
            }
        };

        body.process(type_hint, context)
    }
}

pub fn convert_to_bool(location: &ItemLocation, vasm: Vasm, context: &mut ProgramContext) -> Option<Vasm> {
    if vasm.ty.is_void() {
        context.errors.unexpected_void_expression(location);
        None
    } else if vasm.ty.is_bool() {
        Some(vasm)
    } else if !vasm.ty.is_undefined() {
        let ty = vasm.ty.clone();

        let result = context.vasm()
            .append(vasm)
            .call_regular_method(&ty, IS_NONE_METHOD_NAME, &[], vec![], context)
            .eqz()
            .set_type(context.bool_type());

        Some(result)
    } else {
        None
    }
}