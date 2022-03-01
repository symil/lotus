use std::{str, net::TcpListener, io::{Read, Write}, collections::HashMap, thread::sleep, time::{Duration, Instant}};
use colored::Colorize;
use parsable::StringReader;
use crate::{program::{ProgramContext, ProgramContextOptions}, command_line::{infer_root_directory, bundle_with_prelude}, utils::FileSystemCache, language_server::LanguageServerCommand};
use super::{LanguageServerCommandKind, LanguageServerCommandParameters};

const PORT : u16 = 9609;
const BUFFER_SIZE : usize = 65536;

pub fn start_language_server(test_command: &Option<String>) {
    let mut buffer = [0 as u8; BUFFER_SIZE];
    let mut connections = vec![];
    let mut cache = FileSystemCache::new();

    if let Some(string) = test_command {
        let command = LanguageServerCommand::from_str(string).unwrap();
        let output = command.run(None);
        
        println!("{}", output);

        return;
    }

    let addr = format!("127.0.0.1:{}", PORT);
    let listener = TcpListener::bind(addr).unwrap();

    listener.set_nonblocking(true).unwrap();

    println!("{} server open on port {}", "info:".bold(), PORT.to_string().bold());

    loop {
        match listener.accept() {
            Ok((stream, _)) => {
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

                    if let Some(command) = LanguageServerCommand::from_str(content) {
                        let output = command.run(Some(&mut cache));
                        let bytes = output.as_bytes();

                        for (src, dest) in bytes.iter().zip(buffer.as_mut()) {
                            *dest = *src;
                        }

                        stream.write(&buffer[0..bytes.len()]).unwrap();
                    }

                    // println!("COMMAND TOOK: {}ms", duration);
                },
                Err(error) => {
                },
            }
        }

        sleep(Duration::from_millis(5));
    }
}