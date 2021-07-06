use std::{cell::RefCell, collections::HashMap, marker::PhantomData, mem::{self, take}, rc::Rc, thread::sleep, time::Duration};
use lotus_common::{Serializable, server_state::ServerState, server_message::ServerMessage, traits::{world::World}};

use crate::websocket_server::{websocket::{Message, State, WebSocket}, websocket_server::{Mode, WebSocketServer}};

pub struct Connection<P> {
    websocket: WebSocket,
    player: Rc::<RefCell<P>>,
}

pub struct Server<P, R, E, W>
    where
        W : World<P, R, E>
{
    state: Option<ServerState<E>>,
    websocket_server: WebSocketServer,
    connections: HashMap<usize, Connection<P>>,
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
            state: None,
            #[allow(invalid_value)]
            websocket_server: unsafe { mem::zeroed() },
            connections: HashMap::new(),
            world,
            _r: PhantomData
        }
    }

    pub fn start(&mut self) {
        let mut state = ServerState::new();
        let websocket_server = WebSocketServer::bind("127.0.0.1:8123", Mode::NonBlocking);

        self.world.on_start(&mut state);

        self.state = Some(state);
        self.websocket_server = websocket_server;

        loop {
            self.update();

            sleep(Duration::from_millis(5));
        }
    }

    fn update(&mut self) {
        let mut state = take(&mut self.state).unwrap();

        match self.websocket_server.accept() {
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
                            state.notify_state_update(&connection.player);
                            self.world.on_player_connect(&mut state, &connection.player);
                        },
                        Message::Disconnection | Message::Error => {
                            self.world.on_player_disconnect(&mut state, &mut connection.player);
                        },
                        Message::Binary(bytes) => {
                            if let Some(request) = R::deserialize(&bytes) {
                                self.world.on_player_request(&mut state, &mut connection.player, &request);
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

        self.world.update(&mut state);

        for (id, events) in state.poll_outgoing_messages().into_iter() {
            if let Some(connection) = self.connections.get_mut(&id) {
                let message = ServerMessage {
                    player: Rc::clone(&connection.player),
                    events
                };
                let bytes = message.serialize();

                connection.websocket.send_binary(&bytes);
            }
        }

        self.state = Some(state);
    }
}