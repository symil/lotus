use std::ops::{Deref, DerefMut};

use crate::{Parsable, string_reader::StringReader};

#[derive(Debug, Default)]
pub struct DataLocation {
    pub index: usize,
    pub file_name: &'static str,
    pub line: usize,
    pub column: usize
}

#[derive(Debug, Default)]
pub struct LocatedData<T> {
    pub data: T,
    pub location: DataLocation,
}

impl<T : Parsable> Parsable for LocatedData<T> {
    fn parse(_reader: &mut StringReader) -> Option<Self> {
        todo!()
    }
}

impl<T> Deref for LocatedData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for LocatedData<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }    
}