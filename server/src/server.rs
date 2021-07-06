use std::{cell::RefCell, collections::HashMap, marker::PhantomData, rc::Rc, thread::sleep, time::Duration};
use lotus_common::{Serializable, server_api::ServerApi, server_message::ServerMessage, traits::{world::World}};

use crate::websocket_server::{websocket::{Message, State, WebSocket}, websocket_server::{Mode, WebSocketServer}};

pub struct Connection<P> {
    websocket: WebSocket,
    player: Rc::<RefCell<P>>,
}

pub struct Server<P, R, E, W>
    where
        W : World<P, R, E>
{
    connections: HashMap<usize, Connection<P>>,
    api: ServerApi<E>,
    world: W,

    // wtf rust
    _r: PhantomData<R>,
}

impl<P, R, E, W> Server<P, R, E, W>
    where
        P : Serializable + Default + 'static,
        R : Serializable,
        E : Serializable,
        W : World<P, R, E>
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
                                self.api.notify_state_update(&connection.player);
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

            for (id, events) in self.api.poll_outgoing_messages().into_iter() {
                if let Some(connection) = self.connections.get_mut(&id) {
                    let message = ServerMessage {
                        player: Rc::clone(&connection.player),
                        events
                    };
                    let bytes = message.serialize();

                    connection.websocket.send_binary(&bytes);
                }
            }

            sleep(Duration::from_millis(5));
        }
    }
}