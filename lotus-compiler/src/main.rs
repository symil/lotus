#![allow(incomplete_features)]
#![feature(option_result_contains)]
#![feature(array_methods)]
#![feature(adt_const_params)]
#![allow(unused)]
use std::{env, process};
use colored::*;
use command_line::{CommandLineOptions, LogLevel, Timer, ProgramStep};
use indexmap::IndexSet;
use language_server::start_language_server;
use package::Package;
use program::{ProgramContext, ProgramContextOptions};
use utils::FileSystemCache;

use crate::program::ProgramContextMode;

mod utils;
mod program;
mod items;
mod command_line;
mod package;
mod language_server;

const PROGRAM_NAME : &'static str = "lotus-compiler";

fn main() {
    let args: Vec<String> = env::args().collect();
    let options = CommandLineOptions::parse_from_args(args);

    if options.run_benchmark {
        let package = Package::from_path(options.input_path.as_ref().unwrap());
        let source_directories = package.get_source_directories();
        let mut cache = FileSystemCache::new();
        let mut context = ProgramContext::new(ProgramContextOptions::compile());
        let mut timer = Timer::new();

        for i in 0..3 {
            let duration = timer.time(ProgramStep::Total, || {
                context = ProgramContext::new(ProgramContextOptions {
                    mode: ProgramContextMode::Validate,
                    cursor_location: None,
                });
                context.parse_source_files(&source_directories, Some(&mut cache));
                context.process_source_files();
            });

            println!("validation #{}: {} ms", i + 1, (duration * 1000.0).round());
        }

    } else if options.run_as_server {
        start_language_server(&options.command);
    } else {
        // dbg!(&options);
        if let (Some(input_path), Some(output_path)) = (&options.input_path, &options.output_path) {
            let package = Package::from_path(input_path);
            let source_directories = package.get_source_directories();
            let program_options = match options.validate {
                true => ProgramContextOptions::validate(),
                false => ProgramContextOptions::compile(),
            };
            let mut timer = Timer::new();
            let mut context = ProgramContext::new(program_options);

            timer.time(ProgramStep::Parse, || context.parse_source_files(&source_directories, None));

            if !options.validate && !context.has_errors() {
                timer.time(ProgramStep::Process, || context.process_source_files());
            }

            match context.take_errors() {
                Some(errors) => {
                    let mut set = IndexSet::new();

                    for error in errors {
                        if let Some(string) = error.to_string() {
                            set.insert(string);
                        }
                    }

                    for string in set {
                        println!("{}", string);
                    }
                    
                    process::exit(1);
                },
                None => {
                    if !options.validate {
                        timer.time(ProgramStep::Resolve, || context.resolve_wat());
                        timer.time(ProgramStep::Stringify, || context.generate_output_file());
                        timer.time(ProgramStep::Write, || context.write_output_file(output_path));
                    }

                    match options.log_level {
                        LogLevel::Silent => {},
                        LogLevel::Short => {
                            println!("{} {} ({}s)", "ok:".blue().bold(), output_path.bold(), timer.get_total_duration());
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
            }
        } else {
            display_usage_and_exit()
        }
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