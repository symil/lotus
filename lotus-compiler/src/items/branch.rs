use colored::Colorize;
use parsable::parsable;
use crate::{program::{BuiltinInterface, BuiltinType, IS_NONE_METHOD_NAME, ProgramContext, Type, VI, Vasm}, vasm, wat};
use super::{Expression, BlockExpression};

#[parsable]
pub struct Branch {
    #[parsable(set_marker="no-object")]
    pub condition: Expression,
    pub body: BlockExpression
}

impl Branch {
    pub fn process_condition(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        if let Some(condition_vasm) = self.condition.process(None, context) {
            if condition_vasm.ty.is_void() {
                context.errors.add(&self.condition, format!("expected typed expression"));
            } else if condition_vasm.ty.is_bool() {
                result = Some(condition_vasm);
            } else if !condition_vasm.ty.is_undefined() {
                let convert_vasm = Vasm::new(context.bool_type(), vec![], vec![
                    VI::call_regular_method(&condition_vasm.ty, IS_NONE_METHOD_NAME, &[], vec![], context),
                    VI::raw(wat!["i32.eqz"])
                ]);

                result = Some(vasm![condition_vasm, convert_vasm]);
            }
            // else {
                // context.errors.add(&self.condition, format!("expected `{}` or `{}`, got `{}`", BuiltinType::Bool.get_name(), "Option<_>".bold(), &condition_vasm.ty));
            // }
        }

        result
    }

    pub fn process_body(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        self.body.process(type_hint, context)
    }
}