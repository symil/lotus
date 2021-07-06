#![allow(unused_variables)]
use std::{cell::RefCell, rc::Rc};

use crate::server_state::ServerState;

pub trait World<P, R, E> {
    fn on_start(&mut self, api: &mut ServerState<E>) { }
    fn on_player_connect(&mut self, api: &mut ServerState<E>, player: &Rc<RefCell<P>>) { }
    fn on_player_disconnect(&mut self, api: &mut ServerState<E>, player: &Rc<RefCell<P>>) { }
    fn on_player_request(&mut self, api: &mut ServerState<E>, player: &Rc<RefCell<P>>, request: &R) { }
    fn update(&mut self, api: &mut ServerState<E>) { }
}