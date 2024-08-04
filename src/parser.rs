use crate::{expression::Expr, token::{Token, TokenType}, value::Value};

pub struct Parser{
    tokens: Vec<Token>,
    current: usize,
    pub has_error: bool
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0, has_error: false }
    }

    fn expression(&mut self) -> Box<Expr>{
        self.equality()
    }

    fn equality(&mut self) -> Box<Expr> {
        let mut expr = self.comparsion();

        let tokens = vec![
            TokenType::BangEqual,
            TokenType::EqualEqual];

        while self.matching(&tokens) {
            let operator = self.previous().clone();
            let right = self.comparsion();

            expr = Box::new(Expr::Binary { left: expr, operator, right})
        }

        return expr;
    }

    fn comparsion(&mut self) -> Box<Expr> {
        let mut expr = self.term();

        let tokens = vec![
            TokenType::Greater,
            TokenType::GreaterEqual, 
            TokenType::Less,
            TokenType::LessEqual];

        while self.matching(&tokens) {
            let operator = self.previous().clone();
            let right = self.term();

            expr = Box::new(Expr::Binary { left: expr, operator, right})
        }

        return expr;
    }

    fn term(&mut self) -> Box<Expr> {
        let mut expr = self.factor();

        let tokens = vec![
            TokenType::Minus,
            TokenType::Plus];

        while self.matching(&tokens) {
            let operator = self.previous().clone();
            let right = self.factor();

            expr = Box::new(Expr::Binary { left: expr, operator, right})
        }

        return expr;
    }

    fn factor(&mut self) -> Box<Expr> {
        let mut expr = self.unary();

        let tokens = vec![
            TokenType::Slash,
            TokenType::Star];

        while self.matching(&tokens) {
            let operator = self.previous().clone();
            let right = self.unary();

            expr = Box::new(Expr::Binary { left: expr, operator, right})
        }

        return expr;
    }

    fn unary(&mut self) -> Box<Expr> {
        let tokens = vec![
            TokenType::Bang,
            TokenType::Minus];

        if self.matching(&tokens) {
            let operator = self.previous().clone();
            let right = self.unary();

            return Box::new(Expr::Unary { operator, right })
        }

        return self.primary();
    }

    fn primary(&mut self) -> Box<Expr> {
        if self.matching(&vec![TokenType::False]){
            return Box::new(Expr::Literal { value: Value::Bool(false) });
        }
        if self.matching(&vec![TokenType::True]){
            return Box::new(Expr::Literal { value: Value::Bool(true) });
        }
        if self.matching(&vec![TokenType::Nil]){
            return Box::new(Expr::Literal { value: Value::Nil });
        }

        if self.matching(&vec![TokenType::Number,
                                     TokenType::String]) {
            return Box::new(Expr::Literal { value:  self.previous().literal.clone() })
        }

        if self.matching(&vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(&TokenType::RightParen, String::from("Expect ')' after expression."));
            return Box::new(Expr::Grouping { expression: expr })
        }

        let p = self.peek();

        panic!("{} {}", p, "Expect expression.");
    }

    pub fn parse(&mut self) -> Box<Expr> {
        self.expression()
    }

    fn consume(&mut self, token_type: &TokenType, message: String) -> &Token {
        if self.check(token_type) {
            return self.advance();
        }

        panic!("{} {}", self.peek(), message);
    }

    fn matching(&mut self, types: &Vec<TokenType>) -> bool {
        for token_type in types.iter(){
            if self.check(token_type){
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_end() {
            if self.previous().token_type == TokenType::Semicolon{
                return;
            }

            match self.peek().token_type {
                TokenType::Class | TokenType::Fun |
                TokenType::For   | TokenType::If |
                TokenType::Print | TokenType::Return |
                TokenType::Var   | TokenType::While => return,
                _ => self.advance()
            };
        }
    }

    fn check(&self, token_type: &TokenType) -> bool{
        if self.is_end() {
            return false;
        }
        return self.peek().token_type == *token_type
    }

    fn advance(&mut self) -> &Token{
        if !self.is_end() {
            self.current+=1;
        }
        return self.previous();
    }

    fn is_end(&self) -> bool{
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }
}