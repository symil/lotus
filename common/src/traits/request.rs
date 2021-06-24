use std::fmt::Debug;

use crate::serialization::serializable::Serializable;

pub trait Request : Debug + Serializable {
}