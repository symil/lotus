use std::{any::Any, collections::HashMap, rc::Rc};

pub struct ReadBuffer<'a> {
    bytes: &'a [u8],
    index: usize,
    objects: HashMap<usize, Rc<dyn Any>>
}

impl<'a> ReadBuffer<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            index: 0,
            objects: HashMap::new()
        }
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn read_byte(&mut self) -> u8 {
        if self.index == self.bytes.len() {
            0
        } else {
            self.index += 1;
            self.bytes[self.index - 1]
        }
    }

    pub fn read(&mut self, length: usize) -> Option<&[u8]> {
        if self.index + length > self.bytes.len() {
            None
        } else {
            let start = self.index;
            let end = self.index + length;

            self.index = end;

            Some(&self.bytes[start..end])
        }
    }

    #[allow(unused)]
    pub fn read_unchecked(&mut self, length: usize) -> &[u8] {
        let start = self.index;
        let end = self.index + length;

        self.index = end;

        &self.bytes[start..end]
    }

    // unused, keeping it just in case
    pub fn register<T : 'static>(&mut self, addr: usize, value: T) -> Rc<T> {
        let wrapped = Rc::new(value);
        let clone = Rc::clone(&wrapped);

        self.objects.insert(addr, clone);

        wrapped
    }

    // unused, keeping it just in case
    pub fn retrieve<T : 'static>(&mut self, addr: usize) -> Option<Rc<T>> {
        match self.objects.get(&addr) {
            Some(value) => {
                match value.downcast_ref::<Rc<T>>() {
                    Some(value) => Some(Rc::clone(value)),
                    None => None
                }
            },
            None => None
        }
    }
}