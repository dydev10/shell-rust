#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::PathBuf;

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
                        let paths = std::env::var("PATH").unwrap();
                        let path_list: Vec<PathBuf> = std::env::split_paths(&paths).collect();
                        let mut is_found = false;
                        for path in path_list {
                            let exec_path = path.join(content);

                            let is_present = exec_path.exists();
                            let is_file = exec_path.is_file();
                            // let is_executable = exec_path.is_file();
                            // TODO: Add exec check
                            if is_present && is_file {
                                is_found = true;
                                println!("{content} is {}", exec_path.to_str().unwrap());
                                break;
                            }
                        }

                        if !is_found {
                            println!("{}: not found", content);
                        }
                    }
                } else {
                    println!("{}: command not found", user_command.trim());
                }
            }
            Err(error) => println!("Error {error}"),
        }
    }
}
