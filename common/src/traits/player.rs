use std::fmt::Debug;

use crate::serialization::serializable::Serializable;

pub trait Player : Serializable + Clone + Debug + Default {
    fn from_id(id: u128) -> Self;
    fn get_id(&self) -> u128;
}