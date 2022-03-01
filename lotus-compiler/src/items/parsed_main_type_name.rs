use parsable::parsable;

#[parsable(name = "main type name")]
pub enum ParsedMainTypeName {
    User = "USER_TYPE",
    World = "WORLD_TYPE",
    Window = "WINDOW_TYPE",
    LocalData = "LOCAL_DATA_TYPE",
}