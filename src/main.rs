#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut input;
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        input = String::new();
        stdin.read_line(&mut input).unwrap();
        let args: Vec<&str> = input.split_whitespace().collect();
        match args[0].trim() {
            "echo" => println!("{} ",args[1..].join(" ")),
            "exit" => {
                if args.len() >=2 && args[1] == "0" {
                    break;
                }
                println!("{}: command not found", input.trim())
            },
            _ => println!("{}: command not found", input.trim()),
        }
    }
}
