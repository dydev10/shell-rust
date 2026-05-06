#[allow(unused_imports)]
use std::io::{self, Write};

mod shell;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Read command from user
        let mut user_command = String::new();
        match io::stdin().read_line(&mut user_command) {
            Ok(_) => {
                if shell::run(user_command).is_err() {
                    break;
                }
            }
            Err(error) => println!("Error {error}"),
        }
    }
}
