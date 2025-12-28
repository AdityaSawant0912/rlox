use std::env;
use std::io;
use std::fs;
use std::io::Write;
use std::process;

use crate::ast_printer::print_ast;
use crate::scanner::Scanner;
use crate::token::Token;
use crate::parser::Parser;
mod error;
mod error_type;
mod token;
mod token_type;
mod scanner;
mod expr;
mod parser;
mod ast_printer;

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
    let contents = fs::read_to_string(file_path)
        .expect("Failed to read file.");
    match run(&contents) {
        Ok(_n) => println!("Wooho!"),
        Err(_e) => process::exit(65),
    }
    
}


fn run_prompt() {
    loop{
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
            Ok(_n) => println!("Wooho!"),
            Err(_e) => process::exit(65),
        }

    }
}


fn run(source: &str) -> Result<(), error_type::LoxError> {
    let mut scanner = Scanner::new(source.to_string());

    let tokens: Vec<Token> = scanner.scan_tokens();
    // for token in &tokens {
    //     println!("{}", token)
    // }
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(expr) => {
            println!("Parsed the tree successfully.\n");
            println!("{}", print_ast(expr));
        }
        Err(_e) => {
            return Ok(())
        }
    }

    Ok(())
}