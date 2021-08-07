use parsable::DataLocation;

#[derive(Debug, Clone, Copy)]
pub enum AccessType<'a> {
    Get,
    Set(&'a DataLocation)
}