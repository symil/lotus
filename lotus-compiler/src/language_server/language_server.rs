use std::{str, net::TcpListener, io::{Read, Write}, collections::HashMap, thread::sleep, time::Duration};
use colored::Colorize;
use parsable::StringReader;
use crate::{program::{ProgramContext, ProgramContextOptions}, command_line::{infer_root_directory, bundle_with_prelude}};
use super::{LanguageServerCommand, LanguageServerCommandParameters};

const PORT : u16 = 9609;
const BUFFER_SIZE : usize = 65536;
const COMMAND_SEPARATOR : char = ';';

pub fn start_language_server() {
    let mut buffer = [0 as u8; BUFFER_SIZE];
    let mut current_root_directory = String::new();
    let mut connections = vec![];
    let mut context = ProgramContext::new(ProgramContextOptions {
        validate_only: true,
    });

    let addr = format!("127.0.0.1:{}", PORT);
    let listener = TcpListener::bind(addr).unwrap();

    listener.set_nonblocking(true).unwrap();

    println!("{} server open on port {}", "info:".bold(), PORT.to_string().bold());

    loop {
        match listener.accept() {
            Ok((stream, _)) => {
                // println!("OPEN");
                stream.set_nonblocking(true).unwrap();
                connections.push(stream);
            },
            Err(_) => {},
        }

        for stream in &mut connections {
            match stream.read(&mut buffer) {
                Ok(size) => {
                    if size == 0 {
                        // println!("CLOSE");
                        return;
                    }

                    let content = str::from_utf8(&buffer[0..size]).unwrap();
                    // println!("{}", content);
                    let mut arguments = content.split(COMMAND_SEPARATOR);
                    let id = arguments.next().and_then(|str| str.parse::<u32>().ok()).unwrap_or(0);
                    let command = arguments.next().and_then(|str| LanguageServerCommand::from_str(str));
                    let file_path = arguments.next().and_then(|str| Some(str.to_string()));
                    let cursor_index = arguments.next().and_then(|str| str.parse::<usize>().ok());
                    let new_name = arguments.next().and_then(|str| Some(str.to_string()));
                    let root_directory_path = infer_root_directory(file_path.as_ref().map(|s| s.as_str()).unwrap_or_default());
                    let parameters = LanguageServerCommandParameters { root_directory_path, file_path, cursor_index, new_name };

                    if let Some(command) = command {
                        let command_content = command.get_content();
                        
                        if let Some(root_directory) = &parameters.root_directory_path {
                            let mut lines = vec![id.to_string()];

                            if command_content.force_init || context.is_new() || root_directory != &current_root_directory {
                                current_root_directory = root_directory.clone();
                                
                                StringReader::clear_all_static_strings();
                                context.reset();
                                context.read_source_files(&bundle_with_prelude(&root_directory));
                                context.parse_source_files();

                                if !context.has_errors() {
                                    context.process_source_files();
                                }
                            }

                            (command_content.callback)(&parameters, &context, &mut lines);

                            let content = lines.join("\n");

                            for (src, dest) in content.as_bytes().iter().zip(buffer.as_mut()) {
                                *dest = *src;
                            }

                            stream.write(&buffer[0..content.as_bytes().len()]).unwrap();
                        }
                    }
                },
                Err(error) => {
                },
            }
        }

        sleep(Duration::from_millis(5));
    }
}