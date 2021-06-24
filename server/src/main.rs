#![allow(unused_imports)]
use lotus_common::{game::{game_request::GameRequest, game_world::GameWorld}, serialization::serializable::Serializable};
use server::Server;

pub mod server;

fn main() {
    // let world = GameWorld::new();
    // let mut server = Server::new(world);
    
    let request = GameRequest { a: 6, b: 8 };
    let bytes = GameRequest::serialize(&request);
    dbg!(&bytes);
    // let result : Option<GameRequest> = GameRequest::deserialize(&bytes);
    // dbg!(&result);

    // server.start();
}