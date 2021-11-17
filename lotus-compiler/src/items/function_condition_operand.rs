use parsable::parsable;
use crate::{program::{ProgramContext, Side, TypeBlueprint}, utils::Link};
use super::{Identifier, VarPrefix};

#[parsable]
pub struct FunctionConditionOperand {
    pub prefix: Option<VarPrefix>,
    pub field_name: Identifier
}

impl FunctionConditionOperand {
    pub fn process(&self, side: Side, event_type_blueprint: &Link<TypeBlueprint>, context: &mut ProgramContext) -> Option<Identifier> {
        let mut result = None;
        // let (required_prefix, target_type_blueprint) = match side {
        //     Side::Left => (VarPrefix::Payload, event_type_blueprint),
        //     Side::Right => (VarPrefix::This, context.current_type.as_ref().unwrap()),
        // };

        // if !self.has_prefix(&required_prefix) {
        //     context.errors.add(&self, format!("{}-hand side of event callback condition must be prefixed by '{}'", side, required_prefix));
        // }

        // if let Some(field) = target_type_blueprint.borrow().fields.get(self.field_name.as_str()) {
        //     result = Some(self.field_name.clone());
        // } else {
        //     context.errors.add(&self.field_name, format!("type `{}` does not have a `{}` field", &target_type_blueprint.borrow().name, &self.field_name));
        // }

        result
    }

    fn has_prefix(&self, prefix: &VarPrefix) -> bool {
        match &self.prefix {
            Some(self_prefix) => self_prefix == prefix,
            _ => false
        }
    }
}