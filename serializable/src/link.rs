use std::{cell::{Ref, RefCell, RefMut}, fmt::Debug, rc::Rc};

use crate::{ReadBuffer, Serializable, WriteBuffer};

pub struct Link<T> {
    original: bool, 
    value: Rc<RefCell<Option<T>>>
}

impl<T> Link<T> {
    pub fn empty() -> Self {
        Self { original: true, value: Rc::new(RefCell::new(None)) }
    }

    pub fn new(value: T) -> Self {
        Self { original: true, value: Rc::new(RefCell::new(Some(value))) }
    }

    pub fn set(&self, value: T) {
        *self.value.borrow_mut() = Some(value);
    }

    pub fn get_addr(&self) -> usize {
        Rc::as_ptr(&self.value) as usize
    }

    pub fn clone(&self) -> Self {
        Self { original: false, value: Rc::clone(&self.value) }
    }

    pub fn is_empty(&self) -> bool {
        self.value.borrow().is_none()
    }

    pub fn borrow(&self) -> Ref<T> {
        Ref::map(self.value.borrow(), |option| option.as_ref().unwrap())
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        RefMut::map(self.value.borrow_mut(), |option| option.as_mut().unwrap())
    }

    pub fn with_ref<F : FnOnce(RefMut<T>)>(&self, f: F) {
        let ref_mut = self.borrow_mut();

        f(ref_mut)
    }
}

impl<T> Default for Link<T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<T : Debug> Debug for Link<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.original {
            fmt.debug_struct("Link")
                .field("addr", &format_args!("0x{:X}", self.get_addr()))
                .field("value", &format_args!("{:?}", self.value.borrow()))
                .finish()
        } else {
            fmt.write_fmt(format_args!("Link [0x{:X}]", self.get_addr()))
        }
    }
}

impl<T : Serializable + 'static> Serializable for Link<T> {
    fn write_bytes(input: &Self, buffer: &mut WriteBuffer) {
        match input.value.borrow().as_ref() {
            Some(value) => {
                let addr = input.get_addr();

                usize::write_bytes(&addr, buffer);

                if buffer.register(addr) {
                    T::write_bytes(value, buffer)
                }
            },
            None => {
                usize::write_bytes(&0, buffer);
            }
        }
    }

    fn read_bytes(buffer: &mut ReadBuffer) -> Option<Self> {
        let addr = usize::read_bytes(buffer)?;

        if addr == 0 {
            None
        } else {
            match buffer.retrieve::<Self>(addr) {
                Some(link) => Some(Link::clone(&link)),
                None => {
                    let link = buffer.register(addr, Self::empty());
                    let value = T::read_bytes(buffer)?;
                    let mut result = Link::clone(&link);

                    result.set(value);
                    result.original = true;

                    Some(result)
                }
            }
        }
    }
}