#[allow(unused_imports)]
use std::io::{self, Write};
// #[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;

const BUILTINS: [&str; 6] = [
    "exit", "echo", "type", "pwd", "cd", /* Extra builtins by me */ "hwd",
];
const ARG_SEPARATOR: char = ' ';

#[derive(PartialEq)]
enum CommandParseState {
    Normal,
    InSingleQuote,
    InDoubleQuote,
}

fn tokenize(input: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut state = CommandParseState::Normal;

    for c in input.chars() {
        match (&state, c) {
            (CommandParseState::Normal, '\'') => state = CommandParseState::InSingleQuote,
            (CommandParseState::Normal, '"') => state = CommandParseState::InDoubleQuote,
            (CommandParseState::Normal, ARG_SEPARATOR) => {
                // push the current token and start new token
                if !&current.is_empty() {
                    tokens.push(String::from(&current));
                    current.clear();
                }
            }

            (CommandParseState::InSingleQuote, '\'') => state = CommandParseState::Normal,

            (CommandParseState::InDoubleQuote, '"') => state = CommandParseState::Normal,

            (_, ch) => current.push(ch),
        }
    }

    if !current.is_empty() {
        tokens.push(String::from(&current));
    }

    tokens
}

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

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Read command from user
        let mut user_command = String::new();
        match io::stdin().read_line(&mut user_command) {
            Ok(_) => {
                let command = user_command.trim();
                if command.is_empty() {
                    continue;
                }

                let command_tokens = tokenize(command);
                let program: &str = command_tokens[0].as_str();
                let args: Vec<&str> = command_tokens[1..].iter().map(|arg| arg.as_str()).collect();

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
