use crate::server::Server;

pub mod graphics;
pub mod entity;
pub mod client_api;
pub mod world;
pub mod server_api;
pub mod server;

fn main() {
    let mut server = Server::new();
    
    server.start();
}
