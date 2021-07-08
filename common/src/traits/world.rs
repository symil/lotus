#![allow(unused_variables)]
use crate::server_state::ServerState;

pub type Id = u128;
pub type RequestResult = Result<Vec<Id>, ()>;

pub trait World<U, R, E> {
    fn on_start(&mut self, server: &mut ServerState<E>) { }
    fn on_user_connect(&mut self, server: &mut ServerState<E>, id: Id) { }
    fn on_user_disconnect(&mut self, server: &mut ServerState<E>, id: Id) { }
    fn on_user_request(&mut self, server: &mut ServerState<E>, id: Id, request: R) -> RequestResult { Err(()) }
    fn update(&mut self, server: &mut ServerState<E>) -> Vec<Id> { vec![] }

    fn get_user_state(&self, id: Id) -> U;
}