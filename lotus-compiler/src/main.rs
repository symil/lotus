#![feature(option_result_contains)]
#![feature(array_methods)]
#![allow(unused)]
use std::{env, fs, path::{Path, PathBuf}, process};
use colored::*;
use program::{LotusProgram, Timer, ProgramStep};

mod utils;
mod program;
mod items;

const PROGRAM_NAME : &'static str = "lotus-compiler";
const PRELUDE_DIR_NAME : &'static str = "prelude";

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = args.get(1).or_else(|| display_usage_and_exit()).unwrap();
    let output_path = args.get(2).or_else(|| display_usage_and_exit()).unwrap();
    let silent = args.iter().any(|s| s == "--silent");
    let details = args.iter().any(|s| s == "--details");
    let prelude_path = get_prelude_path();
    let mut timer = Timer::new();

    timer.start(ProgramStep::Total);

    match LotusProgram::from_path(input_path, Some(&prelude_path), &mut timer) {
        Ok(program) => {
            timer.start(ProgramStep::Write);
            program.write_to(output_path);
            timer.stop(ProgramStep::Write);

            let total_time = timer.stop(ProgramStep::Total);

            if !silent {
                match details {
                    true => {
                        for (step, duration) in timer.consume() {
                            if !step.is_negligible() {
                                println!("{}: {}s", step.get_name().bold(), duration);
                            }
                        }
                    },
                    false => {
                        println!("{} {} ({}s)", "ok:".blue().bold(), output_path.bold(), total_time);
                    },
                }
            }

            process::exit(0);
        },
        Err(errors) => {
            for error in errors {
                if let Some(string) = error.to_string() {
                    println!("{}", string);
                }
            }
            process::exit(1);
        }
    };
}

fn get_prelude_path() -> String {
    let mut path_buf = PathBuf::new();

    path_buf.push(env!("CARGO_MANIFEST_DIR"));
    path_buf.push(PRELUDE_DIR_NAME);

    path_buf.into_os_string().into_string().unwrap()
}

fn display_usage_and_exit() -> ! {
    println!("{} {} <input_dir_or_file> <output_file>", "usage:".magenta().bold(), PROGRAM_NAME.bold());
    process::exit(1)
}