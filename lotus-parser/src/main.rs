#![allow(unused)]
use std::fs;
use generation::generate_wat;
use program::LotusProgram;

mod program;
mod items;
mod generation;

fn main() {
    // match LotusProgram::from_directory_path("test") {
    //     Ok(_) => {
    //         println!("parse ok");
    //     },
    //     Err(errors) => {
    //         for error in errors {
    //             println!("{}", error.to_string());
    //         }
    //     }
    // }

    let wat = generate_wat();

    fs::write("build/module.wat", &wat);
}