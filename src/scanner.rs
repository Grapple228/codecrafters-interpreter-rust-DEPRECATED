use crate::token::{Token, TokenType};

pub struct Scanner{
    source: String,
    pub tokens: Vec<Token>
}


pub struct ScannerBuilder{
    scanner: Scanner,
    start: usize,
    current: usize,
    line: usize
}

impl ScannerBuilder{
    pub fn new(source: String) -> ScannerBuilder {
        ScannerBuilder{
            current: 0,
            scanner: Scanner::new(source),
            line: 1,
            start: 0
        }
    }

    pub fn finalize(self) -> Scanner{
        self.scanner
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
        //println!("start {} end {}", self.start, self.current);
        let text: String = self.substring(self.start, self.current);
        //println!("{}", text);
        self.scanner.tokens.push(Token { token_type, lexeme: text, literal, line: self.line })
    }

    fn is_end(&self) -> bool {
        self.scanner.is_end(self.current)
    }

    fn get_current(&self) -> char{
        self.char_at(self.current)
    }

    fn char_at(&self, index: usize) -> char{
        self.scanner.source.chars().nth(index).unwrap()
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

    pub fn scan_tokens(&mut self){
        while !self.is_end() {
            self.start = self.current;
            self.scan_token();
        }
        
        self.scanner.tokens.push(Token{
            token_type: TokenType::EOF,
            lexeme: String::new(),
            line: self.line,
            literal: String::new()
        })
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
                    panic!("Unexpected token!")
                }
            }
        }
    }

    fn peek(&self) -> char{
        if self.is_end() {return '\0';}
        self.get_current()
    }

    fn peek_next(&self) -> char{
        if self.current + 1 >= self.scanner.source.len() {
            return '\0';
        }
        self.char_at(self.current + 1)
    }

    fn substring(&self, start: usize, end: usize) -> String{
        self.scanner.source.chars().skip(start).take(end-start).collect()
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

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner{
            source, tokens: Vec::new()
        }
    }

    fn is_end(&self, current: usize) -> bool {
        current >= self.source.len()
    }
}