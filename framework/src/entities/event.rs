pub trait Event {
    fn get_type_id(&self) -> u32;
}