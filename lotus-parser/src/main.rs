use program::LotusProgram;

mod program;
mod items;

fn main() {
    match LotusProgram::from_directory_path("test") {
        Ok(_) => {
            println!("parse ok");
        },
        Err(errors) => {
            for error in errors {
                println!("{}", error.to_string());
            }
        }
    }
}