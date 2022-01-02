use std::ptr::NonNull;
use parsable::{DataLocation, Parsable, parsable};
use crate::{program::{BuiltinType, ProgramContext, VI, Vasm}, wat};

#[parsable]
pub struct VarPrefixWrapper {
    pub value: VarPrefix
}

#[parsable(impl_display=true)]
#[derive(PartialEq, Clone, Copy)]
pub enum VarPrefix {
    // This = "#",
    // Payload = "$",
    System = "@"
}

impl VarPrefixWrapper {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        match &self.value {
            // VarPrefix::This => match &context.current_function {
            //     Some(function_blueprint) => match &function_blueprint.borrow().this_arg {
            //         Some(this_var) => {
            //             result = Some(Vasm::new(this_var.ty.clone(), vec![], vec![VI::get(this_var)]));
            //         },
            //         None => context.errors.add(self, "no `this` value can be referenced in this context")
            //     }
            //     None => context.errors.add(self, "no `this` value can be referenced in this context")
            // },
            // VarPrefix::Payload => match &context.current_function {
            //     Some(function_blueprint) => match &function_blueprint.borrow().payload_arg {
            //         Some(payload_var) => {
            //             result = Some(Vasm::new(payload_var.ty.clone(), vec![], vec![VI::get(payload_var)]));
            //         },
            //         None => context.errors.add(self, "no `payload` value can be referenced in this context")
            //     }
            //     None => context.errors.add(self, "no `payload` value can be referenced in this context")
            // },
            VarPrefix::System => {
                result = Some(context.vasm()
                    .set_type(context.get_builtin_type(BuiltinType::System, vec![]))
                )
            },
        };

        result
    }
}