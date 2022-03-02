use super::{COMMAND_SEPARATOR, COMMAND_OUTPUT_ITEM_LINE_START};

pub struct LanguageServerCommandOutput {
    id: u32,
    lines: Vec<String>,
    current_line: Vec<String>
}

impl LanguageServerCommandOutput {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            lines: vec![],
            current_line: vec![]
        }
    }

    fn flush(&mut self) {
        if !self.current_line.is_empty() {
            self.lines.push(self.current_line.join(COMMAND_SEPARATOR));
            self.current_line.clear();
        }
    }

    pub fn line(&mut self, kind: &str) -> &mut Self {
        self.flush();
        self.current_line.push(kind.to_string());
        self
    }

    pub fn push<T : ToString>(&mut self, value: T) -> &mut Self {
        self.current_line.push(value.to_string());
        self
    }

    pub fn push_opt<T : ToString>(&mut self, value: Option<&T>) -> &mut Self {
        let content = match value {
            Some(v) => v.to_string(),
            None => String::new(),
        };
        self.current_line.push(content);

        self
    }

    pub fn format(mut self, message: Option<String>) -> String {
        let mut header = self.id.to_string();

        if let Some(msg) = message {
            header.push_str(&format!(" {}", msg));
        }

        self.flush();
        self.lines.insert(0, header);
        self.lines.join(COMMAND_OUTPUT_ITEM_LINE_START)
    }
}