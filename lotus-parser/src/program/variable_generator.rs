pub struct VariableGenerator {
    counter: u64
}

impl VariableGenerator {
    pub fn new() -> Self {
        Self { counter: 0 }
    }

    pub fn generate(&mut self, name: &str) -> String {
        self.counter += 1;

        format!("{}_{}", name, self.counter)
    }
}