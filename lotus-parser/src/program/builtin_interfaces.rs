#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinInterface {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Shl,
    Shr,
    And,
    Or,
    Eq,
    Ne,
    Ge,
    Gt,
    Le,
    Lt,
    ToBool
}