use std::cell::UnsafeCell;
use serializable::Serializable;

#[derive(Serializable)]
pub struct ValueWrapper<T : Serializable> {
    data: UnsafeCell<T>
}

impl<T : Serializable> ValueWrapper<T> {
    pub fn new(data: T) -> Self {
        Self { data: UnsafeCell::new(data) }
    }

    pub fn get(&self) -> &T {
        unsafe { self.data.get().as_ref().unwrap() }
    }

    pub fn set(&self, data: T) {
        unsafe { *self.data.get().as_mut().unwrap() = data; }
    }
}