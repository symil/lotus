use crate::{wat, merge};
use super::{MemoryStack, ToWat, ToWatVec, WasmModule, Wat};

pub struct Memory;

const ATOMIC_VALUE_BYTE_SIZE : usize = 4;
const BASE_ALLOCATED_VALUE_COUNT : usize = 4;

const STACK_METADATA_SIZE : usize = 2;
const STACK_COUNT_PER_BLOCK : usize = 64;
const STACK_BLOCK_COUNT : usize = 8;
const STACK_BYTE_SIZE : usize = 2usize.pow(20); // 20 MiB
const WASM_PAGE_BYTE_SIZE : usize = 2usize.pow(16); // 64 KiB
const ATOMIC_VALUE_COUNT_PER_STACK : usize = STACK_BYTE_SIZE / ATOMIC_VALUE_BYTE_SIZE;

const WASM_PAGE_COUNT_PER_STACK : usize = STACK_BYTE_SIZE / WASM_PAGE_BYTE_SIZE;

const STACKS_METADATA_BYTE_OFFSET : usize = 0;

const STACK_BLOCK_BYTE_SIZE : usize = STACK_METADATA_SIZE * STACK_COUNT_PER_BLOCK * ATOMIC_VALUE_BYTE_SIZE;
const ALL_STACKS_BYTE_SIZE : usize = STACK_BLOCK_BYTE_SIZE * STACK_BLOCK_COUNT;

static INIT_FUNC_NAME : &'static str = "mem_init";
static ALLOC_FUNC_NAME : &'static str = "mem_alloc";
static ALLOC_STACK_FUNC_NAME : &'static str = "mem_alloc_stack";

const EMPTY_STACK_MARKER : i32 = -1;
const UNINITIALIZED_STACK_MARKER : i32 = -2;

impl Memory {
    pub fn new() -> Self {
        Self
    }

    fn get_init_function(&self) -> Wat {
        Wat::declare_function(INIT_FUNC_NAME, None, vec![], None, vec![
            Wat::declare_local_i32("i"),
            Wat::set_local("i", Wat::const_i32(STACKS_METADATA_BYTE_OFFSET)),
            Wat::while_loop(wat!["i32.lt_u", Wat::get_local("i"), Wat::const_i32(STACKS_METADATA_BYTE_OFFSET + ALL_STACKS_BYTE_SIZE)], vec![
                Wat::mem_set_i32("i", Wat::const_i32(UNINITIALIZED_STACK_MARKER)),
                Wat::increment_local_i32("i", ATOMIC_VALUE_BYTE_SIZE * STACK_METADATA_SIZE)
            ])
        ])
    }

    fn get_alloc_stack_function(&self) -> Wat {
        Wat::declare_function(ALLOC_STACK_FUNC_NAME, None, vec![], Some("i32"), vec![
            Wat::declare_local_i32("new_stack_addr"),
            Wat::set_local("new_stack_addr", wat![
                "i32.mul",
                wat!["memory.grow", Wat::const_i32(WASM_PAGE_COUNT_PER_STACK)],
                Wat::const_i32(WASM_PAGE_BYTE_SIZE)
            ]),
            Wat::get_local("new_stack_addr"),
            Wat::call("log_special", vec![])
        ])
    }

