#![allow(unused_imports)]
use lotus_common::{game::{game_request::GameRequest, game_world::GameWorld}, serialization::serializable::Serializable};
use server::Server;

pub mod server;

fn main() {
    // let world = GameWorld::new();
    // let mut server = Server::new(world);
    
    // let request = GameRequest::A;
    // let bytes = GameRequest::serialize(&request);
    // dbg!(&bytes);
    let result : Option<GameRequest> = GameRequest::deserialize(&[1]);
    dbg!(&result);

    // server.start();
}