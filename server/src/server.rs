use std::{collections::HashMap, marker::PhantomData, thread::sleep, time::Duration, u128};
use lotus_common::{server_api::ServerApi, traits::{player::Player, request::Request, world::World}};
use rand::{Rng, prelude::ThreadRng, thread_rng};

use crate::websocket_server::{websocket::{Message, State, WebSocket}, websocket_server::{Mode, WebSocketServer}};

pub struct Connection<P : Player> {
    websocket: WebSocket,
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
        let mut websocket_server = WebSocketServer::bind("127.0.0.1:8123", Mode::NonBlocking);

        self.world.on_start();

        loop {
            match websocket_server.accept() {
                Some(websocket) => {
                    let id : u128 = self.rng.gen();
                    let player = P::from_id(id);

                    self.connections.insert(id, Connection { websocket, player });
                },
                None => {}
            }

            for connection in self.connections.values_mut() {
                match connection.websocket.read_message() {
                    Some(message) => {
                        match message {
                            Message::Connection => {
                                self.api.notify_player_update(&connection.player);
                                self.world.on_player_connect(&mut connection.player, &mut self.api);
                            },
                            Message::Disconnection | Message::Error => {
                                self.world.on_player_disconnect(&mut connection.player, &mut self.api);
                            },
                            Message::Binary(bytes) => {
                                if let Some(request) = R::deserialize(&bytes) {
                                    self.world.on_player_request(&mut connection.player, &request, &mut self.api);
                                }
                            },
                            _ => {}
                        }
                    },
                    None => {}
                }
            }

            self.connections.retain(|_id, connection| {
                connection.websocket.get_state() != State::Closed
            });

            self.world.update();

            for id in self.api.drain_players_to_notify() {
                if let Some(connection) = self.connections.get_mut(&id) {
                    let bytes = connection.player.serialize();

                    connection.websocket.send_binary(&bytes);
                }
            }

            sleep(Duration::from_millis(5));
        }
    }
}