pub struct TypeContext {
    pub name: String
    // TODO
}

impl TypeContext {
    pub fn get_name(&self) -> &str {
        &self.name
    }
}