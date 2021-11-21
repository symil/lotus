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

#[macro_export]
macro_rules! vasm {
    ($($arg:expr),*) => {
        {
            let mut result = crate::program::Vasm::void();
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
pub use vasm;