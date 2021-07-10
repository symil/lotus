// #[derive(Parsable)]
// pub enum TypeQualifier {
//     Struct = "struct",
//     View = "view"
// }

use parsable::*;

#[parsable(located)]
#[derive(Debug)]
pub enum TypeQualifier {
    Struct = "struct",
    View = "view"
}