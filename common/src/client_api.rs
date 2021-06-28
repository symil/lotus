use crate::traits::request::Request;

#[derive(Debug)]
pub struct ClientApi<R : Request> {
    pending_requests: Vec<R>
}

impl<R : Request> ClientApi<R> {
    pub fn new() -> Self {
        Self {
            pending_requests: vec![]
        }
    }

    pub fn send_request(&mut self, request: R) {
        self.pending_requests.push(request);
    }

    pub fn poll_requests(&mut self) -> Vec<R> {
        self.pending_requests.drain(..).collect()
    }
}