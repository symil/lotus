use std::{rc::Rc, ops::Deref, fmt::Debug};

pub struct Wrapper<T> {
    rc: Rc<T>
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Self {
            rc: Rc::new(value)
        }
    }

    pub fn content(&self) -> &T {
        &self.rc
    }

    // pub fn get_mut(&mut self) -> &mut T {
    //     Rc::get_mut(&mut self.rc).unwrap()
    // }
}

impl<T> Clone for Wrapper<T> {
    fn clone(&self) -> Self {
        Self { rc: Rc::clone(&self.rc) }
    }
}

impl<T> Deref for Wrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        Rc::as_ref(&self.rc)
    }
}

impl<T : Debug> Debug for Wrapper<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", Rc::as_ref(&self.rc))
    }
}