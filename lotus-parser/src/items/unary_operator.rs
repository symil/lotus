use parsable::parsable;

#[parsable(impl_display=true)]
pub enum UnaryOperator {
    Not = "!",
    Plus = "+",
    Minus = "-"
}