use std::ops::{Deref, DerefMut};

use pest::{RuleType, iterators::Pair};

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

impl<'i, R : RuleType, T : From<Pair<'i, R>>> From<Pair<'i, R>> for LocatedData<T> {
    fn from(entry: Pair<'i, R>) -> Self {
        LocatedData {
            location: DataLocation {
                index: entry.as_span().start(),
                file_name: "",
                line: 0,
                column: 0
            },
            data: T::from(entry)
        }
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