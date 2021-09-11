use std::{cell::{Ref, RefCell, RefMut}, fmt::Debug, rc::Rc};

pub struct Link<T> {
    value: Rc<RefCell<T>>
}

impl<T> Link<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Rc::new(RefCell::new(value))
        }
    }

    pub fn borrow(&self) -> Ref<T> {
        Rc::as_ref(&self.value).borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        Rc::as_ref(&self.value).borrow_mut()
    }
}

impl<T> Clone for Link<T> {
    fn clone(&self) -> Self {
        Self {
            value: Rc::clone(&self.value)
        }
    }
}

impl<T> Debug for Link<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<Link>")
    }
}

impl<T> PartialEq for Link<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value.as_ptr() == other.value.as_ptr()
    }
}