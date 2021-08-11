#![allow(unused)]
use std::fs;
use generation::generate_wat;
use program::LotusProgram;

mod program;
mod items;
mod generation;

const OUTPUT_PATH : &'static str = "build/module.wat";

fn main() {
    match LotusProgram::from_directory_path("test") {
        Ok(program) => {
            println!("build ok");
            program.write_to(OUTPUT_PATH);
        },
        Err(errors) => {
            for error in errors {
                println!("{}", error.to_string());
            }
        }
    };
}