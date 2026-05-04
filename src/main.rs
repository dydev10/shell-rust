#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let builtins = ["exit", "echo", "type"];

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Read command from user
        let mut user_command = String::new();
        match io::stdin().read_line(&mut user_command) {
            Ok(_) => {
                let command = user_command.trim();

                if command == "exit" {
                    break;
                } else if command.starts_with("echo ") {
                    let content = command.strip_prefix("echo ").unwrap();
                    println!("{}", content);
                } else if command.starts_with("type ") {
                    let content = command.strip_prefix("type ").unwrap();
                    if builtins.contains(&content) {
                        println!("{content} is a shell builtin",);
                    } else {
                        println!("{}: not found", content);
                    }
                } else {
                    println!("{}: command not found", user_command.trim());
                }
            }
            Err(error) => println!("Error {error}"),
        }
    }
}
