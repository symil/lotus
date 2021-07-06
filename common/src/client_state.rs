use std::{cell::RefCell, fmt::Debug, mem::take, rc::Rc};

use crate::{logger::Logger, traits::{view::View}};

pub struct ClientViews<P, R, E, D> {
    pub hovered: Option<Rc<dyn View<P, R, E, D>>>,
    pub hover_stack: Vec<Rc<dyn View<P, R, E, D>>>,
    pub all: Vec<Rc<dyn View<P, R, E, D>>>,
}

pub struct ClientState<P, R, E, D> {
    pub logger: Logger,
    pub user: Rc<RefCell<P>>,
    pub hovered: Option<Rc<dyn View<P, R, E, D>>>,
    pub hover_stack: Vec<Rc<dyn View<P, R, E, D>>>,
    pub all_views: Vec<Rc<dyn View<P, R, E, D>>>,
    pub local_data: D,
    pub outgoing_requests: Vec<R>,
}

impl<P : Default, R, E, D : Default> ClientState<P, R, E, D> {
    pub fn new(log_function: fn(&str)) -> Self {
        Self {
            logger: Logger::new(log_function),
            user: Rc::new(RefCell::new(P::default())),
            hovered: None,
            hover_stack: vec![],
            all_views: vec![],
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

    pub fn take_views(&mut self) -> ClientViews<P, R, E, D> {
        ClientViews {
            hovered: take(&mut self.hovered),
            hover_stack: take(&mut self.hover_stack),
            all: take(&mut self.all_views),
        }
    }

    pub fn set_views(&mut self, views: ClientViews<P, R, E, D>) {
        self.hovered = views.hovered;
        self.hover_stack = views.hover_stack;
        self.all_views = views.all;
    }
}