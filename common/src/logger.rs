pub trait Logger {
    fn log(&self, value: &str);
    fn log_time_start(&self, label: &str);
    fn log_time_end(&self, label: &str);
}