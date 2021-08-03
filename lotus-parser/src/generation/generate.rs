use crate::generation::Wat;

pub fn generate_wat() -> String {
    let mut expressions = vec![
        Wat::memory("memory", 100),
        Wat::function("main", Some("_start"), vec![], None, vec![
            Wat::const_i32(1),
            Wat::drop()
        ])
    ];

    // dbg!(&expressions);

    Wat::module(expressions).to_string()
}