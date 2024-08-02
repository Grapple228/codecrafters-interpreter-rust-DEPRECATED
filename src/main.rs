mod token;
mod scanner;

use std::env;
use std::fs;
use std::io::{self, Write};

use scanner::Scanner;

fn main() {
    let args: Vec<String> = env::args().collect();

    //println!("{:?}", args);

    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let mut scanner = Scanner::new(file_contents);
            let result = scanner.scan_tokens();

            match result {
                Ok(tokens) => {
                    for token in tokens.iter(){
                        println!("{}", token.to_string())
                    }
                }
                Err(errors) => {
                    for error in errors.iter(){
                        println!("{}", error.to_string())
                    }
                }
            }

            
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
