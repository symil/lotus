use crate::{logger::Logger, traits::{player::Player, request::Request, view::View}};

pub struct ClientState<P : Player, R : Request> {
    pub logger: Logger,
    pub user: P,
    pub hovered: Option<Box<dyn View<P, R>>>,
}

impl<P : Player, R : Request> ClientState<P, R> {
    pub fn new(log_function: fn(&str)) -> Self {
        Self {
            logger: Logger::new(log_function),
            user: P::from_id(0),
            hovered: None,
        }
    }
}