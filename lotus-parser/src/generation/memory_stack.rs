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
        let item_size = self.item_field_count * ATOMIC_VALUE_SIZE;
        let stack_start = self.offset;
        let stack_end = self.offset + (self.item_count * ATOMIC_VALUE_SIZE);
        let item_pool_start = stack_end;
        let item_pool_end = item_pool_start + (self.item_count * self.item_field_count * ATOMIC_VALUE_SIZE);

        vec![
            Wat::global_i32(&self.next_addr_ptr_global_name, stack_start as i32),
            Wat::function(&self.init_func_name, None, vec![], None, vec![
                Wat::declare_i32_local("stack_index"),
                Wat::declare_i32_local("pointed_addr"),
                
                Wat::set_local("stack_index", Wat::const_i32(stack_start)),
                Wat::set_local("pointed_addr", Wat::const_i32(item_pool_start)),

                wat!["block", wat![
                    "loop",
                        wat!["br_if", 1, wat![
                            "i32.eq", Wat::get_local("stack_index"), Wat::const_i32(stack_end)
                        ],

                        Wat::set_i32_at_addr(Wat::get_local("stack_index"), Wat::get_local("pointed_addr")),
                        Wat::increment_i32_local("stack_index", ATOMIC_VALUE_SIZE),
                        Wat::increment_i32_local("pointed_addr", item_size),

                        wat!["br", 0]
                    ]
                ]],
            ])
        ]
    }

    pub fn get_total_size(&self) -> usize {
        // There are `item_capacity` items, + `item_capacity` pointers
        ((self.item_count * self.item_field_count) + self.item_count) * ATOMIC_VALUE_SIZE
    }

    pub fn assemble(memory_stacks: &mut[&mut Self]) {
        let mut offset = 0;
        let stride = memory_stacks.iter().fold(0, |acc, mem| acc + mem.get_total_size());

        for (index, mem) in memory_stacks.iter_mut().enumerate() {
            mem.index = index;
            mem.offset = offset;
            mem.stride = stride;
            mem.next_addr_ptr_global_name = format!("mem_{}_next_addr_ptr", index);
            mem.init_func_name = format!("mem_{}_init", index);
            mem.alloc_func_name = format!("mem_{}_alloc", index);
            mem.free_func_name = format!("mem_{}_free", index);

            offset += mem.get_total_size();
        }
    }
}