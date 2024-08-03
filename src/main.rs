mod token;
mod scanner;
mod char_extensions;
mod string_extensions;
mod error;

use std::env;
use std::fs;
use std::io::{self, Write};

use scanner::Scanner;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

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
                println!("{}", token.to_string())
            }

            std::process::exit(exit_code);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
