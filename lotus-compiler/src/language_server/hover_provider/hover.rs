use parsable::DataLocation;
use crate::program::Type;

pub struct Hover {
    pub location: DataLocation,
    pub ty: Option<Type>
}

impl Hover {
    pub fn new(location: &DataLocation) -> Self {
        Self {
            location: location.clone(),
            ty: None,
        }
    }
}