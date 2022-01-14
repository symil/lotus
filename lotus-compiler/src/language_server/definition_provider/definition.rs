use parsable::DataLocation;

pub struct Definition {
    pub target_location: DataLocation
}

impl Definition {
    pub fn new(target_location: &DataLocation) -> Self {
        Self {
            target_location: target_location.clone(),
        }
    }
}