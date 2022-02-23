#[derive(Debug, Clone)]
pub struct FieldCompletionOptions {
    pub show_methods: bool,
    pub insert_arguments: bool,
    pub hide_private: bool,
    pub prefix: &'static str,
    pub suffix: &'static str,
}

impl Default for FieldCompletionOptions {
    fn default() -> Self {
        Self {
            show_methods: false,
            insert_arguments: false,
            hide_private: false,
            prefix: "",
            suffix: "",
        }
    }
}