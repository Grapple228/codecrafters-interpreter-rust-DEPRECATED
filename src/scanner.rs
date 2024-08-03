use crate::{char_extensions::CharExtensions, error::ScannerError, string_extensions::StringExtensions, token::{Data, Token, TokenType}};

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut m: HashMap<&'static str, TokenType> = HashMap::new();
        m.insert("and",    TokenType::And);
        m.insert("class",  TokenType::Class);
        m.insert("else",   TokenType::Else);
        m.insert("false",  TokenType::False);
        m.insert("for",    TokenType::For);
        m.insert("fun",    TokenType::Fun);
        m.insert("if",     TokenType::If);
        m.insert("nil",    TokenType::Nil);
        m.insert("or",     TokenType::Or);
        m.insert("print",  TokenType::Print);
        m.insert("return", TokenType::Return);
        m.insert("super",  TokenType::Super);
        m.insert("this",   TokenType::This);
        m.insert("true",   TokenType::True);
        m.insert("var",    TokenType::Var);
        m.insert("while",  TokenType::While);
        m
    };
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

    pub fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_value(token_type, Data::Null)
    }

    fn add_token_with_value(&mut self, token_type: TokenType, literal: Data) {
        let text: String = self.source.substring(self.start, self.current);

        self.tokens.push(Token { token_type, lexeme: text, literal, line: self.line})
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_end() {
            self.start = self.current;
            self.scan_token();
        }
        
        self.tokens.push(Token{
            token_type: TokenType::Eof,
            lexeme: String::new(),
            line: self.line,
            literal: Data::Null
        });
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let check_next = self.check_next('=');
                self.add_token(if check_next {TokenType::BangEqual} else {TokenType::Bang})
            }
            '=' => {
                let check_next = self.check_next('=');
                self.add_token(if check_next {TokenType::EqualEqual} else {TokenType::Equal})
            }
            '<' => {
                let check_next = self.check_next('=');
                self.add_token(if check_next {TokenType::LessEqual} else {TokenType::Less})
            }
            '>' => {
                let check_next = self.check_next('=');
                self.add_token(if check_next {TokenType::GreaterEqual} else {TokenType::Greater})
            }
            '/' =>{
                if self.check_next('/'){
                    while self.peek() != '\n' && !self.is_end() {
                        _ = self.advance();
                    }
                } else{
                    self.add_token(TokenType::Slash);
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
                else if c.is_alpha(){
                    self.identifier();
                }
                else{
                    self.error(ScannerError::UnexpectedCharacter(self.line, c));
                }
            }
        }
    }

    fn is_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char{
        let c = self.source.char_at(self.current);
        self.current+=1;
        c
    }

    fn peek(&self) -> char{
        if self.is_end() {return '\0';}
        self.get_current()
    }

    fn peek_next(&self) -> char{
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.char_at(self.current + 1)
    }

    fn get_current(&self) -> char{
        self.source.char_at(self.current)
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

    fn error(&mut self, error: ScannerError){
        self.has_error = true;
        self.errors.push(error);
    }

    fn identifier(&mut self){
        while self.peek().is_alpha_numeric() {
            self.advance();

        }

        let a = String::new();
        a.substring(1, 3);

        self.add_token(TokenType::Identifier)
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

        let value = self.source.substring(self.start, self.current);
        let literal = Data::Number(value.parse().unwrap_or_default());

        self.add_token_with_value(TokenType::Number, literal);
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
            return;
        }

        self.advance();

        let value = self.source.substring(self.start + 1, self.current - 1);
        self.add_token_with_value(TokenType::String, Data::String(value));
    }
}