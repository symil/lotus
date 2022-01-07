use std::{cell::{Ref, RefCell, RefMut}, fmt::Debug, hash::Hash, rc::Rc, borrow::Borrow};

pub struct Link<T> {
    value: Rc<RefCell<T>>
}

impl<T> Link<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Rc::new(RefCell::new(value))
        }
    }

    pub fn get_addr(&self) -> usize {
        Rc::as_ptr(&self.value) as usize
    }

    pub fn borrow(&self) -> Ref<T> {
        Rc::as_ref(&self.value).borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        Rc::as_ref(&self.value).borrow_mut()
    }

    pub fn with_ref<V, F : FnOnce(Ref<T>) -> V>(&self, mut f: F) -> V {
        f(self.borrow())
    }

    pub fn with_mut<V, F : FnOnce(RefMut<T>) -> V>(&self, mut f: F) -> V {
        f(self.borrow_mut())
    }
}

impl<T> Clone for Link<T> {
    fn clone(&self) -> Self {
        Self {
            value: Rc::clone(&self.value)
        }
    }
}

impl<T : Debug> Debug for Link<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.borrow())
    }
}

impl<T> PartialEq for Link<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value.as_ptr() == other.value.as_ptr()
    }
}

impl<T> Hash for Link<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.as_ptr().hash(state)
    }
}

impl<T> Eq for Link<T> {
    
}