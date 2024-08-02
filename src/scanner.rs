use std::fmt::{Display, Error as FmtError, Formatter};

use crate::token::{Token, TokenType};

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut m: HashMap<&'static str, TokenType> = HashMap::new();
        m.insert("and",    TokenType::AND);
        m.insert("class",  TokenType::CLASS);
        m.insert("else",   TokenType::ELSE);
        m.insert("false",  TokenType::FALSE);
        m.insert("for",    TokenType::FOR);
        m.insert("fun",    TokenType::FUN);
        m.insert("if",     TokenType::IF);
        m.insert("nil",    TokenType::NIL);
        m.insert("or",     TokenType::OR);
        m.insert("print",  TokenType::PRINT);
        m.insert("return", TokenType::RETURN);
        m.insert("super",  TokenType::SUPER);
        m.insert("this",   TokenType::THIS);
        m.insert("true",   TokenType::TRUE);
        m.insert("var",    TokenType::VAR);
        m.insert("while",  TokenType::WHILE);
        m
    };
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ScannerError{
    UnexpectedCharacter(usize, char),
    UnterminatedString(usize),
}

impl Display for ScannerError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::UnexpectedCharacter(line, character) => write!(f, "[line {}] Error: Unexpected character: {}", line, character),
            Self::UnterminatedString(line) => write!(f, "[line {}] Error: Unterminated string", line),            
        }
    }
}

pub struct Scanner{
    source: String,
    start: usize,
    current: usize,
    line: usize,
    pub tokens: Vec<Token>,
    pub errors: Vec<ScannerError>,
    pub has_error: bool
}

impl Scanner{
    pub fn new(source: String) -> Scanner {
        Scanner{
            current: 0,
            source,
            line: 1,
            start: 0,
            errors: Vec::new(),
            tokens: Vec::new(),
            has_error: false,
        }
    }

    fn error(&mut self, error: ScannerError){
        self.has_error = true;
        self.errors.push(error);
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

    pub fn scan_tokens(&mut self) {
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
                self.add_token(if check_next {TokenType::LESS_EQUAL} else {TokenType::LESS})
            }
            '>' => {
                let check_next = self.check_next('=');
                self.add_token(if check_next {TokenType::GREATER_EQUAL} else {TokenType::GREATER})
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
                else if self.is_alpha(c){
                    self.identifier();
                }
                else{
                    self.error(ScannerError::UnexpectedCharacter(self.line, c));
                }
            }
        }
    }

    fn is_alpha(&self, c: char) -> bool {
        c >= 'a' && c <= 'z' ||
        c >= 'A' && c <= 'Z' ||
        c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || c.is_digit(10)
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

    fn identifier(&mut self){
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        self.add_token(TokenType::IDENTIFIER)
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
            self.error(ScannerError::UnterminatedString(self.line));
        }

        self.advance();

        let value = self.substring(self.start + 1, self.current - 1);
        self.add_token_with_value(TokenType::STRING, value);
    }
}