use crate::{error::{ErrorHandler, ParserError}, expression::Expr, statement::{self, Stmt}, token::{Token, TokenType}, value::{self, Value}};

pub struct Parser{
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0}
    }

    fn expression(&mut self) -> Result<Box<Expr>, ParserError> {
        self.assignment()
    }

    fn equality(&mut self) -> Result<Box<Expr>, ParserError> {
        let mut expr = self.comparsion();

        let tokens = vec![
            TokenType::BangEqual,
            TokenType::EqualEqual];

        while self.matching(&tokens) {
            let operator = self.previous().clone();
            let right = self.comparsion();

            expr = Expr::Binary { left: expr?, operator, right: right?}.wrap()
        }

        return expr;
    }

    fn comparsion(&mut self) -> Result<Box<Expr>, ParserError> {
        let mut expr = self.term();

        let tokens = vec![
            TokenType::Greater,
            TokenType::GreaterEqual, 
            TokenType::Less,
            TokenType::LessEqual];

        while self.matching(&tokens) {
            let operator = self.previous().clone();
            let right = self.term()?;

            expr = Expr::Binary { left: expr?, operator, right}.wrap()
        }

        return expr;
    }

    fn term(&mut self) -> Result<Box<Expr>, ParserError> {
        let mut expr = self.factor();

        let tokens = vec![
            TokenType::Minus,
            TokenType::Plus];

        while self.matching(&tokens) {
            let operator = self.previous().clone();
            let right = self.factor()?;

            expr = Expr::Binary { left: expr?, operator, right}.wrap()
        }

        return expr;
    }

    fn factor(&mut self) -> Result<Box<Expr>, ParserError> {
        let mut expr = self.unary();

        let tokens = vec![
            TokenType::Slash,
            TokenType::Star];

        while self.matching(&tokens) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            expr = Expr::Binary { left: expr?, operator, right}.wrap()
        }

        return expr;
    }

    fn unary(&mut self) -> Result<Box<Expr>, ParserError> {
        let tokens = vec![
            TokenType::Bang,
            TokenType::Minus];

        if self.matching(&tokens) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            return Expr::Unary { operator, right }.wrap()
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<Box<Expr>, ParserError> {
        if self.matching(&vec![TokenType::False]){
            return Expr::Literal { value: Value::Bool(false) }.wrap()
        }
        if self.matching(&vec![TokenType::True]){
            return Expr::Literal { value: Value::Bool(true) }.wrap()
        }
        if self.matching(&vec![TokenType::Nil]){
            return Expr::Literal { value: Value::Nil }.wrap()
        }
        if self.matching(&vec![TokenType::Number,
                                     TokenType::String]) {
            return Expr::Literal { value:  self.previous().literal.clone() }.wrap()
        }

        if self.matching(&vec![TokenType::Identifier]) {
            return Expr::Variable { name: self.previous().clone() }.wrap()
        }

        if self.matching(&vec![TokenType::LeftParen]) {
            let expr = self.expression();
            _ = self.consume(&TokenType::RightParen, String::from("Expect ')' after expression."));
            return Expr::Grouping { expression: expr? }.wrap()
        }

        Err(self.error(self.peek().clone(), String::from("Expect expression.")))
    }

    pub fn parse_expr(&mut self) -> Option<Box<Expr>> {
        match self.expression(){
            Ok(expr) => Some(expr),
            Err(_) => None,
        }
    }

    fn assignment(&mut self) -> Result<Box<Expr>, ParserError> {
        let expr = self.equality().unwrap();

        if self.matching(&vec![TokenType::Equal]){
            let equals = &self.previous().clone();
            let value = self.assignment();

            match *expr {
                Expr::Variable { name } => {
                    return Expr::Assign { name, value: value? }.wrap()
                },
                _ => {
                    self.error(equals.clone(), String::from("Invalid assignment target."));
                }
            }
        }

        Ok(expr)
    }

    fn var_declaration(&mut self) -> Result<Box<Stmt>, ParserError> {
        let name = &self.consume(&TokenType::Identifier, String::from("Expect variable name.")).unwrap().clone();

        let mut initializer = Expr::Literal { value: Value::Nil }.wrap();

        if self.matching(&vec![TokenType::Equal]){
            initializer = self.expression();
        }

        _ = self.consume(&TokenType::Semicolon, String::from("Expect ';' after variable declaration"));

        Stmt::Var { name: name.clone(), initializer: initializer? }.wrap()
    }

    fn declaration(&mut self) -> Result<Box<Stmt>, ParserError> {
        let result = if self.matching(&vec![TokenType::Var]){
            self.var_declaration()
        } else {
            self.statement()
        };

        match result {
            Ok(o) => Ok(o),
            Err(e) => {
                self.synchronize();
                Err(e)
            }
        }
    }

    pub fn parse_stmt(&mut self) -> Vec<Box<Stmt>> {
        let mut stmts: Vec<Box<Stmt>> = Vec::new();

        while !self.is_end() {
            match self.declaration() {
                Ok(stmt) => stmts.push(stmt),
                Err(_) => {},
            }
        }

        stmts
    }

    fn statement(&mut self) -> Result<Box<Stmt>, ParserError> {
        if self.matching(&vec![TokenType::Print]){
            return self.print_statement();
        }

        self.expression_statement()
    }

    fn expression_statement(&mut self) -> Result<Box<Stmt>, ParserError>{
        let value = self.expression();
        _ = self.consume(&TokenType::Semicolon, String::from("Expect ';' after expression."));
        Stmt::Expression { expression: value? }.wrap()
    }

    fn print_statement(&mut self) -> Result<Box<Stmt>, ParserError> {
        let value = self.expression();
        _ = self.consume(&TokenType::Semicolon, String::from("Expect ';' after value."));
        Stmt::Print { expression: value? }.wrap()
    }

    fn consume(&mut self, token_type: &TokenType, message: String) -> Result<&Token, ParserError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        Err(self.error(self.peek().clone(), message))
    }

    fn error(&mut self, token: Token, message: String) -> ParserError {
        ErrorHandler::error_token(token, message);
        ParserError::new()
    }

    fn matching(&mut self, types: &Vec<TokenType>) -> bool {
        for token_type in types.iter(){
            if self.check(token_type){
                self.advance();
                return true;
            }
        }
        false
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