use std::borrow::Borrow;

use parsable::ItemLocation;

use crate::items::Identifier;
use super::Visibility;

pub trait GlobalItem {
    fn get_name(&self) -> &Identifier;
    fn get_visibility(&self) -> Visibility;
}