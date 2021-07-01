use std::{fmt::Debug, rc::Rc};

use crate::{logger::Logger, traits::{local_data::LocalData, player::Player, request::Request, view::View}};

pub struct ClientState<P : Player, R : Request, D : LocalData> {
    pub logger: Logger,
    pub user: P,
    pub hovered: Option<Rc<dyn View<P, R, D>>>,
    pub local_data: D,
    pub outgoing_requests: Vec<R>,
}

impl<P : Player, R : Request, D : LocalData> ClientState<P, R, D> {
    pub fn new(log_function: fn(&str)) -> Self {
        Self {
            logger: Logger::new(log_function),
            user: P::from_id(0),
            hovered: None,
            local_data: D::default(),
            outgoing_requests: vec![]
        }
    }
    
    pub fn log(&self, value: &str) {
        self.logger.log(value);
    }

    pub fn log_value<T : Debug>(&self, value: &T) {
        self.logger.log_value(&format!("{:?}", value));
    }

    pub fn send_request(&mut self, request: R) {
        self.outgoing_requests.push(request);
    }
}