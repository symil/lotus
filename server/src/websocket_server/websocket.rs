#![allow(dead_code)]
use std::{io::{ErrorKind, Read, Write}, net::{SocketAddr, TcpStream}};

use super::{handshake::compute_websocket_server_handshake, read_buffer::ReadBuffer, write_buffer::WriteBuffer};

#[derive(PartialEq, Clone, Copy)]
pub enum State {
    Connecting,
    Connected,
    Closing,
    Closed
}

pub enum Message {
    Connection,
    Text(String),
    Binary(Vec<u8>),
    Disconnection,
    Error,
    Unsupported,
}

pub struct WebSocket {
    stream: TcpStream,
    addr: SocketAddr,
    state: State,
    incoming_messages: Vec<Message>
}

const TEXT : u8 = 1;
const BINARY : u8 = 2;
const CONNECTION_CLOSE : u8 = 8;
const BUFFER_CAPACITY : usize = 2usize.pow(24);

// This assumes single-threaded server
static mut BUFFER : [u8; BUFFER_CAPACITY] = [0; BUFFER_CAPACITY];

// https://datatracker.ietf.org/doc/html/rfc6455
impl WebSocket {
    pub fn new(stream: TcpStream, addr: SocketAddr) -> Self {
        Self {
            stream,
            addr,
            state: State::Connecting,
            incoming_messages: vec![]
        }
    }

    pub fn get_addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn get_state(&self) -> State {
        self.state
    }

    fn queue(&mut self, message: Message) {
        if self.state == State::Closed {
            return;
        }

        match &message {
            Message::Connection => self.state = State::Connected,
            Message::Disconnection => self.state = State::Closed,
            Message::Error => self.state = State::Closed,
            _ => {}
        };

        self.incoming_messages.push(message);
    }

    pub fn read_message(&mut self) -> Option<Message> {
        match unsafe { self.stream.read(&mut BUFFER) } {
            Ok(size) => {
                match size {
                    0 => {},
                    _ => match self.state {
                        State::Connecting => {
                            let client_handshake = unsafe { String::from_utf8_lossy(&BUFFER[0..size]) };

                            match compute_websocket_server_handshake(client_handshake.into_owned()) {
                                Some(handshake) => match self.stream.write(handshake.as_bytes()) {
                                    Ok(_) => self.queue(Message::Connection),
                                    Err(_) => self.queue(Message::Error)
                                },
                                None => self.queue(Message::Error)
                            }
                        },
                        State::Connected => {
                            let mut buffer = ReadBuffer::new(unsafe { &BUFFER[0..size] });

                            while !buffer.is_consumed() {
                                let b1 = buffer.read_byte();
                                let b2 = buffer.read_byte();
                                let fin = b1 >> 7;
                                let opcode = b1 & 0b1111;
                                let mask = b2 >> 7;
                                let mut length = (b2 & 0b1111111) as usize;

                                if length == 126 {
                                    length = u16::from_be_bytes(buffer.read_as_array()) as usize;
                                } else if length == 127 {
                                    length = u64::from_be_bytes(buffer.read_as_array()) as usize;
                                }

                                let mut masking_key : [u8; 4] = [0; 4];

                                if mask == 1 {
                                    // masking_key = u32::from_be_bytes(buffer.read_as_array());
                                    masking_key = buffer.read_as_array();
                                }

                                let mut data = buffer.read(length).to_vec();

                                if mask == 1 {
                                    for i in 0..data.len() {
                                        data[i] = data[i] ^ masking_key[i % 4];
                                    }
                                }

                                let message = match fin {
                                    1 => match opcode {
                                        TEXT => Message::Text(String::from_utf8_lossy(&data).to_string()),
                                        BINARY => Message::Binary(data),
                                        CONNECTION_CLOSE => {
                                            self.send(CONNECTION_CLOSE, &[]);
                                            Message::Disconnection
                                        },
                                        _ => Message::Unsupported
                                    }
                                    _ => Message::Unsupported,
                                };

                                self.queue(message);
                            }

                        },
                        _ => {}
                    }
                }
            }
            Err(error) => match error.kind() {
                ErrorKind::WouldBlock => {},
                ErrorKind::TimedOut => {},
                _ => {
                    dbg!(&error.kind());
                    self.queue(Message::Error)
                }
            }
        };

        match self.incoming_messages.len() {
            0 => None,
            _ => Some(self.incoming_messages.remove(0))
        }
    }

    fn send(&mut self, opcode: u8, data: &[u8]) {
        let mut buffer = unsafe { WriteBuffer::new(&mut BUFFER) };
        let payload_length = data.len();

        let fin = 1;
        let b1 = (fin << 7) + opcode;

        buffer.write_byte(b1);

        if payload_length < 126 {
            buffer.write_byte(payload_length as u8);
        } else if payload_length < 65536 {
            buffer.write_byte(126);
            buffer.write(&(payload_length as u16).to_be_bytes());
        } else {
            buffer.write_byte(127);
            buffer.write(&(payload_length as u64).to_be_bytes());
        }

        buffer.write(data);

        match self.stream.write(buffer.as_bytes()) {
            Ok(_) => {},
            Err(error) => {
                dbg!(error.kind());
            }
        }
    }

    pub fn send_text(&mut self, text: &str) {
        self.send(TEXT, text.as_bytes())
    }

    pub fn send_binary(&mut self, data: &[u8]) {
        self.send(BINARY, data)
    }
}