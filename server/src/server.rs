use std::{collections::HashMap, marker::PhantomData, net::{TcpListener, TcpStream}, thread::sleep, time::Duration, u128};
use lotus_common::{serialization::serializable::Serializable, server_api::ServerApi, state_message::StateMessage, traits::{view::View, player::Player, request::Request, world::World}};
use rand::{Rng, prelude::ThreadRng, thread_rng};
use tungstenite::{Message, WebSocket, Error, accept};

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
    api: ServerApi,
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
            api: ServerApi::new(),

            _r: PhantomData
        }
    }

    pub fn start(&mut self) {
        let server = TcpListener::bind("127.0.0.1:8123").unwrap();
        server.set_nonblocking(true).expect("Cannot set non-blocking");

        println!(">>> READY <<<");
        println!("Listening on port 8123...");

        loop {
            match server.accept() {
                Ok((stream, _addr)) => {
                    stream.set_nonblocking(true).unwrap();

                    let open = true;
                    let id : u128 = self.rng.gen();
                    let websocket = accept(stream).unwrap();
                    let mut player = P::from_id(id);

                    self.api.notify_player_update(&player);
                    self.world.on_player_connect(&mut player, &mut self.api);
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
                                dbg!(4);
                                if let Some(request) = R::deserialize(&bytes) {
                                    self.world.on_player_request(&mut connection.player, &request, &mut self.api);
                                }
                            },
                            _ => {}
                        }
                    },
                    Err(error) => {
                        match error {
                            Error::ConnectionClosed | Error::AlreadyClosed => {
                                self.world.on_player_disconnect(&mut connection.player, &mut self.api);
                                connection.open = false;
                            },
                            _ => {}
                        }
                    },
                }
            }

            self.connections.retain(|_id, connection| {
                connection.open
            });

            self.world.update();

            for id in self.api.drain_players_to_notify() {
                if let Some(connection) = self.connections.get_mut(&id) {
                    let bytes = connection.player.serialize();

                    match connection.websocket.write_message(Message::Binary(bytes)) {
                        Ok(_) => {},
                        Err(_) => {},
                    }
                }
            }

            sleep(Duration::from_millis(5));
        }
    }
}