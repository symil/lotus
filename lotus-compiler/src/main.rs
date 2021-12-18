#![feature(option_result_contains)]
#![feature(array_methods)]
#![allow(unused)]
use std::{env, fs, path::{Path, PathBuf}, process};
use colored::*;
use command_line::{CommandLineOptions, LogLevel, PROGRAM_NAME, CompilerMode, Timer, ProgramStep};
use program::{ProgramContext};

mod utils;
mod program;
mod items;
mod command_line;

fn main() {
    let args: Vec<String> = env::args().collect();

    match CommandLineOptions::parse_from_args(args) {
        Some(options) => {
            let directories = options.get_source_directories();
            let mut timer = Timer::new();
            let mut context = ProgramContext::new();

            timer.time(ProgramStep::Write, || context.read_source_files(&directories));
            timer.time(ProgramStep::Parse, || context.parse_source_files());

            if !context.has_errors() {
                timer.time(ProgramStep::Process, || context.process_source_files());
            }

            match options.mode {
                CompilerMode::Compile => match context.take_errors() {
                    Some(errors) => {
                        for error in errors {
                            if let Some(string) = error.to_string() {
                                println!("{}", string);
                            }
                        }
                        process::exit(1);
                    },
                    None => {
                        timer.time(ProgramStep::Resolve, || context.resolve_wat());
                        timer.time(ProgramStep::Stringify, || context.generate_output_file());
                        timer.time(ProgramStep::Write, || context.write_output_file(&options.output_path));

                        match options.log_level {
                            LogLevel::Silent => {},
                            LogLevel::Short => {
                                println!("{} {} ({}s)", "ok:".blue().bold(), options.output_path.bold(), timer.get_total_duration());
                            },
                            LogLevel::Detailed => {
                                for (step, duration) in timer.get_all_durations() {
                                    if !step.is_negligible() {
                                        print_step(step.get_name(), duration);
                                    }
                                }

                                print_step("total", timer.get_total_duration());
                            },
                        }
                    },
                },
                CompilerMode::Validate => {
                    for error in context.errors.consume() {
                        if let Some(string) = error.to_detailed_string() {
                            println!("{}", string);
                        }
                    }
                },
            }
        },
        None => display_usage_and_exit(),
    }
}

fn print_step(name: &str, time: f64) {
    let name_string = format!("{}:", name);

    println!("{: <10} {}s", name_string.bold(), time);
}

fn display_usage_and_exit() -> ! {
    println!("{} {} <input_directory> <output_file>", "usage:".magenta().bold(), PROGRAM_NAME.bold());
    process::exit(1)
}