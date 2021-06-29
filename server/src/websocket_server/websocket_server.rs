#![allow(dead_code)]
use std::net::TcpListener;

use super::websocket::WebSocket;

pub struct WebSocketServer {
    listener: TcpListener,
    nonblocking: bool
}

#[derive(Clone, Copy)]
pub enum Mode {
    Blocking,
    NonBlocking
}

impl WebSocketServer {
    pub fn bind(addr: &str, mode: Mode) -> Self {
        let listener = TcpListener::bind(addr).unwrap();
        let nonblocking = match mode {
            Mode::Blocking => false,
            Mode::NonBlocking => true
        };

        listener.set_nonblocking(nonblocking).unwrap();

        Self { listener, nonblocking }
    }

    pub fn accept(&mut self) -> Option<WebSocket> {
        match self.listener.accept() {
            Ok((stream, addr)) => {
                stream.set_nonblocking(self.nonblocking).unwrap();

                Some(WebSocket::new(stream, addr))

            },
            Err(_) => None
        }
    }
}