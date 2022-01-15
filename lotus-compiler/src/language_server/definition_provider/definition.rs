use parsable::ItemLocation;

pub struct Definition {
    pub target_location: ItemLocation
}

impl Definition {
    pub fn new(target_location: &ItemLocation) -> Self {
        Self {
            target_location: target_location.clone(),
        }
    }
}