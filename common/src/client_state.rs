use std::{fmt::Debug, marker::PhantomData};

use crate::{logger::Logger, traits::{player::Player, request::Request, view::View}};

#[derive(Clone, Debug)]
pub struct ClientState<P : Player, R : Request, V : View<P, R>> {
    pub logger: Logger,
    pub user: P,
    pub hovered: V,
    _r: PhantomData<R>
}

impl<P : Player, R : Request, V : View<P, R>> ClientState<P, R, V> {
    pub fn new(log_function: fn(&str)) -> Self {
        Self {
            logger: Logger::new(log_function),
            user: P::default(),
            hovered: V::none(),
            _r: PhantomData
        }
    }
}