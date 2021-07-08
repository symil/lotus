use std::{fmt::Debug, mem::take, rc::Rc};

use crate::{logger::Logger, traits::{view::{View, ViewState}}};

pub struct ClientViews<U, R, E, D> {
    pub hover_stack: Vec<ViewState<U, R, E, D>>,
    pub all: Vec<ViewState<U, R, E, D>>,
}

pub struct ClientState<U, R, E, D> {
    pub logger: Rc<dyn Logger>,
    pub user: U,
    pub local_data: D,
    pub hovered: Option<Rc<dyn View<U, R, E, D>>>,

    // these fields should only be accessed internally
    pub hover_stack: Vec<ViewState<U, R, E, D>>,
    pub all_views: Vec<ViewState<U, R, E, D>>,
    pub outgoing_requests: Vec<R>,
}

impl<U, R, E, D> ClientState<U, R, E, D>
    where
        U : Default,
        D : Default
{
    pub fn new<L : Logger + 'static>(logger: L) -> Self {
        Self {
            logger: Rc::new(logger),
            user: U::default(),
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
        self.logger.log(&format!("{:?}", value));
    }

    pub fn log_time_start(&self, value: &str) {
        self.logger.log_time_start(value);
    }

    pub fn log_time_end(&self, value: &str) {
        self.logger.log_time_end(value);
    }

    pub fn send_request(&mut self, request: R) {
        self.outgoing_requests.push(request);
    }

    pub fn take_views(&mut self) -> ClientViews<U, R, E, D> {
        ClientViews {
            hover_stack: take(&mut self.hover_stack),
            all: take(&mut self.all_views),
        }
    }

    pub fn set_views(&mut self, views: ClientViews<U, R, E, D>) {
        self.hover_stack = views.hover_stack;
        self.all_views = views.all;
    }
}