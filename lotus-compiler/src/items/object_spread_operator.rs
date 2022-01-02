use parsable::parsable;
use crate::{items::ObjectInitResult, program::{ProgramContext, Type, VariableInfo, Vasm}};
use super::Expression;

#[parsable]
pub struct SpreadOperator {
    #[parsable(prefix="..")]
    pub expression: Expression
}

impl SpreadOperator {
    pub fn process(&self, object_type: &Type, context: &mut ProgramContext) -> ObjectInitResult {
        let mut object_init_result = ObjectInitResult::default();

        if let Some(vasm) = self.expression.process(None, context) {
            let var_info = VariableInfo::tmp("spread_tmp", vasm.ty.clone());
            let expr_type = vasm.ty.clone();

            object_init_result.init = Some(context.vasm()
                .declare_variable(&var_info)
                .append(vasm)
                .set_tmp_var(&var_info)
            );

            for field in expr_type.get_all_fields() {
                if let Some(object_field) = object_type.get_field(field.name.as_str()) {
                    let actual_type = field.ty.replace_parameters(Some(&expr_type), &[]);
                    let expected_type = object_field.ty.replace_parameters(Some(object_type), &[]);

                    if actual_type.is_assignable_to(&expected_type) {
                        object_init_result.fields.push((
                            field.name.to_string(),
                            context.vasm()
                                .get_tmp_var(&var_info)
                                .get_field(&actual_type, field.offset)
                        ));
                    }
                }
            }
        }

        object_init_result
    }
}