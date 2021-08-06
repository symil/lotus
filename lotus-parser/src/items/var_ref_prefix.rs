use parsable::parsable;

#[parsable(impl_display=true)]
#[derive(PartialEq, Copy)]
pub enum VarRefPrefix {
    This = "#",
    Payload = "$",
    System = "@"
}