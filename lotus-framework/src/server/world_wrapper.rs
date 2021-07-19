use std::{collections::HashMap, marker::PhantomData};

use crate::{Entity, Link};

pub struct WorldWrapper<E> {
    // events_hooks: HashMap<u32, Link<dyn Entity<E>>>,
    _e: PhantomData<E>
}

impl<E> WorldWrapper<E> {
    pub fn trigger_event(event: E) {

    }
}