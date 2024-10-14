#[macro_export]
macro_rules! wat {
    () => {
        Wat::default()
    };
    ($keyword:expr $(,$arg:expr)*) => {
        {
            let keyword = $keyword;
            let mut result = crate::program::Wat::from(keyword);
            $(
                {
                    result.extend($arg);
                }
            )*

            result
        }
    };
}

pub use wat;