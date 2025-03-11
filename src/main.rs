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
        println!("{}: command not found", input.trim());
    }
}
