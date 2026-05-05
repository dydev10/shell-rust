#[allow(unused_imports)]
use std::io::{self, Write};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

fn is_exec(path: &PathBuf) -> bool {
    #[cfg(unix)]
    match std::fs::metadata(path) {
        Ok(metadata) => {
            let mode = metadata.permissions().mode();

            let owner_exec = mode & 0o100 != 0; // bit 6
            let group_exec = mode & 0o010 != 0; // bit 3
            let other_exec = mode & 0o001 != 0; // bit 0

            owner_exec || group_exec || other_exec
        }
        Err(_) => {
            eprintln!("is_exec:: Can't read metadata");
            false
        }
    }

    #[cfg(windows)]
    {
        let pathext =
            std::env::var("PATHEXT").unwrap_or_else(|_| ".COM;.EXE;.BAT;.CMD".to_string());
        let exec_extensions: Vec<&str> = pathext.split(";").collect();
        let file_ext = path.extension().and_then(|e| e.to_str());
        match file_ext {
            Some(ext) => {
                let uppercased = ext.to_ascii_uppercase();
                let ext_with_dot = format!(".{uppercased}");
                exec_extensions
                    .iter()
                    .any(|e| e.eq_ignore_ascii_case(&ext_with_dot))
            }
            None => {
                eprintln!("is_exec:: Can't read extension");
                false
            }
        }
    }
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
                        let paths = std::env::var("PATH").unwrap();
                        let path_list: Vec<PathBuf> = std::env::split_paths(&paths).collect();
                        let mut is_found = false;
                        for path in path_list {
                            let exec_path = path.join(content);

                            let is_present = exec_path.exists();
                            let is_file = exec_path.is_file();
                            if is_present && is_file {
                                let is_executable = is_exec(&exec_path);
                                if is_executable {
                                    is_found = true;
                                    println!("{content} is {}", exec_path.to_str().unwrap());
                                    break;
                                }
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
