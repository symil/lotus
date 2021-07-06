#![allow(unused_variables)]
use std::{cell::RefCell, rc::Rc};

use crate::server_api::ServerApi;

pub trait World<P, R, E> {
    fn on_start(&mut self, api: &mut ServerApi<E>) { }
    fn on_player_connect(&mut self, api: &mut ServerApi<E>, player: &Rc<RefCell<P>>) { }
    fn on_player_disconnect(&mut self, api: &mut ServerApi<E>, player: &Rc<RefCell<P>>) { }
    fn on_player_request(&mut self, api: &mut ServerApi<E>, player: &Rc<RefCell<P>>, request: &R) { }
    fn update(&mut self, api: &mut ServerApi<E>) { }
}