#[derive(Debug, Clone)]
pub struct FieldCompletionOptions {
    pub show_fields: bool,
    pub show_methods: bool,
    pub insert_arguments: bool,
    pub insert_dynamic_methods: bool,
    pub hide_private: bool,
    pub prefix: &'static str,
    pub suffix: &'static str,
}

impl Default for FieldCompletionOptions {
    fn default() -> Self {
        Self {
            show_fields: true,
            show_methods: false,
            insert_arguments: false,
            insert_dynamic_methods: false,
            hide_private: false,
            prefix: "",
            suffix: "",
        }
    }
}