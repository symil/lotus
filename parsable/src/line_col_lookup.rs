pub struct LineColLookup {
    pub lookup: Vec<(usize, usize)>
}

impl LineColLookup {
    pub fn new(string: &str) -> Self {
        let mut lookup = Vec::with_capacity(string.len() + 1);
        let mut line = 1;
        let mut col = 1;
        
        for byte in string.as_bytes() {
            let c = *byte as char;

            lookup.push((line, col));

            if c == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }

        lookup.push((line, col));

        Self { lookup }
    }

    pub fn get(&self, index: usize) -> (usize, usize) {
        self.lookup[index]
    }
}