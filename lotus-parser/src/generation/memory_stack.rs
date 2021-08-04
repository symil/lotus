use crate::merge;

use super::{ToWat, Wat, wat};

const ATOMIC_VALUE_SIZE : usize = 4;
const PAGE_SIZE : usize = 2usize.pow(30); // 1 MB, 64MB would be 2**36

#[derive(Default)]
pub struct MemoryStack {
    item_count: usize,
    item_field_count: usize,

    index: usize,
    offset: usize,
    stride: usize,
    item_size: usize,
    stack_start: usize,
    stack_end: usize,
    item_pool_start: usize,
    item_pool_end: usize,
    init_func_name: String,
    alloc_func_name: String,
    free_func_name: String,
    next_addr_ptr_global_name: String
}

impl MemoryStack {
    pub fn new(item_count: usize, item_field_count: usize) -> Self {
        let mut stack = Self::default();

        stack.item_count = item_count;
        stack.item_field_count = item_field_count;

        stack
    }

    pub fn get_header(&self) -> Vec<Wat> {
        vec![
            Wat::global_i32(&self.next_addr_ptr_global_name, self.stack_start),
            self.get_init_function(),
            self.get_alloc_function()
        ]
    }

    fn get_init_function(&self) -> Wat {
        Wat::function(&self.init_func_name, None, vec![], None, vec![
            Wat::declare_i32_local("stack_index"),
            Wat::declare_i32_local("pointed_addr"),

            Wat::set_local("stack_index", Wat::const_i32(self.stack_start)),
            Wat::set_local("pointed_addr", Wat::const_i32(self.item_pool_start)),

            Wat::while_loop("stack_index", Wat::const_i32(self.stack_end), ATOMIC_VALUE_SIZE, vec![
                Wat::set_i32_at_addr(Wat::get_local("stack_index"), Wat::get_local("pointed_addr")),
                Wat::increment_i32_local("pointed_addr", self.item_size),
            ]),
        ])
    }

    fn get_alloc_function(&self) -> Wat {
        Wat::function(&self.alloc_func_name, None, vec![], Some("i32"), vec![
            Wat::const_i32(0)
        ])
    }

    pub fn get_total_size(&self) -> usize {
        // There are `item_capacity` items, + `item_capacity` pointers
        ((self.item_count * self.item_field_count) + self.item_count) * ATOMIC_VALUE_SIZE
    }

    pub fn assemble(memory_stacks: &mut[&mut Self], start_offset: usize) -> usize {
        let mut offset = start_offset;
        let stride = memory_stacks.iter().fold(0, |acc, mem| acc + mem.get_total_size());

        for (index, mem) in memory_stacks.iter_mut().enumerate() {
            mem.index = index;
            mem.offset = offset;
            mem.stride = stride;
            mem.item_size = mem.item_field_count * ATOMIC_VALUE_SIZE;
            mem.stack_start = offset;
            mem.stack_end = offset + (mem.item_count * ATOMIC_VALUE_SIZE);
            mem.item_pool_start = mem.stack_end;
            mem.item_pool_end = mem.item_pool_start + (mem.item_count * mem.item_field_count * ATOMIC_VALUE_SIZE);
            mem.next_addr_ptr_global_name = format!("mem_{}_next_addr_ptr", index);
            mem.init_func_name = format!("mem_{}_init", index);
            mem.alloc_func_name = format!("mem_{}_alloc", index);
            mem.free_func_name = format!("mem_{}_free", index);

            offset += mem.get_total_size();
        }

        stride
    }
}