// #[derive(Parsable)]
// pub enum TypeQualifier {
//     Struct = "struct",
//     View = "view"
// }

use lotus_parsable_macro::*;

#[parsable]
#[derive(Debug)]
pub enum TypeQualifier {
    Struct = "struct",
    View = "view"
}