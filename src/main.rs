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

use std::env;
use std::fs;
use std::io::{self, Write};

use ast_printer::AstPrinter;
use expression::AcceptVisitor;
use expression::Expr;
use parser::Parser;
use scanner::Scanner;

fn tokenize(filename: &String){
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });

    let mut exit_code = 0;

    let mut scanner = Scanner::new(file_contents);
    scanner.scan_tokens();

    if scanner.has_error{
        for error in scanner.errors.iter(){
            eprintln!("{}", error)
        }
        exit_code = 65;
    }

    for token in scanner.tokens.iter(){
        println!("{}", token)
    }

    std::process::exit(exit_code);
}

fn parse(filename: &String){
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });

    let mut scanner = Scanner::new(file_contents);
    scanner.scan_tokens();

    //println!("{:?}", scanner.tokens);

    let mut parser = Parser::new(scanner.tokens);
    let expr = parser.parse();

    if parser.has_error{
        return;
    }

    let printer = AstPrinter::new();
    println!("{}", printer.print(expr));
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
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
