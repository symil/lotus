use colored::Colorize;
use parsable::{DataLocation, parsable};
use crate::{program::{BuiltinInterface, BuiltinType, CompilationError, IS_NONE_METHOD_NAME, ProgramContext, Type, VI, Vasm}, vasm, wat};
use super::{Expression, BlockExpression};

#[parsable]
pub struct Branch {
    #[parsable(set_marker="no-object")]
    pub condition: Expression,
    pub body: BlockExpression
}

impl Branch {
    pub fn process_condition(&self, context: &mut ProgramContext) -> Option<Vasm> {
        match self.condition.process(None, context) {
            Some(condition_vasm) => convert_to_bool(&self.condition, condition_vasm, context),
            None => None,
        }
    }

    pub fn process_body(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        self.body.process(type_hint, context)
    }
}

pub fn convert_to_bool(location: &DataLocation, vasm: Vasm, context: &mut ProgramContext) -> Option<Vasm> {
    if vasm.ty.is_void() {
        context.errors.unexpected_void_expression(location);
        None
    } else if vasm.ty.is_bool() {
        Some(vasm)
    } else if !vasm.ty.is_undefined() {
        let convert_vasm = Vasm::new(context.bool_type(), vec![], vec![
            VI::call_regular_method(&vasm.ty, IS_NONE_METHOD_NAME, &[], vec![], context),
            VI::raw(wat!["i32.eqz"])
        ]);

        Some(vasm![vasm, convert_vasm])
    } else {
        None
    }
}