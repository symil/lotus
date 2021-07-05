use std::{cell::RefCell, collections::HashMap, marker::PhantomData, rc::Rc, thread::sleep, time::Duration};
use lotus_common::{serialization::Serializable, server_api::ServerApi, traits::{world::World}};

use crate::websocket_server::{websocket::{Message, State, WebSocket}, websocket_server::{Mode, WebSocketServer}};

pub struct Connection<P> {
    websocket: WebSocket,
    player: Rc::<RefCell<P>>,
}

pub struct Server<P, R, W>
    where
        W : World<P, R>
{
    connections: HashMap<usize, Connection<P>>,
    api: ServerApi,
    world: W,

    // wtf rust
    _r: PhantomData<R>,
}

impl<P, R, W> Server<P, R, W>
    where
        P : Serializable + Default,
        R : Serializable,
        W : World<P, R>
{
    pub fn new(world: W) -> Self {
        Self {
            connections: HashMap::new(),
            world,
            api: ServerApi::new(),

            _r: PhantomData
        }
    }

    pub fn start(&mut self) {
        let mut websocket_server = WebSocketServer::bind("127.0.0.1:8123", Mode::NonBlocking);

        self.world.on_start(&mut self.api);

        loop {
            match websocket_server.accept() {
                Some(websocket) => {
                    let player = Rc::new(RefCell::new(P::default()));
                    let addr = player.as_ptr() as usize;

                    self.connections.insert(addr, Connection { websocket, player });
                },
                None => {}
            }

            for connection in self.connections.values_mut() {
                match connection.websocket.read_message() {
                    Some(message) => {
                        match message {
                            Message::Connection => {
                                self.api.notify_player_update(&connection.player);
                                self.world.on_player_connect(&mut self.api, &connection.player);
                            },
                            Message::Disconnection | Message::Error => {
                                self.world.on_player_disconnect(&mut self.api, &mut connection.player);
                            },
                            Message::Binary(bytes) => {
                                if let Some(request) = R::deserialize(&bytes) {
                                    self.world.on_player_request(&mut self.api, &mut connection.player, &request);
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

            self.world.update(&mut self.api);

            for id in self.api.drain_players_to_notify() {
                if let Some(connection) = self.connections.get_mut(&id) {
                    let bytes = connection.player.borrow().serialize();

                    connection.websocket.send_binary(&bytes);
                }
            }

            sleep(Duration::from_millis(5));
        }
    }
}