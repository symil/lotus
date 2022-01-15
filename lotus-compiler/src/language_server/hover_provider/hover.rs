use parsable::ItemLocation;
use crate::program::Type;

pub struct Hover {
    pub location: ItemLocation,
    pub ty: Option<Type>
}

impl Hover {
    pub fn new(location: &ItemLocation) -> Self {
        Self {
            location: location.clone(),
            ty: None,
        }
    }
}