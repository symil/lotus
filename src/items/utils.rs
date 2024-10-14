use parsable::{Parsable, ItemLocation};

use crate::program::ProgramContext;

pub fn unwrap_item<'a, T : Parsable>(item: &'a Option<T>, location: &ItemLocation, context: &mut ProgramContext) -> Option<&'a T> {
    match item.as_ref() {
        Some(value) => Some(value),
        None => {
            context.errors.expected_item::<T>(location);
            None
        },
    }
}