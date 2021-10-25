use colored::Colorize;
use parsable::parsable;
use crate::{program::{BuiltinInterface, BuiltinType, IS_NONE_FUNC_NAME, ProgramContext, VI, Vasm}, vasm, wat};
use super::{Expression, Statement, StatementList};

#[parsable]
pub struct Branch {
    #[parsable(set_marker="no-object")]
    pub condition: Expression,
    pub statements: StatementList
}

impl Branch {
    pub fn process_condition(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        if let Some(condition_vasm) = self.condition.process(None, context) {
            if condition_vasm.ty.is_bool() {
                result = Some(condition_vasm);
            } else if let Some(option_type) = condition_vasm.ty.get_builtin_type_parameter(BuiltinType::Option) {
                let convert_vasm = Vasm::new(context.bool_type(), vec![], vec![
                    VI::call_regular_method(option_type, IS_NONE_FUNC_NAME, &[], vec![], context),
                    VI::raw(wat!["i32.eqz"])
                ]);

                result = Some(vasm![condition_vasm, convert_vasm]);
            } else {
                context.errors.add(&self.condition, format!("expected `{}` or `{}`, got `{}`", BuiltinType::Bool.get_name(), "Option<_>".bold(), &condition_vasm.ty));
            }
        }

        result
    }

    pub fn process_body(&self, context: &mut ProgramContext) -> Option<Vasm> {
        self.statements.process(context)
    }
}