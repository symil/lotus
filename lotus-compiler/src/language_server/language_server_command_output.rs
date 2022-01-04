use super::COMMAND_SEPARATOR;

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

    pub fn consume(mut self) -> String {
        self.flush();
        self.lines.insert(0, self.id.to_string());
        self.lines.join("\n")
    }
}