#[derive(Debug)]
pub enum FlexRef<'a, T> {
    Const(&'a T),
    Mut(&'a mut T)
}

impl<'a, T> FlexRef<'a, T> {
    pub fn as_ref(&self) -> &T {
        match self {
            FlexRef::Const(value) => *value,
            FlexRef::Mut(value) => *value,
        }
    }
}