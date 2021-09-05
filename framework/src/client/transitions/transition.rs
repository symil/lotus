#![allow(unused_variables)]

use crate::ClientApi;

pub trait Transition<U, R, E, D> {
    fn get_duration(&self) -> f64;
    fn get_id(&self) -> u32 { 0 }

    fn on_start(&mut self, client: &mut ClientApi<U, R, E, D>) { }
    fn on_end(&mut self, client: &mut ClientApi<U, R, E, D>) { }
    fn on_progress(&mut self, client: &mut ClientApi<U, R, E, D>, t: f64) { }
}

pub struct TransitionWrapper<U, R, E, D> {
    pub transition: Box<dyn Transition<U, R, E, D>>,
    pub id: u32,
    pub started: bool,
    pub ended: bool,
    pub start_time: f64,
    pub duration: f64,
}

impl<U, R, E, D> TransitionWrapper<U, R, E, D> {
    pub fn new(transition: Box<dyn Transition<U, R, E, D>>, id: u32) -> Self {
        Self {
            transition,
            id,
            started: false,
            ended: false,
            start_time: 0.,
            duration: 0.,
        }
    }
}