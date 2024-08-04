mod token;
mod scanner;
mod char_extensions;
mod string_extensions;
mod error;
mod expression;
mod value;
mod statement;
mod parser;
mod ast_printer;
mod interpreter;

use std::env;
use std::fs;
use std::io::{self, Write};

use ast_printer::AstPrinter;
use error::ErrorHandler;
use expression::Expr;
use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;
use value::Value;

fn read_file(filename: &String) -> String{
    fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    })
}

fn tokenize(filename: &String){
    let file_contents = read_file(filename);

    let mut exit_code = 0;

    let mut scanner = Scanner::new(file_contents);
    scanner.scan_tokens();

    if ErrorHandler::had_error(){
        exit_code = 65;
    }

    for token in scanner.tokens.iter(){
        println!("{}", token)
    }

    std::process::exit(exit_code);
}

fn parse(filename: &String){
    let file_contents = read_file(filename);

    let mut scanner = Scanner::new(file_contents);
    scanner.scan_tokens();

    let mut parser = Parser::new(scanner.tokens);
    let expr = parser.parse();

    if ErrorHandler::had_error(){
        std::process::exit(65)
    }

    match expr {
        Some(e) => println!("{}", AstPrinter::new().print(e)),
        _ => {},
    }
}

fn evaluate(filename: &String) {
    let file_contents = read_file(filename);

    let mut scanner = Scanner::new(file_contents);
    scanner.scan_tokens();

    let mut parser = Parser::new(scanner.tokens);
    let expr = parser.parse();

    if ErrorHandler::had_error(){
        std::process::exit(65)
    }

    match expr {
        Some(expr) => {
            let interpreter = Interpreter::new();
            let value = interpreter.evaluate(&expr);
            
            if ErrorHandler::had_error(){
                std::process::exit(70)
            }

            println!("{}", value.interp_to_string())
        },
        _ => {},
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => tokenize(filename),
        "parse" => parse(filename),
        "evaluate" => evaluate(filename),
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}


