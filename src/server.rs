use std::{collections::HashMap, net::{TcpListener, TcpStream}, thread::sleep, time::Duration, u128};
use rand::{Rng, prelude::ThreadRng, thread_rng};
use tungstenite::{WebSocket, Message, accept};

pub struct Connection {
    id: u128,
    websocket: WebSocket<TcpStream>,
    open: bool
}

pub struct Server {
    rng: ThreadRng,
    connections: HashMap<u128, Connection>
}

impl Server {
    pub fn new() -> Self {
        Self {
            rng: thread_rng(),
            connections: HashMap::new()
        }
    }

    pub fn start(&mut self) {
        let server = TcpListener::bind("127.0.0.1:8123").unwrap();
        server.set_nonblocking(true).expect("Cannot set non-blocking");

        println!("Listening on port 8123...");

        loop {
            match server.accept() {
                Ok((stream, _addr)) => {
                    let id : u128 = self.rng.gen();
                    let websocket = accept(stream).unwrap();
                    let open = true;

                    println!("connected: {}", id);
                    self.connections.insert(id, Connection { id, websocket, open });
                },
                Err(_) => {},
            }

            for connection in self.connections.values_mut() {
                match connection.websocket.read_message() {
                    Ok(message) => {
                        match message {
                            Message::Text(text) => {
                                println!("{}", &text);
                            },
                            Message::Binary(_) => todo!(),
                            Message::Close(_) => {},
                            _ => {}
                        }
                    },
                    Err(_) => {
                        println!("disconnected: {}", connection.id);
                        connection.open = false;
                    },
                }
            }

            self.connections.retain(|_id, connection| {
                connection.open
            });

            sleep(Duration::from_millis(5));
        }
    }
}