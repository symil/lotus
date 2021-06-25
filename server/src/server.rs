use std::{collections::HashMap, marker::PhantomData, net::{TcpListener, TcpStream}, thread::sleep, time::Duration, u128};
use lotus_common::{serialization::serializable::Serializable, server_api::ServerApi, traits::{entity::Entity, player::Player, request::Request, world::World}};
use rand::{Rng, prelude::ThreadRng, thread_rng};
use tungstenite::{Message, WebSocket, accept};

pub struct Connection<P : Player, E : Entity<P>> {
    open: bool,
    websocket: WebSocket<TcpStream>,
    player: P,
    ui: Option<E>
}

pub struct Server<P, R, E, W>
    where
        P : Player,
        R : Request,
        E : Entity<P>,
        W : World<P, R, E>
{
    rng: ThreadRng,
    connections: HashMap<u128, Connection<P, E>>,
    api: ServerApi<P, E>,
    world: W,

    // wtf rust
    _r: PhantomData<R>,
}

impl<P, R, E, W> Server<P, R, E, W>
    where
        P : Player + Serializable,
        R : Request + Serializable,
        E : Entity<P> + Serializable,
        W : World<P, R, E>
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

        println!("Listening on port 8123...");

        loop {
            match server.accept() {
                Ok((stream, _addr)) => {
                    let open = true;
                    let id : u128 = self.rng.gen();
                    let websocket = accept(stream).unwrap();
                    let mut player = P::from_id(id);
                    let ui = None;

                    self.world.on_player_connect(&mut player, &mut self.api);
                    self.connections.insert(id, Connection { open, websocket, player, ui });
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
                                if let Some(request) = R::deserialize(&bytes) {
                                    self.world.on_player_request(&mut connection.player, &request, &mut self.api);
                                }
                            },
                            _ => {}
                        }
                    },
                    Err(_) => {
                        self.world.on_player_disconnect(&mut connection.player, &mut self.api);
                        connection.open = false;
                    },
                }
            }

            self.connections.retain(|_id, connection| {
                connection.open
            });

            self.world.update();

            for (id, ui) in self.api.drain_items() {
                if let Some(connection) = self.connections.get_mut(&id) {
                    let bytes = ui.serialize();

                    connection.ui = Some(ui);
                    connection.websocket.write_message(Message::Binary(bytes)).unwrap();
                }
            }

            sleep(Duration::from_millis(5));
        }
    }
}