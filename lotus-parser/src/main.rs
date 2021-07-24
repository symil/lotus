use program::LotusProgram;

mod program;
mod items;
mod definitions;
pub mod context;
pub mod error;
mod constants;

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