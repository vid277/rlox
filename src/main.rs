pub mod error;
pub mod expr;
pub mod interpreter;
pub mod parser;
pub mod scanner;

use parser::Parser;

use crate::scanner::*;

use std::fs::read_to_string;
use std::io::{self, BufRead, Write};
use std::{env, process};

fn run_file(path: &str) -> Result<(), String> {
    match read_to_string(path) {
        Ok(file_content) => return run(&file_content),
        Err(msg) => return Err(msg.to_string()),
    };
}

fn run_prompt() -> Result<(), String> {
    loop {
        print!(">>> ");

        io::stdout().flush().expect("Could not flush stdout");

        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        match handle.read_line(&mut buffer) {
            Ok(n) => {
                if n <= 1 {
                    return Ok(());
                }
            }
            Err(err) => return Err(err.to_string()),
        }

        if let Err(msg) = run(&buffer) {
            println!("{}", msg);
        }
    }
}

fn run(source: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    let expr = parser.expression().map_err(|e| e.message)?;

    match interpreter::interpret(&expr) {
        Ok(result) => {
            println!("{}", result);
            Ok(())
        }
        Err(error) => Err(error.report()),
    }
}

fn main() {
    println!("Welcome to the rlox interpreter!");

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
