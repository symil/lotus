use program::LotusProgram;

mod program;
mod items;
pub mod context;

fn main() {
    match LotusProgram::from_directory("test") {
        Ok(program) => {
            dbg!(program);
        },
        Err(error) => {
            println!("{}", error.to_string());
        }
    }
}