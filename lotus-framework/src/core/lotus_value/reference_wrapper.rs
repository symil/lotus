use std::{cell::UnsafeCell, rc::Rc};
use serializable::Serializable;

pub struct ReferenceWrapper<T : Serializable + Default> {
    data: Rc<UnsafeCell<T>>
}

impl<T : Serializable + Default> ReferenceWrapper<T> {
    pub fn new(data: T) -> Self {
        Self { data: Rc::new(UnsafeCell::new(data)) }
    }

    pub fn get(&self) -> &T {
        // TODO: `unwrap_unchecked`
        unsafe { self.data.get().as_ref().unwrap() }
    }

    pub fn get_mut(&self) -> &mut T {
        // TODO: `unwrap_unchecked`
        unsafe { self.data.get().as_mut().unwrap() }
    }

    pub fn set_outer(&mut self, reference: &ReferenceWrapper<T>) {
        self.data = Rc::clone(&reference.data)
    }

    pub fn get_addr(&self) -> usize {
        Rc::as_ptr(&self.data) as usize
    }
}

impl<T : Serializable + Default> Clone for ReferenceWrapper<T> {
    fn clone(&self) -> Self {
        Self { data: Rc::clone(&self.data) }
    }
}

impl<T : Serializable + Default + 'static> Serializable for ReferenceWrapper<T> {
    fn write_bytes(value: &Self, buffer: &mut serializable::WriteBuffer) {
        let addr = Rc::as_ptr(&value.data) as usize;

        usize::write_bytes(&addr, buffer);

        if buffer.register(addr) {
            <UnsafeCell<T>>::write_bytes(&value.data, buffer)
        }
    }

    fn read_bytes(buffer: &mut serializable::ReadBuffer) -> Option<Self> {
        let addr = usize::read_bytes(buffer)?;

        match buffer.retrieve::<Self>(addr) {
            Some(rc_result) => Some(Rc::as_ref(&rc_result).clone()),
            None => {
                let rc_result = buffer.register(addr, Self::new(T::default()));
                let value = T::read_bytes(buffer)?;
                let result = Rc::as_ref(&rc_result).clone();

                *result.get_mut() = value;

                Some(result)
            }
        }
    }
}