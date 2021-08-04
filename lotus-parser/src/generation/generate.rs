use crate::{generation::{Imports, MainFunction, Memory, Wat}, merge};

pub fn generate_wat() -> String {
    let imports = Imports::new();
    let memory = Memory::new();
    let main_function = MainFunction::new();

    let module = Wat::new("module", merge![
        imports.get_header(),
        memory.get_header(),
        main_function.get_header()
    ]);

    module.to_string(0)
}