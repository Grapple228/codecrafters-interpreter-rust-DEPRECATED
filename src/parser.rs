use std::fmt::format;

use crate::{environment::Object, error::{ErrorHandler, ParserError}, expression::Expr, statement::Stmt, token::{Token, TokenType}};

pub type MyResult<T> = std::result::Result<Box<T>, ParserError>;

pub struct Parser{
    tokens: Vec<Token>,
    current: usize,
    is_expression: bool
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0, is_expression: true}
    }

    pub fn is_expression(&self) -> bool {
        self.is_expression
    }

    fn expression(&mut self) -> MyResult<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> MyResult<Expr> {
        let expr = self.or();

        if self.matching(vec![TokenType::Equal]){
            let equals = self.previous().to_owned();
            let value = self.assignment()?;

            match expr.as_ref() {
                Ok(ok) => {
                    match ok.as_ref() {
                        Expr::Variable { name } => {
                            return Expr::Assign { name: name.to_owned(), value }.wrap()
                        },
                        _ => {
                            self.error(equals, String::from("Invalid assignment target."));
                        }
                    }
                },
                Err(e) => return Err(e.clone())
            }
        }

        expr
    }

    fn or(&mut self) -> MyResult<Expr> {
        let mut expr = self.and();

        while self.matching(vec![TokenType::Or]) {
            let operator = self.previous().to_owned();
            let right = self.and()?;
            expr = Expr::Logical { left: expr?, operator, right }.wrap();
        }

        expr
    }

    fn and(&mut self) -> MyResult<Expr> {
        let mut expr = self.equality();
        
        while self.matching(vec![TokenType::And]) {
            let operator = self.previous().to_owned();
            let right = self.equality()?;
            expr = Expr::Logical { left: expr?, operator, right }.wrap();
        }

        expr
    }

    fn equality(&mut self) -> MyResult<Expr> {
        let mut expr = self.comparsion();

        while self.matching(vec![TokenType::BangEqual,TokenType::EqualEqual]) {
            let operator = self.previous().to_owned();
            let right = self.comparsion()?;

            expr = Expr::Binary { left: expr?, operator, right}.wrap()
        }

        return expr;
    }

    fn comparsion(&mut self) -> MyResult<Expr> {
        let mut expr = self.term();

        while self.matching( vec![TokenType::Greater,TokenType::GreaterEqual, TokenType::Less,TokenType::LessEqual]) {
            let operator = self.previous().to_owned();
            let right = self.term()?;

            expr = Expr::Binary { left: expr?, operator, right}.wrap()
        }

        return expr;
    }

    fn term(&mut self) -> MyResult<Expr> {
        let mut expr = self.factor();

        while self.matching(vec![TokenType::Minus,TokenType::Plus]) {
            let operator = self.previous().to_owned();
            let right = self.factor()?;

            expr = Expr::Binary { left: expr?, operator, right}.wrap()
        }

        return expr;
    }

    fn factor(&mut self) -> MyResult<Expr> {
        let mut expr = self.unary();

        while self.matching(vec![TokenType::Slash,TokenType::Star]) {
            let operator = self.previous().to_owned();
            let right = self.unary()?;

            expr = Expr::Binary { left: expr?, operator, right}.wrap()
        }

        return expr;
    }

    fn unary(&mut self) -> MyResult<Expr> {
        if self.matching(vec![TokenType::Bang,TokenType::Minus]) {
            let operator = self.previous().to_owned();
            let right = self.unary()?;

            return Expr::Unary { operator, right }.wrap()
        }

        return self.call();
    }

    fn call(&mut self) -> MyResult<Expr> {
        let mut expr = self.primary();

        loop {
            if self.matching(vec![TokenType::LeftParen]){
                expr = self.finish_call(expr?);
            } else {
                break expr;
            }
        }
    }

    fn finish_call(&mut self, callee: Box<Expr>) -> MyResult<Expr> {
        let mut arguments = vec![];

        if !self.check(&TokenType::RightParen){
            arguments.push(self.expression()?);
            while self.matching(vec![TokenType::Comma]) {
                if arguments.len() > 255 {
                    self.error(self.peek().to_owned(), String::from("Can't have more than 255 arguments."));
                }
                arguments.push(self.expression()?);
            }
        }

        let paren = self.consume(&TokenType::RightParen, String::from("Expect ')' after argumetns."));

        Expr::Call { callee, paren: paren?.to_owned(), arguments }.wrap()
    }

    fn primary(&mut self) -> MyResult<Expr> {
        if self.matching(vec![TokenType::False]){
            return Expr::Literal { value: Object::Boolean(false) }.wrap()
        }
        if self.matching(vec![TokenType::True]){
            return Expr::Literal { value: Object::Boolean(true) }.wrap()
        }
        if self.matching(vec![TokenType::Nil]){
            return Expr::Literal { value: Object::Nil }.wrap()
        }
        if self.matching(vec![TokenType::Number,
                                     TokenType::String]) {
            return Expr::Literal { value:  self.previous().literal.to_owned() }.wrap()
        }

        if self.matching(vec![TokenType::Identifier]) {
            return Expr::Variable { name: self.previous().to_owned() }.wrap()
        }

        if self.matching(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            _ = self.consume(&TokenType::RightParen, String::from("Expect ')' after expression."));
            return Expr::Grouping { expression: expr? }.wrap()
        }

        Err(self.error(self.peek().clone(), String::from("Expect expression.")))
    }

    fn declaration(&mut self) -> MyResult<Stmt> {
        let result = if self.matching(vec![TokenType::Fun]) {
            self.function("function")
        }
        else if self.matching(vec![TokenType::Var]){
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

    fn add_parameter(&mut self, params: &mut Vec<Token>){
        if params.len() >= 255{
            self.error(self.peek().to_owned(), String::from("Can't have more than 255 parameters."));
        }
        let token = self.consume(&TokenType::Identifier, String::from("Expect parameter name.")); 
        params.push(token.unwrap().to_owned());
    }

    fn function(&mut self, kind: &'static str) -> MyResult<Stmt> {
        self.is_expression = false;
        let name = &self.consume(&TokenType::Identifier, format!("Expect {} name.", kind))?.to_owned();

        _ = self.consume(&TokenType::LeftParen, format!("Expect '(' after {} name.", kind));
        
        let mut params = vec![];
        
        if !self.check(&TokenType::RightParen){
            self.add_parameter(&mut params);
            
            while self.matching(vec![TokenType::Comma]) {
                self.add_parameter(&mut params);
            }
        }
        
        _ = self.consume(&TokenType::RightParen, format!("Expect ')' after parameters."));

        _ = self.consume(&TokenType::LeftBrace, format!("Expect '{{' before {} body.", kind));

        let body = self.block();

        Stmt::Function { name: name.to_owned(), params, body }.wrap()
    }

    fn var_declaration(&mut self) -> MyResult<Stmt> {
        self.is_expression = false;

        let name = self.consume(&TokenType::Identifier, String::from("Expect variable name.")).unwrap().to_owned();

        let mut initializer = Expr::Literal { value: Object::Unitialized }.wrap();

        if self.matching(vec![TokenType::Equal]){
            initializer = self.expression();
        }

        _ = self.consume(&TokenType::Semicolon, String::from("Expect ';' after variable declaration"));

        Stmt::Var { name: name.to_owned(), initializer: initializer? }.wrap()
    }

    pub fn parse_expr(&mut self) -> Option<Box<Expr>> {
        match self.expression(){
            Ok(expr) => Some(expr),
            Err(_) => None,
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

    fn block(&mut self) -> Vec<Box<Stmt>>{
        let mut stmts = Vec::new();

        while !self.check(&TokenType::RightBrace) 
            & !self.is_end() {
            stmts.push(self.declaration().unwrap());
        }

        _ = self.consume(&TokenType::RightBrace, String::from("Expect '}' after block."));

        stmts
    }

    fn statement(&mut self) -> MyResult<Stmt> {
        if self.matching(vec![TokenType::For]){
            self.is_expression = false;
            return self.for_statement();
        }
        
        if self.matching(vec![TokenType::If]){
            self.is_expression = false;
            return self.if_statement();
        }
        
        if self.matching(vec![TokenType::Print]){
            self.is_expression = false;
            return self.print_statement();
        }

        if self.matching(vec![TokenType::While]){
            self.is_expression = false;
            return self.while_statement();
        }

        if self.matching(vec![TokenType::LeftBrace]){
            self.is_expression = false;
            return Stmt::Block { statements: self.block() }.wrap();
        }

        self.expression_statement()
    }

    fn if_statement(&mut self) -> MyResult<Stmt> {
        _ = self.consume(&TokenType::LeftParen, String::from("Expect '(' after 'if'."));
        let condition = self.expression();
        _ = self.consume(&TokenType::RightParen, String::from("Expect ')' after condition."));

        let then_branch = self.statement();
        let else_branch = if self.matching(vec![TokenType::Else]){
            Some(self.statement()?)
        } else {
            None
        };

        Stmt::If { condition:condition?, then_branch: then_branch?, else_branch }.wrap()
    }

    fn for_statement(&mut self) -> MyResult<Stmt> {
        _ = self.consume(&TokenType::LeftParen, String::from("Expect '(' after 'for'."));
        
        let initializer = if self.matching(vec![TokenType::Semicolon]){
            None
        } else if self.matching(vec![TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };
        
        let mut condition: Option<Box<Expr>> = None;
        if !self.check(&TokenType::Semicolon){
            condition = Some(self.expression()?);
        };
        
        _ = self.consume(&TokenType::Semicolon, String::from("Expect ';' after loop condition."));
        
        let mut increment: Option<Box<Expr>> = None;
        if !self.check(&TokenType::RightParen){
            increment = Some(self.expression()?);
        };
        
        _ = self.consume(&TokenType::RightParen, String::from("Expect ')' after for clauses."));

        let mut body = self.statement()?;

        match increment {
            Some(increment) => {
                let statements = vec![body, Box::new(Stmt::Expression { expression: increment })];
                body = Box::new(Stmt::Block { statements });
            },
            None => {},
        }

        if condition.is_none(){
            condition = Some(Box::new(Expr::Literal { value: Object::Boolean(true) }));
        }

        body = Box::new(Stmt::While { condition: condition.unwrap(), body });

        match initializer{
            Some(initializer) => {
                let statements = vec![initializer, body];
                body = Box::new(Stmt::Block { statements });
            },
            None => {},
        }

        return body.wrap();

    }

    fn while_statement(&mut self) -> MyResult<Stmt> {
        _ = self.consume(&TokenType::LeftParen, String::from("Expect '(' after 'while'."));
        let condition = self.expression();
        _ = self.consume(&TokenType::RightParen, String::from("Expect ')' after condition."));
        let body = self.statement();
        
        Stmt::While { condition: condition?, body: body? }.wrap()
    }

    fn expression_statement(&mut self) -> MyResult<Stmt> {
        let value = self.expression();
        if !self.is_expression{
            _ = self.consume(&TokenType::Semicolon, String::from("Expect ';' after expression."));
        }
        Stmt::Expression { expression: value? }.wrap()
    }

    fn print_statement(&mut self) -> MyResult<Stmt> {
        self.is_expression = false;
        let value = self.expression();
        _ = self.consume(&TokenType::Semicolon, String::from("Expect ';' after value."));
        Stmt::Print { expression: value? }.wrap()
    }

    fn consume(&mut self, token_type: &TokenType, message: String) -> Result<&Token, ParserError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        Err(self.error(self.peek().to_owned(), message))
    }

    fn error(&mut self, token: Token, message: String) -> ParserError {
        ErrorHandler::error_token(token, message);
        ParserError::new()
    }

    fn matching(&mut self, types: Vec<TokenType>) -> bool {
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