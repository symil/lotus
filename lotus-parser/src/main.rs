#![allow(unused)]
use std::{env, fs, process};
use colored::*;
use generation::generate_wat;
use program::LotusProgram;

mod program;
mod items;
mod generation;

#[macro_use]
macro_rules! prelude_file {
    ($name:literal) => {
        ($name, include_str!($name))
    }
}

const PROGRAM_NAME : &'static str = "lotus";
const PRELUDE_FILES : &'static[(&'static str, &'static str)] = &[
    prelude_file!("prelude/math.lt"),
    prelude_file!("prelude/memory.lt"),
];

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = args.get(1).or_else(|| display_usage_and_exit()).unwrap();
    let output_path = args.get(2).or_else(|| display_usage_and_exit()).unwrap();
    let silent = args.iter().any(|s| s == "--silent");

    match LotusProgram::from_path(input_path, PRELUDE_FILES) {
        Ok(program) => {
            program.write_to(output_path);

            if !silent {
                println!("{} {} ({}s)", "ok:".blue().bold(), output_path.bold(), program.process_time);
            }

            process::exit(0);
        },
        Err(errors) => {
            for error in errors {
                println!("{}", error.to_string());
            }
            process::exit(1);
        }
    };
}

fn display_usage_and_exit() -> ! {
    println!("{} {} <input_dir_or_file> <output_file>", "usage:".magenta().bold(), PROGRAM_NAME.bold());
    process::exit(1)
}