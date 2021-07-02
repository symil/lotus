pub struct StringReader<'a> {
    string: &'a str,
    index: usize
}

impl<'a> StringReader<'a> {
    pub fn new(string: &'a str) -> Self {
        Self {
            string,
            index: 0
        }
    }

    fn advance(&mut self, amount: usize) -> &'a str {
        let start = self.index;
        let end = self.index + amount;

        self.index = end;
        &self.string[start..end]
    }

    fn as_str(&self) -> &str {
        &self.string[self.index..]
    }

    fn as_char(&self) -> char {
        match self.string.as_bytes().first() {
            Some(byte) => *byte as char,
            None => 0 as char
        }
    }

    fn eat_spaces(&mut self) {
        while is_space(self.as_char()) {
            self.index += 1;
        }
    }

    pub fn location(&self) -> usize {
        self.index
    }

    pub fn read<F : Fn(&str) -> usize>(&mut self, f: F) -> Option<&'a str> {
        match f(self.as_str()) {
            0 => None,
            length => Some(self.advance(length))
        }
    }

    pub fn read_and_eat_spaces<F : Fn(&str) -> usize>(&mut self, f: F) -> Option<&'a str> {
        match self.read(f) {
            Some(value) => {
                self.eat_spaces();
                Some(value)
            },
            None => None
        }
    }
}

fn is_space(c: char) -> bool {
    match c {
        ' ' | '\n' | '\t' => true,
        _ => false
    }
}