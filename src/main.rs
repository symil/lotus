use game::game_world::GameWorld;

use crate::server::Server;

pub mod graphics;
pub mod client_api;
pub mod server_api;
pub mod server;
pub mod game;
pub mod traits;

fn main() {
    let world = GameWorld::new();
    let mut server = Server::new(world);
    
    server.start();
}
