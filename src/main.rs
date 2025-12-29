use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process;

use crate::executer::execute;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::stmt::Stmt;
use crate::token::Token;
mod ast_printer;
mod error;
mod error_type;
mod executer;
mod expr;
mod interpreter;
mod parser;
mod scanner;
mod stmt;
mod token;
mod token_type;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        panic!("Usage: rlox [script]");
    } else if args.len() == 2 {
        run_file(&args[1])
    } else {
        run_prompt()
    }
}

fn run_file(file_path: &str) {
    let contents = fs::read_to_string(file_path).expect("Failed to read file.");
    match run(&contents) {
        Ok(()) => process::exit(0),
        Err(e) => {
            println!("{}", e);
            process::exit(65)
        }
    }
}

fn run_prompt() {
    loop {
        let mut buffer = String::new();
        print!(">>> ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line.");
        if buffer.trim() == "" {
            break;
        }
        match run(&buffer) {
            Ok(()) => process::exit(0),
            Err(e) => {
                println!("{}", e);
                process::exit(65)
            }
        }
    }
}

fn run(source: &str) -> Result<(), error_type::LoxError> {
    let mut scanner = Scanner::new(source.to_string());

    let tokens: Vec<Token> = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let statements: Vec<Stmt> = parser.parse()?;

    for statement in statements {
        execute(statement)?;
    }

    Ok(())
}
