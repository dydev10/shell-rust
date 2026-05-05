#[allow(unused_imports)]
use std::io::{self, Write};
// #[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;

const BUILTINS: [&str; 5] = ["exit", "echo", "type", "pwd", "hwd"];

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

fn is_builtin_command(command: &str) -> bool {
    BUILTINS.contains(&command)
}

fn is_exec_command(command: &str) -> bool {
    let command_split: Vec<&str> = command.split(" ").collect();
    let exec_name = command_split[0];

    get_exec_path(exec_name).is_some()
}

fn run_builtin_command(program: &str, args: &[&str]) -> bool {
    match program {
        "exit" => true,
        "echo" => {
            //
            let content = args.join(" ");
            println!("{}", content);
            false
        }
        "type" => {
            //
            if BUILTINS.contains(&args[0]) {
                println!("{} is a shell builtin", &args[0]);
            } else {
                match get_exec_path(args[0]) {
                    Some(path) => println!("{} is {}", &args[0], path.to_str().unwrap()),
                    None => println!("{}: not found", &args[0]),
                }
            }
            false
        }
        "pwd" => {
            //
            match std::env::current_dir() {
                Ok(dir) => {
                    println!("{}", dir.to_str().unwrap());
                    false
                }
                Err(e) => {
                    eprintln!("pwd:: failed to read current dir: {}", e);
                    true // crash on unxpected error
                }
            }
        }
        "hwd" => {
            //
            match std::env::home_dir() {
                Some(dir) => {
                    println!("{}", dir.to_str().unwrap());
                    false
                }
                None => {
                    eprintln!("hwd:: no home dir found");
                    false
                }
            }
        }
        _ => true, // exit on unknown command send as builtin
    }
}

fn run_exec_command(program: &str, args: &[&str]) -> bool {
    Command::new(program)
        .args(args)
        .status()
        .expect("run_exec: failed to run executable");

    // to follow run command interface?
    false
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Read command from user
        let mut user_command = String::new();
        match io::stdin().read_line(&mut user_command) {
            Ok(_) => {
                let command = user_command.trim();
                let command_split: Vec<&str> = command.split(" ").collect();
                let program: &str = command_split[0];
                let args = &command_split[1..];

                if is_builtin_command(program) {
                    if run_builtin_command(program, args) {
                        // exit commands trigger this break
                        break;
                    };
                } else if is_exec_command(command) {
                    if run_exec_command(program, args) {
                        // not used but maybe later
                        break;
                    };
                } else {
                    println!("{}: command not found", user_command.trim());
                }
            }
            Err(error) => println!("Error {error}"),
        }
    }
}
