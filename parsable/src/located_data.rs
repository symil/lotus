use std::ops::{Deref, DerefMut};

use crate::{Parsable, string_reader::StringReader};

#[derive(Debug, Default, Clone)]
pub struct DataLocation {
    pub start: usize,
    pub end: usize,
    // pub file_name: &'static str,
    // pub line: usize,
    // pub column: usize
}

impl DataLocation {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

pub type Ld<T> = LocatedData<T>;

#[derive(Debug, Default)]
pub struct LocatedData<T> {
    pub data_internal__: T,
    pub location_internal__: DataLocation,
}

impl<T : Parsable> LocatedData<T> {
    pub fn data(value: &Self) -> &T {
        &value.data_internal__
    }

    pub fn loc(value: &Self) -> &DataLocation {
        &value.location_internal__
    }
}

impl<T : Parsable> Parsable for LocatedData<T> {
    fn parse(reader: &mut StringReader) -> Option<Self> {
        let start = reader.get_index();

        match T::parse(reader) {
            Some(value) => {
                let end = reader.get_index();

                Some(Self {
                    data_internal__: value,
                    location_internal__: DataLocation { start, end }
                })
            },
            None => None
        }
    }
}

impl<T> Deref for LocatedData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data_internal__
    }
}

impl<T> DerefMut for LocatedData<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data_internal__
    }    
}