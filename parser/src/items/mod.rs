#[macro_use]
macro_rules! item {
    (struct $ty:ident { $($field_name:ident : $field_type:ty),* } $name:ident => $expr:expr) => {
        #[derive(Debug)]
        pub struct $ty {
            $(pub $field_name: $field_type,)*
        }

        impl<'i> From<pest::iterators::Pair<'i, crate::grammar::Rule>> for $ty {
            fn from($name: pest::iterators::Pair<'i, crate::grammar::Rule>) -> Self {
                $expr
            }
        }
    };
    (enum $ty:ident { $($variant_name:ident $(( $($sub_type:ty),* ))? ),* } $name:ident => $expr:expr) => {
        #[derive(Debug)]
        pub enum $ty {
            $( $variant_name $( ($($sub_type),*) )? ,)*
        }

        impl<'i> From<pest::iterators::Pair<'i, crate::grammar::Rule>> for $ty {
            fn from($name: pest::iterators::Pair<'i, crate::grammar::Rule>) -> Self {
                $expr
            }
        }
    };
}

#[macro_use]
macro_rules! iterator {
    ($entry:expr) => {
        $entry.into_inner().peekable()
    }
}

#[macro_use]
macro_rules! parse {
    ($iterator:expr) => {
        $iterator.next().unwrap().into()
    };
    ($iterator:expr, $variant:ident) => {
        $iterator.next_if(|entry| match entry.as_rule() { crate::grammar::Rule::$variant => true, _ => false })
    };
}

#[macro_use]
macro_rules! parse_list {
    ($iterator:expr, $variant:ident) => {
        {
            let mut list = vec![];

            while let Some(entry) = parse!($iterator, $variant) {
                list.push(entry.into());
            }

            list
        }
    };
}

pub mod type_declaration;
pub mod identifier;
pub mod file;
pub mod type_qualifier;