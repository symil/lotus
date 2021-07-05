use std::{any::Any, collections::HashMap, rc::Rc};

pub struct ReadBuffer<'a> {
    bytes: &'a [u8],
    cursor: usize,
    objects: HashMap<usize, Rc<dyn Any>>
}

impl<'a> ReadBuffer<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            cursor: 0,
            objects: HashMap::new()
        }
    }

    pub fn read_byte(&mut self) -> u8 {
        if self.cursor == self.bytes.len() {
            0
        } else {
            self.cursor += 1;
            self.bytes[self.cursor - 1]
        }
    }

    pub fn read(&mut self, length: usize) -> Option<&[u8]> {
        if self.cursor + length > self.bytes.len() {
            None
        } else {
            let start = self.cursor;
            let end = self.cursor + length;

            self.cursor = end;

            Some(&self.bytes[start..end])
        }
    }

    pub fn read_unchecked(&mut self, length: usize) -> &[u8] {
        let start = self.cursor;
        let end = self.cursor + length;

        self.cursor = end;

        &self.bytes[start..end]
    }

    pub fn register<T : 'static>(&mut self, addr: usize, value: T) -> Rc<T> {
        let wrapped = Rc::new(value);
        let clone = Rc::clone(&wrapped);

        self.objects.insert(addr, clone);

        wrapped
    }

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