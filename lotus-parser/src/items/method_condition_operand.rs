use parsable::parsable;
use crate::program::{ProgramContext, Side, TypeOld};

use super::{Identifier, VarRefPrefix};

#[parsable]
pub struct MethodConditionOperand {
    pub prefix: Option<VarRefPrefix>,
    pub field_name: Identifier
}

impl MethodConditionOperand {
    pub fn process(&self, struct_name: &Identifier, method_name: &Identifier, side: Side, context: &mut ProgramContext) {
        let (required_prefix, target_struct_name) = match side {
            Side::Left => (VarRefPrefix::Payload, method_name),
            Side::Right => (VarRefPrefix::This, struct_name),
        };

        if !self.has_prefix(&required_prefix) {
            context.errors.add(&self, format!("{}-hand side of event callback condition must be prefixed by '{}'", side, required_prefix));
        }

        if let Some(target_struct) = context.get_struct_by_name(target_struct_name) {
            if let Some(field) = target_struct.fields.get(&self.field_name) {
                let mut ok = false;

                if let TypeOld::Struct(field_struct) = &field.ty {
                    if let Some(field_struct_annotation) = context.get_struct_by_id(field_struct.id) {
                        if field_struct_annotation.qualifier.is_entity_qualifier() {
                            ok = true;
                        }
                    }
                }

                if !ok {
                    context.errors.add(&self.field_name, format!("event callback condition: {}-side must refer to an entity field (`entity`, `world` or `user` qualifier)", side));
                }
            } else {
                context.errors.add(&self.field_name, format!("type `{}` does not have a `{}` field", method_name, &self.field_name));
            }
        }
    }

    fn has_prefix(&self, prefix: &VarRefPrefix) -> bool {
        match &self.prefix {
            Some(self_prefix) => self_prefix == prefix,
            _ => false
        }
    }
}