use parsable::DataLocation;

#[derive(Debug, Clone, Copy)]
pub enum AccessType<'a> {
    Get,
    Set(&'a DataLocation)
}

impl<'a> AccessType<'a> {
    pub fn is_set(&self) -> bool {
        match self {
            AccessType::Get => false,
            AccessType::Set(_) => true,
        }
    }
}