    fn get_alloc_function(&self, module: &WasmModule) -> Wat {
        Wat::declare_function(ALLOC_FUNC_NAME, None, vec![("block_size", "i32")], Some("i32"), vec![
            Wat::declare_local_i32("i"),
            Wat::declare_local_i32("j"),
            Wat::declare_local_i32("block_size_index"),
            Wat::declare_local_i32("metadata_addr_start"),
            Wat::declare_local_i32("new_stack_addr"),
            Wat::declare_local_i32("stack_value_count"),
            Wat::declare_local_i32("rounded_block_size"),
            Wat::declare_local_i32("stack_top_addr"),
            Wat::declare_local_i32("current_stack_addr"),
            Wat::declare_local_i32("block_byte_size"),
            Wat::declare_local_i32("result"),
            Wat::declare_local_i32("index_on_stack"),

            Wat::set_local("block_size_index",
                wat!["i32.add", module.std_lib.log_4(Wat::get_local("block_size")), Wat::const_i32(-1)]
            ), // block_size_index = log4(block_size) - 1

            Wat::set_local("metadata_addr_start", wat!["i32.add",
                Wat::const_i32(STACKS_METADATA_BYTE_OFFSET),
                wat!["i32.mul", Wat::get_local("block_size_index"), Wat::const_i32(STACK_BLOCK_BYTE_SIZE)]
            ]), // metadata_addr_start = STACKS_METADATA_BYTE_OFFSET + block_size_index * STACK_BLOCK_BYTE_SIZE

            Wat::set_local("i", Wat::get_local("metadata_addr_start")), // i = metadata_addr_start
            Wat::set_local("index_on_stack", Wat::mem_get_i32("i")), // index_on_stack = memory[i]
            Wat::while_loop(wat!["i32.eq", Wat::get_local("index_on_stack"), Wat::const_i32(EMPTY_STACK_MARKER)], vec![ // while (index_on_stack == -1)
                Wat::increment_local_i32("i", ATOMIC_VALUE_BYTE_SIZE * STACK_METADATA_SIZE), // i += 2
                Wat::set_local("index_on_stack", Wat::mem_get_i32("i")), // index_on_stack = memory[i]
            ]),

            Wat::if_else(
                wat!["i32.eq", Wat::mem_get_i32("i"), Wat::const_i32(UNINITIALIZED_STACK_MARKER)], // if (i == -2)
                vec![ // then
                    Wat::set_local("new_stack_addr", Wat::call(ALLOC_STACK_FUNC_NAME, vec![])), // new_stack_addr = alloc_new_stack()
                    Wat::set_local("rounded_block_size",
                        module.std_lib.pow_4(wat!["i32.add", Wat::get_local("block_size_index"), Wat::const_i32(1)])
                    ), // rounded_block_size = 4 ** (block_size_index + 1)

                    Wat::set_local("block_byte_size", wat!["i32.mul", Wat::get_local("rounded_block_size"), Wat::const_i32(ATOMIC_VALUE_BYTE_SIZE)]),
                    Wat::set_local("stack_value_count", wat![
                        "i32.div_u",
                        Wat::const_i32(ATOMIC_VALUE_COUNT_PER_STACK),
                        wat!["i32.add", Wat::get_local("rounded_block_size"), Wat::const_i32(1)]
                    ]), // stack_value_count = ATOMIC_VALUE_COUNT_PER_STACK / (rounded_block_size + 1)

                    Wat::set_local("stack_top_addr", wat![
                        "i32.add",
                        Wat::get_local("new_stack_addr"),
                        wat!["i32.mul", Wat::get_local("stack_value_count"), Wat::const_i32(ATOMIC_VALUE_BYTE_SIZE)]
                    ]),
                    Wat::set_local("j", Wat::get_local("new_stack_addr")),
                    Wat::set_local("current_stack_addr", Wat::get_local("stack_top_addr")),

                    Wat::while_loop(
                        wat!["i32.lt_u", Wat::get_local("j"), Wat::get_local("stack_top_addr")], // while (j < stack_top_addr)
                        vec![
                            Wat::mem_set_i32("j", Wat::get_local("current_stack_addr")), // memory[j] = current_stack_addr
                            Wat::set_local("current_stack_addr", wat!["i32.add", Wat::get_local("current_stack_addr"), Wat::get_local("block_byte_size")]), // current_stack_addr += block_byte_size
                            Wat::increment_local_i32("j", ATOMIC_VALUE_BYTE_SIZE), // j += ATOMIC_VALUE_BYTE_SIZE
                        ]
                    ),

                    Wat::mem_set_i32("i", wat!["i32.sub", Wat::get_local("stack_value_count"), Wat::const_i32(1)]), // memory[i] = stack_value_count - ATOMIC_VALUE_BYTE_SIZE
                    Wat::mem_set_i32_with_offset("i", ATOMIC_VALUE_BYTE_SIZE, Wat::get_local("new_stack_addr")), // memory[i + 1] = new_stack_addr
                    Wat::set_local("index_on_stack", Wat::mem_get_i32("i"))
                ],
                vec![]
            ),

            // result = memory[(memory[i] * ATOMIC_VALUE_BYTE_SIZE) + memory[i+1]]
            Wat::set_local("result", wat!["i32.load", wat![
                "i32.add",
                wat!["i32.mul", Wat::get_local("index_on_stack"), Wat::const_i32(ATOMIC_VALUE_BYTE_SIZE)],
                Wat::mem_get_i32_with_offset("i", ATOMIC_VALUE_BYTE_SIZE)
            ]]),

            Wat::mem_set_i32("i", wat!["i32.sub", Wat::get_local("index_on_stack"), Wat::const_i32(1)]),

            Wat::get_local("result")
        ])
    }

    pub fn get_header(&self) -> Vec<Wat> {
        merge![
            wat!["memory", Wat::export("memory"), 1]
        ]
    }

    pub fn get_functions(&self, module: &WasmModule) -> Vec<Wat> {
        merge![
            self.get_init_function(),
            self.get_alloc_stack_function(),
            self.get_alloc_function(module)
        ]
    }

    pub fn init(&self) -> Wat {
        Wat::call(INIT_FUNC_NAME, vec![])
    }

    pub fn alloc(&self, block_size: Wat) -> Wat {
        Wat::call(ALLOC_FUNC_NAME, vec![block_size])
    }
}