use std::{env, fs::{self}};
use std::io::{self, Write};
use std::process::Command;

const BUILT_IN_COMMANDS: [&str; 4] = ["echo", "exit", "type", "pwd"];
fn main() {
    // define vars
    let stdin = io::stdin();
    let mut input;
    let mut command;
    let mut args;
    loop {
        // initiate terminal
        print!("$ ");
        io::stdout().flush().unwrap();

        // take input
        input = String::new();
        stdin.read_line(&mut input).unwrap();

        let split = input.split_once(" ");

        match split {
            Some(str) => {
                command = str.0;
                args = str.1;
            },
            None => {
                command = input.trim();
                args = "";
            }
        }

        // // collect all args with command at args[0]
        let commands: Vec<&str> = input.split_ascii_whitespace().collect();

        // check for type of commands
        match command {
            "echo" => echo_command(args),
            "cat" => cat_command(args),
            "type" => type_command(commands),
            "pwd" => pwd_command(commands),
            "cd" => change_directory_command(commands),
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
    if BUILT_IN_COMMANDS.contains(&&commands[1])  {
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
        // not_found_err(commands, 1);
        println!("{}: not found ", commands[1]);
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
                                    async_execute_file( &commands,"");
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
        not_found_err(commands, 0);
        // println!("{}: not found ", commands[1]);
    }
}

fn async_execute_file(commands: &[&str], explicit_command: &str) {
    let cmd;
    if !explicit_command.is_empty() {
        cmd = explicit_command;
    } else {
        cmd = commands[0];
    }
    let output = Command::new(cmd)
        .args(commands[1..].iter())
        .output();
    io::stdout().write_all(&output.unwrap().stdout).unwrap();
}

fn exit_command(commands: Vec<&str>) -> bool {
    if commands.len() >=2 && commands[1] == "0" {
        return true
    }
    not_found_err(commands, 1);
    false
}

fn pwd_command(commands: Vec<&str>) {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => println!("{}", e),
    }
}

fn change_directory_command(commands: Vec<&str>) {
    if commands[1] == "~" {
        match env::var("HOME") {
            Ok(path) => {
                match env::set_current_dir(path) {
                    Ok(_) => {},
                    Err(_) => {},
                }
            },
            Err(e) => println!("Couldn't read PATH: {}", e),
        }
        return;
    }
    match env::set_current_dir(commands[1]) {
        Ok(_) => {},
        Err(_) => println!("cd: {}: No such file or directory", commands[1]),
    }
}

fn echo_command(mut input: &str) {
    input = input.trim();
    let mut ans: Vec<&str>;
    if input[0..1].to_string() == "'" {
        ans = input.split("'").collect();
    } else if input[0..1].to_string() == "\"" { 
        ans = input.split("\"").collect();
    } else {
        ans = input.split_ascii_whitespace().collect();
        println!("{}", ans.join(" "));
        return
    }
    ans = remove_white_spaces(&ans);
    println!("{}", ans.join(""));
}

fn cat_command(input: &str) {
    let file_names: Vec<&str>;
    if input[0..1].to_string() == "'" {
        file_names = input.split("'").collect();
    } else if input[0..1].to_string() == "\"" { 
        file_names = input.split("\"").collect();
    } else {
        file_names = input.split_whitespace().collect();
    }
    async_execute_file(&file_names, "cat");
}

fn remove_white_spaces<'a>(ans: &[&'a str]) -> Vec<&'a str> {
    let mut result: Vec<&str> = [].to_vec();
    for item in ans {
        if item.len() == 0 {
            continue;
        }
        let mut trim = item.trim();
        if trim.len() == 0 {
            trim = " ";
        }
        result.push(trim);
    }
    result
}

fn not_found_err(commands: Vec<&str>, start_index: usize) {
    println!("{}: command not found ",commands[start_index..].join(" "));
}