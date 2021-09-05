use crate::{EntityField, NumericalField};

pub struct Entity {
    pub kind: u16,
    pub numerical_fields: Vec<NumericalField>,
    pub entity_fields: Vec<EntityField>
}

impl Entity {
    pub fn new(kind: u16, numerical_field_count: usize, entity_field_count: usize) -> Self {
        let mut numerical_fields = Vec::with_capacity(numerical_field_count);
        let mut entity_fields = Vec::with_capacity(entity_field_count);

        numerical_fields.resize_with(numerical_field_count, NumericalField::new);
        entity_fields.resize_with(numerical_field_count, EntityField::new);

        Self { kind, numerical_fields, entity_fields }
    }
}