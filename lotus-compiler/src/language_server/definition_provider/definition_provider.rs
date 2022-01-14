use parsable::DataLocation;

use crate::program::Cursor;
use super::Definition;

pub struct DefinitionProvider {
    pub cursor: Cursor,
    pub definition: Option<Definition>
}

impl DefinitionProvider {
    pub fn new(cursor: &Cursor) -> Self {
        Self {
            cursor: cursor.clone(),
            definition: None,
        }
    }

    pub fn set_definition(&mut self, location: &DataLocation, definition: &DataLocation) {
        if !self.cursor.is_on_location(location) {
            return;
        }

        self.definition = Some(Definition::new(definition));
    }

    pub fn get_definition(&self) -> Option<&Definition> {
        self.definition.as_ref()
    }
}