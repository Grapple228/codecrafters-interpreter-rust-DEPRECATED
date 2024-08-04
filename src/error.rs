use std::fmt::{Display, Formatter, Error};

use crate::token::{Token, TokenType};

static mut HAS_ERROR: bool = false;

pub struct ErrorHandler{}

impl ErrorHandler{
    pub fn error(line: usize, message: String){
        Self::report(line, String::new(), message)
    }

    pub fn error_token(token: Token, message: String) {
        if token.token_type == TokenType::Eof{
            Self::report(token.line, String::from(" at end"), message)
        } else{
            let wher = format!(" at '{}'", token.lexeme);
            Self::report(token.line, wher, message)
        }
    }

    fn report(line: usize, wher: String, message: String){
        eprintln!("[line {0}] Error{1}: {2}", line, wher, message);
        unsafe { HAS_ERROR = true };
    }

    pub fn had_error() -> bool {
        unsafe { HAS_ERROR }
    }

    pub fn reset() {
        unsafe { HAS_ERROR = false }
    }
}

#[derive(Debug)]
pub struct ParserError{}

impl ParserError {
    pub fn new() -> Self {
        Self {  }
    }
}
