use std::error::Error;
#[allow(unused_imports)]
use std::io::{self, Write};
// #[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;

mod interpreter;
mod tokenizer;

const BUILTINS: [&str; 6] = [
    "exit", "echo", "type", "pwd", "cd", /* Extra builtins by me */ "hwd",
];
const ARG_SEPARATOR: char = ' ';

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

fn is_exec_command(program: &str) -> bool {
    get_exec_path(program).is_some()
}

fn run_builtin_command(program: &str, args: Vec<&str>) -> bool {
    let mut exit = false;
    match program {
        "exit" => exit = true,
        "echo" => {
            //
            println!("{}", args.join(ARG_SEPARATOR.to_string().as_str()));
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
        }
        "pwd" => {
            //
            match std::env::current_dir() {
                Ok(dir) => {
                    println!("{}", dir.to_str().unwrap());
                }
                Err(e) => {
                    eprintln!("pwd:: failed to read current dir: {}", e);
                    exit = true; // crash on unxpected error
                }
            }
        }
        "cd" => {
            //
            if args[0] == "~" {
                match std::env::home_dir() {
                    Some(home) => match std::env::set_current_dir(home) {
                        Ok(_) => (),
                        Err(_) => println!("cd:: ~:: failed to switch to home dir"),
                    },
                    None => {
                        eprintln!("cd:: ~:: home dir not found, ignoring cd");
                    }
                }
            } else {
                // set_current_dir handles relative paths
                match std::env::set_current_dir(args[0]) {
                    Ok(_) => (),
                    Err(_) => {
                        println!("cd: {}: No such file or directory", &args[0]);
                    }
                }
            }
        }

        // extra builtins
        "hwd" => {
            //
            match std::env::home_dir() {
                Some(dir) => {
                    println!("{}", dir.to_str().unwrap());
                }
                None => {
                    eprintln!("hwd:: no home dir found");
                }
            }
        }

        _ => exit = true, // exit on unknown command send as builtin
    };

    exit
}

fn run_exec_command(program: &str, args: Vec<&str>) -> bool {
    Command::new(program)
        .args(args)
        .status()
        .expect("run_exec: failed to run executable");

    // to follow run command interface?
    false
}

pub fn run(input: String) -> Result<(), Box<dyn Error>> {
    let command = input.trim();
    if command.is_empty() {
        return Ok(());
    }

    let command_tokens = tokenizer::tokenize(command);
    let program: &str = command_tokens[0].as_str();
    let args: Vec<&str> = command_tokens[1..].iter().map(|arg| arg.as_str()).collect();

    if is_builtin_command(program) {
        if run_builtin_command(program, args) {
            // exit commands triggers Error to exit main loop
            return Err("exit".into());
        };
    } else if is_exec_command(program) {
        if run_exec_command(program, args) {
            // not used but maybe later
            return Err("unknown".into());
        };
    } else {
        println!("{}: command not found", input.trim());
    }

    Ok(())
}
