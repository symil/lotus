use std::{collections::HashMap, marker::PhantomData, net::{TcpListener, TcpStream}, thread::sleep, time::Duration, u128};
use lotus_common::traits::{player::Player, world::World, request::Request};
use rand::{Rng, prelude::ThreadRng, thread_rng};
use tungstenite::{Message, WebSocket, accept};

pub struct Connection<P : Player> {
    open: bool,
    websocket: WebSocket<TcpStream>,
    player: P,
}

pub struct Server<P, R, W>
    where
        P : Player,
        R : Request,
        W : World<P, R>
{
    rng: ThreadRng,
    connections: HashMap<u128, Connection<P>>,
    world: W,

    // wtf rust
    _r: PhantomData<R>,
}

impl<P, R, W> Server<P, R, W>
    where
        P : Player,
        R : Request,
        W : World<P, R>
{
    pub fn new(world: W) -> Self {
        Self {
            rng: thread_rng(),
            connections: HashMap::new(),
            world,

            _r: PhantomData
        }
    }

    pub fn start(&mut self) {
        let server = TcpListener::bind("127.0.0.1:8123").unwrap();
        server.set_nonblocking(true).expect("Cannot set non-blocking");

        println!("Listening on port 8123...");

        loop {
            match server.accept() {
                Ok((stream, _addr)) => {
                    let open = true;
                    let id : u128 = self.rng.gen();
                    let websocket = accept(stream).unwrap();
                    let mut player = P::from_id(id);

                    self.world.on_player_connect(&mut player);
                    self.connections.insert(id, Connection { open, websocket, player });
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
                            Message::Binary(bytes) => {
                                dbg!(&bytes);
                                if let Some(request) = R::deserialize(&bytes) {
                                    self.world.on_player_request(&mut connection.player, &request);
                                }
                            },
                            _ => {}
                        }
                    },
                    Err(_) => {
                        self.world.on_player_disconnect(&mut connection.player);
                        connection.open = false;
                    },
                }
            }

            self.connections.retain(|_id, connection| {
                connection.open
            });

            self.world.update();

            sleep(Duration::from_millis(5));
        }
    }
}