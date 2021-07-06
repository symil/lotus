use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::{logger::Logger, traits::{view::View}};

pub struct ClientState<P, R, D> {
    pub logger: Logger,
    pub user: Rc<RefCell<P>>,
    pub hovered: Option<Rc<dyn View<P, R, D>>>,
    pub local_data: D,
    pub outgoing_requests: Vec<R>,
}

impl<P : Default, R, D : Default> ClientState<P, R, D> {
    pub fn new(log_function: fn(&str)) -> Self {
        Self {
            logger: Logger::new(log_function),
            user: Rc::new(RefCell::new(P::default())),
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