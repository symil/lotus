use parsable::parsable;

#[parsable(located)]
#[derive(Debug)]
pub struct Boolean {
    pub value: bool
}

impl std::ops::Deref for Boolean {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for Boolean {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }    
}