use std::fmt::{Display, Formatter, Error as FmtError};

use crate::token::{Token, TokenType};


#[derive(Debug, Clone, thiserror::Error)]
pub enum ScannerError{
    UnexpectedToken(usize, String)
}

impl Display for ScannerError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::UnexpectedToken(line, token) => write!(f, "Token {} on line {} is unexpected!", token, line)
        }
    }
}

pub struct Scanner{
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    has_error: bool,
    errors: Vec<ScannerError>
}

impl Scanner{
    pub fn new(source: String) -> Scanner {
        Scanner{
            current: 0,
            source,
            tokens: Vec::new(),
            line: 1,
            start: 0,
            has_error: false,
            errors: Vec::new()
        }
    }

    fn is_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char{
        let c = self.char_at(self.current);
        self.current+=1;
        c
    }

    pub fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_value(token_type, String::new())
    }

    fn add_token_with_value(&mut self, token_type: TokenType, literal: String) {
        let text: String = self.substring(self.start, self.current);

        self.tokens.push(Token { token_type, lexeme: text, literal, line: self.line })
    }

    fn get_current(&self) -> char{
        self.char_at(self.current)
    }

    fn char_at(&self, index: usize) -> char{
        self.source.chars().nth(index).unwrap()
    }

    fn check_next(&mut self, expected: char) -> bool{
        if self.is_end() {
            return false;
        }

        if self.get_current() != expected{
            return false;
        }

        self.current+=1;
        true
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, &Vec<ScannerError>>{
        while !self.is_end() {
            self.start = self.current;
            self.scan_token();
        }
        
        self.tokens.push(Token{
            token_type: TokenType::EOF,
            lexeme: String::new(),
            line: self.line,
            literal: String::new()
        });

        if self.has_error{
            Result::Err(&self.errors)
        } else{
            Result::Ok(&self.tokens)
        }       
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                let check_next = self.check_next('=');
                self.add_token(if check_next {TokenType::BANG_EQUAL} else {TokenType::BANG})
            }
            '=' => {
                let check_next = self.check_next('=');
                self.add_token(if check_next {TokenType::EQUAL_EQUAL} else {TokenType::EQUAL})
            }
            '<' => {
                let check_next = self.check_next('=');
                self.add_token(if check_next {TokenType::LESS_EQUAL} else {TokenType::EQUAL})
            }
            '>' => {
                let check_next = self.check_next('=');
                self.add_token(if check_next {TokenType::GREATER_EQUAL} else {TokenType::EQUAL})
            }
            '/' =>{
                if self.check_next('/'){
                    while self.peek() != '\n' && !self.is_end() {
                        _ = self.advance();
                    }
                } else{
                    self.add_token(TokenType::SLASH);
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => {self.line+=1}
            '"' => {self.string()}
            any => {
                if any.is_digit(10){
                    self.number();
                }
                else{
                    self.error(ScannerError::UnexpectedToken(self.line, c.to_string()))
                }
            }
        }
    }

    fn error(&mut self, error: ScannerError){
        self.has_error = true;
        self.errors.push(error);
    }

    fn peek(&self) -> char{
        if self.is_end() {return '\0';}
        self.get_current()
    }

    fn peek_next(&self) -> char{
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.char_at(self.current + 1)
    }

    fn substring(&self, start: usize, end: usize) -> String{
        self.source.chars().skip(start).take(end-start).collect()
    }

    fn number(&mut self){
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        self.add_token_with_value(TokenType::NUMBER, self.substring(self.start, self.current))
    }

    fn string(&mut self){
        while self.peek() != '"' && !self.is_end() {
            if self.peek() == '\n'{
                self.line+=1;
            }
            self.advance();
        }
        
        if self.is_end(){
            panic!("Unterminated string!")
        }

        self.advance();

        let value = self.substring(self.start + 1, self.current - 1);
        self.add_token_with_value(TokenType::STRING, value);
    }
}