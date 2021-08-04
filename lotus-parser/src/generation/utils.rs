#[macro_export]
macro_rules! merge {
    ($($vec:expr),*) => {
        {
            let mut result = vec![];

            $(
                result.extend($vec);
            )*

            result
        }
    };
}

pub use merge;