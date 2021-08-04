use crate::{wat, merge};
use super::{MemoryStack, Wat, ToWat};

pub struct Memory {
    stack: MemoryStack
}

impl Memory {
    pub fn new() -> Self {
        let mut stack = MemoryStack::new(100, 3);

        MemoryStack::assemble(&mut[&mut stack]);

        Self { stack }
    }

    pub fn get_header(&self) -> Vec<Wat> {
        merge!(
            vec![wat!["memory", wat!["export", Wat::string("memory")], 100]],
            self.stack.get_header()
        )
    }
}