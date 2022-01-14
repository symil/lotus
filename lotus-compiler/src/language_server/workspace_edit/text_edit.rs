use parsable::DataLocation;

pub struct TextEdit {
    pub deleted_location: DataLocation,
    pub inserted_text: String
}

impl TextEdit {
    pub fn new(deleted_location: DataLocation, inserted_text: String) -> Self {
        Self {
            deleted_location,
            inserted_text,
        }
    }
}