mod scanner;
use crate::scanner::*;

use std::fs::read_to_string;
use std::{env, process};
use std::io::{self, BufRead, Write};

fn run_file(path: &str) -> Result<(), String> {
    match read_to_string(path) {
        Ok(file_content) => return run(&file_content),
        Err(msg) => return Err(msg.to_string()),
    };
}

fn run_prompt() -> Result<(), String> {
    loop {
        print!("> ");

        io::stdout().flush().expect("Could not flush stdout");

        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        match handle.read_line(&mut buffer) {
            Ok(n) => {
                if n <=1 {
                    return Ok(());
                }
            },
            Err(err) => return Err(err.to_string()),
        }

        println!("ECHO: {}", buffer);

        match run(&buffer) {
            Ok(_) => (),
            Err(msg) => println!("{}", msg),
        }
    }
}

fn run(source: &str) -> Result<(), String> {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    
    println!("{:?}", tokens);

    return Ok(());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => process::exit(0),
            Err(msg) => {
                println!("Error: {}", msg);
                process::exit(65);
            }
        }
    } else {
        match run_prompt() {
            Ok(_) => process::exit(0),
            Err(msg) => {
                println!("Error:\n{}", msg);
                process::exit(1);
            }
        }
    }
}
