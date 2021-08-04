use crate::{wat, merge};
use super::{MemoryStack, Wat, ToWat, ToWatVec};

pub struct Memory {
    stack: MemoryStack
}

static INIT_MEMORY_FUNC_NAME : &'static str = "init_mem";

impl Memory {
    pub fn new() -> Self {
        let mut stack = MemoryStack::new(100, 3);

        MemoryStack::assemble(&mut[&mut stack], 0);

        Self { stack }
    }

    fn get_init_function(&self) -> Wat {
        Wat::declare_function(INIT_MEMORY_FUNC_NAME, None, vec![], None, vec![
            self.stack.init()
        ])
    }

    pub fn get_header(&self) -> Vec<Wat> {
        merge![
            wat!["memory", Wat::export("memory"), 100],
            self.get_init_function(),
            self.stack.get_header()
        ]
    }

    pub fn init(&self) -> Wat {
        Wat::call(INIT_MEMORY_FUNC_NAME, vec![])
    }

    pub fn alloc(&self) -> Wat {
        self.stack.alloc()
    }
}