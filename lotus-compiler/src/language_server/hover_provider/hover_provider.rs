use std::collections::HashMap;
use parsable::ItemLocation;
use crate::{program::{CursorLocation, Type, Cursor}, language_server::{is_invalid_location}};
use super::{Hover};

pub struct HoverProvider {
    pub cursor: Cursor,
    pub hover: Option<Hover>
}

impl HoverProvider {
    pub fn new(cursor: &Cursor) -> Self {
        Self {
            cursor: cursor.clone(),
            hover: None,
        }
    }

    fn modify<F : FnOnce(&mut Hover)>(&mut self, location: &ItemLocation, callback: F) {
        if !self.cursor.is_on_location(location) {
            return;
        }

        let hover = match &mut self.hover {
            Some(hover) => match &hover.location == location {
                true => hover,
                false => {
                    *hover = Hover::new(location);
                    hover
                },
            },
            None => self.hover.insert(Hover::new(location)),
        };

        callback(hover);
    }

    pub fn set_type(&mut self, location: &ItemLocation, ty: &Type) {
        self.modify(location, |hover| hover.ty = Some(ty.clone()));
    }

    pub fn get_hover(&self) -> Option<&Hover> {
        self.hover.as_ref()
    }
}