use std::fmt;

pub fn display_join<T : fmt::Display>(values: &[T]) -> String {
    values.iter().map(|value| format!("`{}`", value)).collect::<Vec<String>>().join(" | ")
}