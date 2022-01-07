use parsable::DataLocation;
use crate::program::Type;

pub struct HoverArea {
    pub location: DataLocation,
    pub ty: Option<Type>,
    pub definition: Option<DataLocation>
}

impl HoverArea {
    pub fn new(location: &DataLocation) -> Self {
        Self {
            location: location.clone(),
            ty: None,
            definition: None,
        }
    }

    pub fn contains_cursor(&self, file_path: &str, cursor_index: usize) -> bool {
        self.location.contains_cursor(file_path, cursor_index)
    }

    pub fn get_location(&self) -> DataLocation {
        self.location.clone()
    }

    pub fn set_type(&mut self, ty: &Type) {
        self.ty = Some(ty.clone());
    }

    pub fn set_definition(&mut self, definition: &DataLocation) {
        self.definition = Some(definition.clone());
    }

    pub fn get_type(&self) -> Option<Type> {
        self.ty.clone()
    }

    pub fn get_definition(&self) -> Option<DataLocation> {
        self.definition.clone()
    }
}