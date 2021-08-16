use super::Id;

#[derive(Debug, Clone)]
pub struct StructInfo {
    pub id: Id,
    pub name: String
}

impl StructInfo {
    pub fn new(id: Id, name: String) -> Self {
        Self { id, name }
    }
}

impl PartialEq for StructInfo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}