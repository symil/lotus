use std::fmt::Debug;
use crate::{logger::Logger, traits::request::Request};

#[derive(Debug)]
pub struct ClientApi<R : Request> {
    logger: Logger,
    pending_requests: Vec<R>
}

impl<R : Request> ClientApi<R> {
    pub fn new(logger: Logger) -> Self {
        Self {
            logger,
            pending_requests: vec![]
        }
    }

    pub fn send_request(&mut self, request: R) {
        self.pending_requests.push(request);
    }

    pub fn poll_requests(&mut self) -> Vec<R> {
        self.pending_requests.drain(..).collect()
    }

    pub fn log(&self, value: &str) {
        self.logger.log(value);
    }

    pub fn log_value<T : Debug>(&self, value: &T) {
        self.logger.log_value(&format!("{:?}", value));
    }
}