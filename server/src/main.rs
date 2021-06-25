#![allow(unused_imports)]
use lotus_common::{game::{game_request::GameRequest, game_world::GameWorld}, serialization::serializable::Serializable};
use server::Server;

pub mod server;

fn main() {
    // let world = GameWorld::new();
    // let mut server = Server::new(world);
    
    // let request = GameRequest::D("A".to_string(), "some very long string".to_string(), "LAST ONE".to_string());
    // let request = GameRequest::E(1, 2, 3, 4, 5);
    let request = GameRequest::H([1, 2, 3, 4, 5], "foo".to_string());
    let bytes = GameRequest::serialize(&request);
    dbg!(&bytes);
    let result : Option<GameRequest> = GameRequest::deserialize(&bytes);
    dbg!(&result);

    // server.start();
}