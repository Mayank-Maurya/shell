#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut input;
    let commands= ["echo", "type", "exit"];
    loop {
        // initiate terminal
        print!("$ ");
        io::stdout().flush().unwrap();

        // take input
        input = String::new();
        stdin.read_line(&mut input).unwrap();

        // collect all args with command at args[0]
        let args: Vec<&str> = input.split_whitespace().collect();
        
        // check if there any command
        if args.len() == 0 {
            continue;
        }

        // check for type of commands
        match args[0].trim() {
            "echo" => echo_command(args),
            "type" => type_command(args, &commands),
            "exit" => {
                if exit_command(args) {
                    break;
                }
            },
            _ => not_found_err(args,0),
        }
    }
}

fn type_command(args: Vec<&str>, commands: &[&str]) {
    if commands.contains(&args[1]) {
        println!("{} is a shell builtin",args[1..].join(" "));
    } else {
        not_found_err(args, 1);
    }
}

fn exit_command(args: Vec<&str>) -> bool {
    if args.len() >=2 && args[1] == "0" {
        return true
    }
    not_found_err(args, 1);
    false
}

fn echo_command(args: Vec<&str>) {
    println!("{} ",args[1..].join(" "));
}

fn not_found_err(args: Vec<&str>, start_index: usize) {
    println!("{}: command not found ",args[start_index..].join(" "));
}