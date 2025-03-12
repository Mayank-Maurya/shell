use std::{env, fs, os::unix::process::CommandExt};
#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;

const built_in_commands: [&str; 3] = ["echo", "exit", "type"];
fn main() {
    // define vars
    let stdin = io::stdin();
    let mut input;
    loop {
        // initiate terminal
        print!("$ ");
        io::stdout().flush().unwrap();

        // take input
        input = String::new();
        stdin.read_line(&mut input).unwrap();

        // collect all args with command at args[0]
        let commands: Vec<&str> = input.split_whitespace().collect();
        
        // check if there any command
        if commands.len() == 0 {
            continue;
        }

        // check for type of commands
        match commands[0].trim() {
            "echo" => echo_command(commands),
            "type" => type_command(commands),
            "exit" => {
                if exit_command(commands) {
                    break;
                }
            },
            _ => execute_files_command(commands),
        }
    }
}

fn type_command(commands: Vec<&str>) {
    let mut paths: Vec<&str> = [].to_vec();
    let p: String;
    // Get PATH from env vars
    match env::var("PATH") {
        Ok(path) => {
            p = path.clone();
            paths = p.split(":").collect();
        },
        Err(e) => println!("Couldn't read PATH: {}", e),
    }
    if built_in_commands.contains(&&commands[1])  {
        println!("{} is a shell builtin", commands[1]);
        return;
    }
    let mut is_found: bool = false;
    for path in paths {
        if is_found {
            break;
        }
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            if let Some(file_name) = entry.path().file_stem() {
                                if file_name == commands[1] {
                                    println!("{} is {}/{}", commands[1],path,file_name.to_string_lossy());
                                    is_found = true;
                                    break;
                                }
                            }
                        }
                        Err(e) => {},
                    }
                }
            },
            Err(e) => {},
        }
    }
    if !is_found {
        not_found_err(commands, 1);
        // println!("{}: not found ", commands[1]);
    }
}

fn execute_files_command(commands: Vec<&str>) {
    let mut paths: Vec<&str> = [].to_vec();
    let p: String;
    // Get PATH from env vars
    match env::var("PATH") {
        Ok(path) => {
            p = path.clone();
            paths = p.split(":").collect();
        },
        Err(e) => println!("Couldn't read PATH: {}", e),
    }
    let mut is_found: bool = false;
    for path in paths {
        if is_found {
            break;
        }
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            if let Some(file_name) = entry.path().file_stem() {
                                if file_name == commands[0] {
                                    println!("{} is {}/{}", commands[0],path,file_name.to_string_lossy());
                                    Command::new(entry.path())
                                        .args(commands.iter())
                                        .spawn();
                                    // println!("{:?}", comnd.unwrap().stdout);
                                    is_found = true;
                                    break;
                                }
                            }
                        }
                        Err(e) => {},
                    }
                }
            },
            Err(e) => {},
        }
    }
    if !is_found {
        not_found_err(commands, 1);
        // println!("{}: not found ", commands[1]);
    }
}

fn exit_command(commands: Vec<&str>) -> bool {
    if commands.len() >=2 && commands[1] == "0" {
        return true
    }
    not_found_err(commands, 1);
    false
}

fn echo_command(commands: Vec<&str>) {
    println!("{} ",commands[1..].join(" "));
}

fn not_found_err(commands: Vec<&str>, start_index: usize) {
    println!("{}: command not found ",commands[start_index..].join(" "));
}