use std::{collections::HashMap, marker::PhantomData, mem::{self, take}, thread::sleep, time::Duration};
use lotus_common::{Serializable, server_message::ServerMessage, server_state::ServerState, traits::{world::{Id, World}}};

use crate::websocket_server::{websocket::{Message, State, WebSocket}, websocket_server::{Mode, WebSocketServer}};

pub struct Server<U, R, E, W>
    where
        W : World<U, R, E>
{
    state: Option<ServerState<E>>,
    websocket_server: WebSocketServer,
    connections: HashMap<Id, WebSocket>,
    world: W,

    // wtf rust
    _u: PhantomData<U>,
    _r: PhantomData<R>,
}

impl<U, R, E, W> Server<U, R, E, W>
    where
        U : Serializable + 'static,
        R : Serializable,
        E : Serializable,
        W : World<U, R, E>
{
    #[allow(invalid_value)]
    pub fn new(world: W) -> Self {
        Self {
            state: None,
            websocket_server: unsafe { mem::zeroed() },
            connections: HashMap::new(),
            world,
            _u: PhantomData,
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
                let id : Id = rand::random();

                self.connections.insert(id, websocket);
            },
            None => {}
        }

        for (id, websocket) in self.connections.iter_mut() {
            match websocket.read_message() {
                Some(message) => {
                    match message {
                        Message::Connection => {
                            state.notify_update(*id);
                            self.world.on_user_connect(&mut state, *id);
                        },
                        Message::Disconnection | Message::Error => {
                            self.world.on_user_disconnect(&mut state, *id);
                        },
                        Message::Binary(bytes) => {
                            if let Some(request) = R::deserialize(&bytes) {
                                match self.world.on_user_request(&mut state, *id, request) {
                                    Ok(ids) => {
                                        for id in ids {
                                            state.notify_update(id);
                                        }
                                    },
                                    Err(_) => { }
                                }
                            }
                        },
                        _ => {}
                    }
                },
                None => {}
            }
        }

        self.connections.retain(|_id, websocket| {
            websocket.get_state() != State::Closed
        });

        self.world.update(&mut state);

        for (id, events) in state.poll_outgoing_messages().into_iter() {
            if let Some(websocket) = self.connections.get_mut(&id) {
                let message = ServerMessage {
                    user: self.world.get_user_state(id),
                    events
                };
                let bytes = message.serialize();

                websocket.send_binary(&bytes);
            }
        }

        self.state = Some(state);
    }
}