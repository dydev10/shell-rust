#[allow(unused_imports)]
use std::io::{self, Write};
// #[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;

fn is_exec(path: &PathBuf) -> bool {
    // #[cfg(unix)]
    match std::fs::metadata(path) {
        Ok(metadata) => {
            let mode = metadata.permissions().mode();

            let owner_exec = mode & 0o100 != 0; // bit 6
            let group_exec = mode & 0o010 != 0; // bit 3
            let other_exec = mode & 0o001 != 0; // bit 0

            owner_exec || group_exec || other_exec
        }
        Err(_) => false,
    }

    // #[cfg(windows)]
    // {
    //     let pathext =
    //         std::env::var("PATHEXT").unwrap_or_else(|_| ".COM;.EXE;.BAT;.CMD".to_string());
    //     let exec_extensions: Vec<&str> = pathext.split(";").collect();
    //     let file_ext = path.extension().and_then(|e| e.to_str());
    //     match file_ext {
    //         Some(ext) => {
    //             let uppercased = ext.to_ascii_uppercase();
    //             let ext_with_dot = format!(".{uppercased}");
    //             exec_extensions
    //                 .iter()
    //                 .any(|e| e.eq_ignore_ascii_case(&ext_with_dot))
    //         }
    //         None => false,
    //     }
    // }
}

fn get_exec_path(exec: &str) -> Option<PathBuf> {
    let paths = std::env::var("PATH").unwrap();
    let path_list: Vec<PathBuf> = std::env::split_paths(&paths).collect();
    let mut found = None;
    for path in path_list {
        let exec_path = path.join(exec);
        let is_executable = is_exec(&exec_path);
        if is_executable {
            found = Some(exec_path);
            break;
        }
    }
    found
}

fn is_exec_command(command: &str) -> bool {
    let command_split: Vec<&str> = command.split(" ").collect();
    let exec_name = command_split[0];

    get_exec_path(exec_name).is_some()
}

fn run_exec(args: Vec<&str>) {
    Command::new(args[0])
        .args(&args[1..])
        .status()
        .expect("run_exec: failed to run executable");
}

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
                        match get_exec_path(content) {
                            Some(path) => println!("{content} is {}", path.to_str().unwrap()),
                            None => println!("{}: not found", content),
                        }
                    }
                } else if is_exec_command(command) {
                    let args: Vec<&str> = command.split(" ").collect();
                    run_exec(args);
                } else {
                    println!("{}: command not found", user_command.trim());
                }
            }
            Err(error) => println!("Error {error}"),
        }
    }
}
