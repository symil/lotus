use crate::{traits::request::Request};
use crate::serialization::serializable::Serializable;

#[derive(Debug, Clone, Serializable)]
pub struct GameRequest {
    pub a: u8,
    pub b: u8
}

impl Request for GameRequest {

}