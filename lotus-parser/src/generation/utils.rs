#[macro_export]
macro_rules! wat {
    () => {
        Wat::default()
    };
    ($keyword:expr $(,$arg:expr)*) => {
        {
            let keyword = $keyword;
            let mut result = keyword.to_wat();
            $(
                {
                    let arg = $arg;
                    result.push(arg);
                }
            )*

            result
        }
    };
}

#[macro_export]
macro_rules! merge {
    ($($vec:expr),*) => {
        {
            let mut result = vec![];

            $(
                result.extend($vec.to_wat_vec());
            )*

            result
        }
    };
}

pub use wat;
pub use merge;