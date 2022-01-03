pub fn lookup_line_col(_file_path: &str, file_content: &str, index: usize) -> (usize, usize) {
    // let global_hashmap = unsafe { LINE_COL_LOOKUPS.get_or_insert_with(|| HashMap::new()) };

    // match global_hashmap.get(file_path) {
    //     Some(line_col_lookup) => line_col_lookup.get(index),
    //     None => {
    //         let line_col_lookup = LineColLookup::new(file_content);
    //         let result = line_col_lookup.get(index);

    //         global_hashmap.insert(file_path, line_col_lookup);
    //         result
    //     },
    // }

    // TODO: cache result
    let line_col_lookup = LineColLookup::new(file_content);

    line_col_lookup.get(index)
}

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