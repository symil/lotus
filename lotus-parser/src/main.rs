#![feature(option_result_contains)]
#![allow(unused)]
use std::{env, fs, process};
use colored::*;
use program::LotusProgram;

mod utils;
mod program;
mod items;
mod generation;

const PROGRAM_NAME : &'static str = "lotus";

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = args.get(1).or_else(|| display_usage_and_exit()).unwrap();
    let output_path = args.get(2).or_else(|| display_usage_and_exit()).unwrap();
    let silent = args.iter().any(|s| s == "--silent");
    let prelude_path = match args.iter().find(|s| s.starts_with("--prelude-path=")) {
        Some(string) => Some(string.split('=').collect::<Vec<&str>>()[1]),
        None => None,
    };

    match LotusProgram::from_path(input_path, prelude_path) {
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