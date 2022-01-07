use std::collections::HashSet;

use parsable::DataLocation;

pub struct DefinitionArea {
    pub definition: DataLocation,
    pub links: HashSet<DataLocation>
}