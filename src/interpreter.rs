use std::{cell::RefCell, io::Error, panic::UnwindSafe, rc::Rc};

use crate::{environment::{environment::{Environment, MutEnv}, BObject, Object, ObjectCaller}, error::ErrorHandler, expression::{Expr, ExprVisitor}, returner::Return, statement::{Stmt, StmtVisitor}, token::{Token, TokenType}};

pub struct Interpreter{
    environment: MutEnv,
    pub globals: MutEnv
}

impl Interpreter {
    pub fn new() -> Self {
        let environment = Rc::new(RefCell::new(Environment::new()));

        Self{
            environment: environment.to_owned(),
            globals: environment.clone()
        }
    }

    pub fn evaluate_expr(&mut self, expr: &Box<Expr>) -> BObject {
        expr.accept(self)
    }

    pub fn evaluate_stmt(&mut self, stmt: &Box<Stmt>) -> (){
        stmt.accept(self)
    }

    fn runtime_error(operator: &Token, message: String) -> BObject{
        ErrorHandler::runtime_error(operator, String::from(message));
        Box::new(Object::Nil)
    }

    pub fn execute_block(&mut self, statements: &Box<[Box<Stmt>]>, environment: MutEnv) {
        let previous = self.environment.to_owned();

        self.environment = environment;

        for stmt in statements.iter() {
            self.evaluate_stmt(stmt)
        }

        self.environment = previous;
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit(&mut self, stmt: &Stmt) -> () {
        match stmt {
            Stmt::Print { expression } => {
                let value = self.evaluate_expr(expression);
                println!("{}", value)
            },
            Stmt::Expression { expression } => {
                self.evaluate_expr(expression);
            },
            Stmt::Block { statements } => {
                let new_enw = Environment::new_enclosing(self.environment.to_owned());

                self.execute_block(statements, 
                    Rc::new(RefCell::new(new_enw)))
            },
            Stmt::Return { keyword, value } => {
                let mut return_value = Box::new(Object::Nil);

                match value {
                    Some(value) => {
                        return_value = self.evaluate_expr(value);
                    },
                    None => {},
                }
                
                Return::add(return_value)
            },
            Stmt::Var { name, initializer } => {
                let value = self.evaluate_expr(initializer);
                self.environment.borrow_mut().define(name, value)
            },
            Stmt::While { condition, body } => {
                while self.evaluate_expr(condition).is_thuthy() {
                    self.evaluate_stmt(body);
                }
            },
            Stmt::If { condition, then_branch, else_branch } => {
                let condition_result = self.evaluate_expr(condition);

                if condition_result.is_thuthy(){
                    self.evaluate_stmt(then_branch)
                } else {
                    match else_branch {
                        Some(branch) => self.evaluate_stmt(branch),
                        None => (),
                    }
                }
            },
            Stmt::Function { name, params, body } => {
                let function = Object::Function{
                    body: body.to_owned(),
                    name: Box::new(name.to_owned()),
                    params: params.to_owned(),
                    environment: self.environment.clone() 
                };
                self.environment.borrow_mut().define(name, Box::new(function))
            },
            _ => panic!("Statement not defined!")
        }
    }
}

impl ExprVisitor<BObject> for Interpreter {
    fn visit(&mut self, expr: &Expr) -> BObject {
        match expr {
            Expr::Assign { name, value } => {
                let value = self.evaluate_expr(value);
                self.environment.borrow_mut().assign(name, value.to_owned());
                return value;
            },
            Expr::Call { callee, paren, arguments } => {
                let mut callee = self.evaluate_expr(callee);

                let mut args = vec![];

                for arg in arguments.iter() {
                    args.push(self.evaluate_expr(arg));
                }

                if !callee.is_callable(){
                    return Interpreter::runtime_error(paren, format!("Can only call functions and classes, but tried {}.", callee));
                }

                if args.len() != callee.arity(){
                    return Interpreter::runtime_error(paren, format!("Expected {} arguments, but got {}.", callee.arity(), args.len()));
                }

                callee.call(self, args.into_boxed_slice())
            },
            Expr::Logical { left, operator, right } => {
                let left = self.evaluate_expr(left);

                if operator.token_type == TokenType::Or{
                    if left.to_owned().is_thuthy() {
                        return left;
                    }
                } else {
                    if !left.to_owned().is_thuthy(){
                        return left;
                    }
                }

                self.evaluate_expr(right)
            },
            Expr::Variable { name } => {
                self.environment.borrow_mut().get(name.to_owned())
            },
            Expr::Literal { value } => value.to_owned(),
            Expr::Grouping { expression } => self.evaluate_expr(expression),
            Expr::Unary { operator, right } => {
                let right = self.evaluate_expr(right);

                match operator.token_type {
                    TokenType::Bang => {
                        Box::new(Object::Boolean(!right.is_thuthy()))
                    },
                    TokenType::Minus => match *right{
                        Object::Number(num) => Box::new(Object::Number(-num)),
                        _ => Interpreter::runtime_error(operator, "Operand must be a number.".to_string()),
                    } ,
                    _ => Box::new(Object::Nil)
                }
            },
            Expr::Binary { left, operator, right } => {
                let left = self.evaluate_expr(left);
                let right = self.evaluate_expr(right);

                match (*left, *right) {
                    (Object::String(str1), Object::String(str2)) => {
                        match operator.token_type{
                            TokenType::Plus => Box::new(Object::String(str1 + &str2)),
                            TokenType::Slash | TokenType::Star | TokenType::Minus => Interpreter::runtime_error(operator, "Operands must be numbers.".to_string()),
                            TokenType::BangEqual => Box::new(Object::Boolean(str1 != str2)),
                            TokenType::EqualEqual => Box::new(Object::Boolean(str1 == str2)),
                            _ => Box::new(Object::Nil)
                        }
                    },
                    (Object::Number(num1), Object::Number(num2)) => {
                        match operator.token_type {
                            TokenType::Plus => Box::new(Object::Number(num1 + num2)),
                            TokenType::Minus => Box::new(Object::Number(num1 - num2)),
                            TokenType::Slash => Box::new(Object::Number(num1 / num2)),
                            TokenType::Star => Box::new(Object::Number(num1 * num2)),
                            TokenType::Greater => Box::new(Object::Boolean(num1 > num2)),
                            TokenType::GreaterEqual => Box::new(Object::Boolean(num1 >= num2)),
                            TokenType::Less => Box::new(Object::Boolean(num1 < num2)),
                            TokenType::LessEqual => Box::new(Object::Boolean(num1 <= num2)),
                            TokenType::BangEqual => Box::new(Object::Boolean(num1 != num2)),
                            TokenType::EqualEqual => Box::new(Object::Boolean(num1 == num2)),
                            _ => Box::new(Object::Number(0.0))
                        }
                    },
                    (val1, val2) => {
                        match operator.token_type {
                            TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual |
                            TokenType::Slash | TokenType::Star | TokenType::Minus => Interpreter::runtime_error(operator, "Operands must be numbers.".to_string()),
                            TokenType::Plus => Interpreter::runtime_error(operator, "Operands must be two numbers or two strings.".to_string()),
                            TokenType::BangEqual => Box::new(Object::Boolean(!val1.is_equal(val2))),
                            TokenType::EqualEqual => Box::new(Object::Boolean(val1.is_equal(val2))),
                            _ => Box::new(Object::Nil)
                        }
                    }
                }
            },
            _ => Box::new(Object::Nil)
        }
    }
}