use crate::{EntityField, NumericalField};

pub struct Entity {
    pub numerical_fields: Vec<NumericalField>,
    pub link_fields: Vec<EntityField>
}