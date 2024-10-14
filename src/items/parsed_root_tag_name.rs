use parsable::parsable;

#[parsable]
pub enum ParsedRootTagName {
    DisableCheckMainType = "disable_check_main_type",
    EnableCheckFieldAccess = "enable_check_field_access"
}