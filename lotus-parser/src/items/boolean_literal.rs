use parsable::parsable;

#[parsable(name="boolean")]
pub struct BooleanLiteral {
    pub value: bool
}

impl std::ops::Deref for BooleanLiteral {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for BooleanLiteral {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }    
}