use pest::{RuleType, iterators::Pair};

#[derive(Debug, Default)]
pub struct DataLocation {
    pub index: usize,
    pub file_id: usize,
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
                file_id: 0,
                line: 0,
                column: 0
            },
            data: T::from(entry)
        }
    }
}