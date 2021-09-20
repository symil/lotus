use std::{collections::HashMap, marker::PhantomData, mem::{self, take}, thread::sleep, time::Duration};

use serializable::Serializable;

use super::websocket_server::{Message, State, WebSocket, Mode, WebSocketServer};
use crate::{ServerMessage, ServerApi, Id, World};

pub struct Server<U, R, E, W>
    where
        W : World<U, R, E>
{
    api: Option<ServerApi<E>>,
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
            api: None,
            websocket_server: unsafe { mem::zeroed() },
            connections: HashMap::new(),
            world,
            _u: PhantomData,
            _r: PhantomData,
        }
    }

    pub fn start(&mut self) {
        let mut api = ServerApi::new();
        let websocket_server = WebSocketServer::bind("127.0.0.1:8123", Mode::NonBlocking);

        self.world.on_start(&mut api);

        self.api = Some(api);
        self.websocket_server = websocket_server;

        loop {
            self.update();

            sleep(Duration::from_millis(5));
        }
    }

    fn update(&mut self) {
        let mut api = take(&mut self.api).unwrap();

        match self.websocket_server.accept() {
            Some(websocket) => {
                let id : Id = rand::random();

                self.connections.insert(id, websocket);
            },
            None => {}
        }

        for (id, websocket) in self.connections.iter_mut() {
            loop {
                match websocket.read_message() {
                    Some(message) => {
                        match message {
                            Message::Connection => {
                                api.notify_update(*id);
                                self.world.on_user_connect(&mut api, *id);
                            },
                            Message::Disconnection | Message::Error => {
                                self.world.on_user_disconnect(&mut api, *id);
                            },
                            Message::Binary(bytes) => {
                                if let Some(request) = R::deserialize(&bytes) {
                                    match self.world.on_user_request(&mut api, *id, request) {
                                        Ok(ids) => {
                                            for id in ids {
                                                api.notify_update(id);
                                            }
                                        },
                                        Err(_) => { }
                                    }
                                }
                            },
                            _ => {}
                        }
                    },
                    None => break
                }
            }
        }

        for id in self.world.update(&mut api) {
            api.notify_update(id);
        }

        self.connections.retain(|_id, websocket| {
            websocket.get_state() != State::Closed
        });

        for (id, events) in api.poll_outgoing_messages().into_iter() {
            if let Some(websocket) = self.connections.get_mut(&id) {
                let message = ServerMessage {
                    user: self.world.get_user_state(id),
                    events
                };
                let bytes = message.serialize();

                websocket.send_binary(&bytes);
            }
        }

        self.api = Some(api);
    }
}