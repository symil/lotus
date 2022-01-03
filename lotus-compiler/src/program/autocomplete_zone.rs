use parsable::DataLocation;
use super::Type;

pub struct AutoCompleteZone {
    pub location: DataLocation,
    pub details: AutoCompleteDetails
}

pub enum AutoCompleteDetails {
    Field(Type)
}