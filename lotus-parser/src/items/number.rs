use parsable::parsable;

#[parsable(located, name="number")]
pub struct Number {
    pub value: f64,
}

impl std::ops::Deref for Number {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for Number {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }    
}