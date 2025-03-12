use std::{env, fs};
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut input;
    let path_vec: Vec<&str>;
    let mut paths: Vec<&str> = [].to_vec();
    // let commands= ["echo", "type", "exit"];
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        path_vec = args[1].split("=").collect();
        if path_vec.len() > 1 {
            paths = path_vec[1].split(":").collect();
        }
    }
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
            "type" => type_command(commands, &paths),
            "exit" => {
                if exit_command(commands) {
                    break;
                }
            },
            _ => not_found_err(commands,0),
        }
    }
}

fn type_command(commands: Vec<&str>, paths: &[&str]) {
    let mut isFound = false;
    for path in paths {
        if isFound {
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
                                    isFound = true;
                                    break;
                                }
                            }
                        }
                        Err(e) => println!("{}", e),
                    }
                }
            },
            Err(e) => println!("{}", e),
        }
    }
    if !isFound {
        println!("{}: not found ", commands[1]);
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