use std::{fmt::Debug, mem::take, rc::Rc};

use crate::{Logger, Transition, View, ViewState};

pub(crate) struct ClientViews<U, R, E, D> {
    pub hover_stack: Vec<ViewState<U, R, E, D>>,
    pub all: Vec<ViewState<U, R, E, D>>,
}

pub struct ClientApi<U, R, E, D> {
    pub user: U,
    pub local_data: D,
    pub hovered: Option<Rc<dyn View<U, R, E, D>>>,

    pub(crate) logger: Rc<dyn Logger>,
    pub(crate) hover_stack: Vec<ViewState<U, R, E, D>>,
    pub(crate) all_views: Vec<ViewState<U, R, E, D>>,
    pub(crate) outgoing_requests: Vec<R>,
    pub(crate) transitions_to_add: Vec<Box<dyn Transition<U, R, E, D>>>
}

impl<U, R, E, D> ClientApi<U, R, E, D>
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
            outgoing_requests: vec![],
            transitions_to_add: vec![]
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

    pub fn add_transition<T : Transition<U, R, E, D> + 'static>(&mut self, transition: T) {
        self.transitions_to_add.push(Box::new(transition));
    }

    pub(crate) fn take_views(&mut self) -> ClientViews<U, R, E, D> {
        ClientViews {
            hover_stack: take(&mut self.hover_stack),
            all: take(&mut self.all_views),
        }
    }

    pub(crate) fn set_views(&mut self, views: ClientViews<U, R, E, D>) {
        self.hover_stack = views.hover_stack;
        self.all_views = views.all;
    }
}