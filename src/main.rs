#[allow(unused_imports)]
use std::io::{self, Write};

use shell_rust::run;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Read command from user
        let mut user_command = String::new();
        match io::stdin().read_line(&mut user_command) {
            Ok(_) => {
                if run(user_command).is_err() {
                    break;
                }
            }
            Err(error) => println!("Error {error}"),
        }
    }
}
