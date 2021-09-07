use parsable::parsable;
use crate::program::{ProgramContext, Side, TypeOld};
use super::{Identifier, VarRefPrefix};

#[parsable]
pub struct FunctionConditionOperand {
    pub prefix: Option<VarRefPrefix>,
    pub field_name: Identifier
}

impl FunctionConditionOperand {
    pub fn process(&self, side: Side, event_type_id: u64, context: &mut ProgramContext) -> Option<String> {
        let mut result = None;
        let (required_prefix, target_type_id) = match side {
            Side::Left => (VarRefPrefix::Payload, event_type_id),
            Side::Right => (VarRefPrefix::This, context.current_type.unwrap()),
        };
        let target_type = context.types.get_by_id(target_type_id).unwrap();

        if !self.has_prefix(&required_prefix) {
            context.errors.add(&self, format!("{}-hand side of event callback condition must be prefixed by '{}'", side, required_prefix));
        }

        if let Some(field) = target_type.fields.get(self.field_name.as_str()) {
            result = Some(self.field_name.to_string());
        } else {
            context.errors.add(&self.field_name, format!("type `{}` does not have a `{}` field", &target_type.name, &self.field_name));
        }

        result
    }

    fn has_prefix(&self, prefix: &VarRefPrefix) -> bool {
        match &self.prefix {
            Some(self_prefix) => self_prefix == prefix,
            _ => false
        }
    }
}