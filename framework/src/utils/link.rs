use std::{cell::{UnsafeCell}, fmt::Debug, mem, rc::Rc};
use serializable::{ReadBuffer, Serializable, WriteBuffer};

pub struct Link<T : ?Sized> {
    value: Rc<UnsafeCell<Option<Rc<T>>>>
}

impl<T> Link<T> {
    pub fn empty() -> Self {
        Self { value: Rc::new(UnsafeCell::new(None)) }
    }

    pub fn new(value: T) -> Self {
        Self { value: Rc::new(UnsafeCell::new(Some(Rc::new(value)))) }
    }

    pub fn clone(&self) -> Self {
        Self { value: Rc::clone(&self.value) }
    }
    
    fn get_opt_mut(&self) -> &mut Option<Rc<T>> {
        unsafe { mem::transmute(self.value.get()) }
    }

    pub fn set(&self, value: T) {
        *self.get_opt_mut() = Some(Rc::new(value));
    }

    pub fn get_addr(&self) -> usize {
        Rc::as_ptr(&self.value) as usize
    }

    pub fn is_empty(&self) -> bool {
        self.get_opt_mut().is_none()
    }

    // TODO: maybe use `unwrap_unchecked`?
    pub fn borrow(&self) -> &T {
        self.get_opt_mut().as_ref().unwrap()
    }

    pub fn borrow_mut(&self) -> &mut T {
        Rc::get_mut(self.get_opt_mut().as_mut().unwrap()).unwrap()
    }

    pub fn with_ref<F : FnOnce(&mut T)>(&self, f: F) {
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
        fmt.write_fmt(format_args!("Link [0x{:X}]", self.get_addr()))
    }
}

impl<T : Serializable + 'static> Serializable for Link<T> {
    fn write_bytes(input: &Self, buffer: &mut WriteBuffer) {
        match input.get_opt_mut() {
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
                    let result = Link::clone(&link);

                    result.set(value);

                    Some(result)
                }
            }
        }
    }
}