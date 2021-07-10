#![allow(unused_variables)]

use std::marker::PhantomData;

use crate::client_state::ClientState;

pub trait Transition<U, R, E, D> {
    fn get_duration(&self) -> f64;
    fn get_id(&self) -> u32 { 0 }

    fn on_start(&mut self, client: &mut ClientState<U, R, E, D>) { }
    fn on_end(&mut self, client: &mut ClientState<U, R, E, D>) { }
    fn on_progress(&mut self, client: &mut ClientState<U, R, E, D>, t: f64) { }
}

pub struct SimpleTransition<U, R, E, D, F>
    where
        F : FnMut(&mut ClientState<U, R, E, D>, f64)
{
    duration: f64,
    on_progress: F,
    _u: PhantomData<U>,
    _r: PhantomData<R>,
    _e: PhantomData<E>,
    _d: PhantomData<D>,
}

impl<U, R, E, D, F> SimpleTransition<U, R, E, D, F>
    where
        F : FnMut(&mut ClientState<U, R, E, D>, f64)
{
    pub fn new(duration: f64, on_progress: F) -> Self {
        Self {
            duration,
            on_progress,
            _u: PhantomData,
            _r: PhantomData,
            _e: PhantomData,
            _d: PhantomData
        }
    }
}

impl<U, R, E, D, F> Transition<U, R, E, D> for SimpleTransition<U, R, E, D, F>
    where 
        F : FnMut(&mut ClientState<U, R, E, D>, f64)
{
    fn get_duration(&self) -> f64 {
        self.duration
    }

    fn on_progress(&mut self, client: &mut ClientState<U, R, E, D>, t: f64) {
        (self.on_progress)(client, t);
    }
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