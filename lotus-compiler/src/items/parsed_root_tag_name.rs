use parsable::parsable;

#[parsable]
pub enum ParsedRootTagName {
    DisableMainTypeChecks = "disable_main_type_checks",
    CheckFieldAccess = "check_field_access"
}