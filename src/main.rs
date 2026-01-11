use std::cell::RefCell;
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process;
use std::rc::Rc;

use crate::error_type::LoxError;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::resolver::Resolver;
use crate::scanner::Scanner;
use crate::stmt::Stmt;
use crate::token::Token;
// mod ast_printer;
mod environment;
mod error;
mod error_type;
mod expr;
mod interpreter;
mod lox_callable;
mod lox_function;
mod native_functions;
mod parser;
mod resolver;
mod scanner;
mod stmt;
mod token;
mod token_type;
mod lox_class;
mod lox_instance;

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

fn run(source: &str) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source.to_string());

    let tokens: Vec<Token> = scanner.scan_tokens();

    // for token in &tokens {
    //     println!("{}", token)
    // }

    let mut parser = Parser::new(tokens);
    let statements: Vec<Stmt> = parser.parse()?;

    let interpreter = Rc::new(RefCell::new(Interpreter::new()));

    let mut resolver = Resolver::new(Rc::clone(&interpreter));
    resolver.resolve_stmts(statements.clone().into_iter().map(Box::new).collect());
    if resolver.had_error {
        println!("{}", LoxError::ParseError);
        process::exit(65)
    }
    for statement in statements {
        interpreter.borrow_mut().execute(statement)?;
    }

    Ok(())
}
