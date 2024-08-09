use std::{cell::RefCell, rc::Rc};

use crate::{environment::{Environment, MutEnv}, error::ErrorHandler, expression::{Expr, ExprVisitor}, object::Object, statement::{Stmt, StmtVisitor}, token::{Token, TokenType}};

pub struct Interpreter{
    environment: MutEnv
}

impl Interpreter {
    pub fn new() -> Self {
        Self { environment: Rc::new(RefCell::new(Environment::new())) }
    }
    
    pub fn evaluate_expr(&mut self, expr: &Box<Expr>) -> Object {
        expr.accept(self)
    }

    pub fn evaluate_stmt(&mut self, stmt: &Box<Stmt>) -> (){
        stmt.accept(self)
    }

    fn runtime_error(operator: &Token, message: &'static str) -> Object{
        ErrorHandler::runtime_error(operator, String::from(message));
        Object::Nil
    }

    fn execute_block(&mut self, statements: &Vec<Box<Stmt>>, environment: MutEnv){
        let previous = self.environment.clone();

        self.environment = environment;

        for stmt in statements{
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
            }
            _ => panic!("Statement not defined!")
        }
    }
}

impl ExprVisitor<Object> for Interpreter {
    fn visit(&mut self, expr: &Expr) -> Object {
        match expr {
            Expr::Assign { name, value } => {
                let value = self.evaluate_expr(value);
                self.environment.borrow_mut().assign(name, value.clone());
                return value;
            },
            Expr::Logical { left, operator, right } => {
                let left = self.evaluate_expr(left);

                if operator.token_type == TokenType::Or{
                    if left.clone().is_thuthy() {
                        return left;
                    }
                } else {
                    if !left.clone().is_thuthy(){
                        return left;
                    }
                }

                self.evaluate_expr(right)
            },
            Expr::Variable { name } => {
                self.environment.borrow_mut().get(name.clone())
            },
            Expr::Literal { value } => value.clone(),
            Expr::Grouping { expression } => self.evaluate_expr(expression),
            Expr::Unary { operator, right } => {
                let right = self.evaluate_expr(right);

                match operator.token_type {
                    TokenType::Bang => {
                        Object::Boolean(!right.is_thuthy())
                    },
                    TokenType::Minus => match right{
                        Object::Number(num) => Object::Number(-num),
                        _ => Interpreter::runtime_error(operator, "Operand must be a number."),
                    } ,
                    _ => Object::Nil
                }
            },
            Expr::Binary { left, operator, right } => {
                let left = self.evaluate_expr(left);
                let right = self.evaluate_expr(right);

                match (left, right) {
                    (Object::String(str1), Object::String(str2)) => {
                        match operator.token_type{
                            TokenType::Plus => Object::String(str1 + &str2),
                            TokenType::Slash | TokenType::Star | TokenType::Minus => Interpreter::runtime_error(operator, "Operands must be numbers."),
                            TokenType::BangEqual => Object::Boolean(str1 != str2),
                            TokenType::EqualEqual => Object::Boolean(str1 == str2),
                            _ => Object::Nil
                        }
                    },
                    (Object::Number(num1), Object::Number(num2)) => {
                        match operator.token_type {
                            TokenType::Plus => Object::Number(num1 + num2),
                            TokenType::Minus => Object::Number(num1 - num2),
                            TokenType::Slash => Object::Number(num1 / num2),
                            TokenType::Star => Object::Number(num1 * num2),
                            TokenType::Greater => Object::Boolean(num1 > num2),
                            TokenType::GreaterEqual => Object::Boolean(num1 >= num2),
                            TokenType::Less => Object::Boolean(num1 < num2),
                            TokenType::LessEqual => Object::Boolean(num1 <= num2),
                            TokenType::BangEqual => Object::Boolean(num1 != num2),
                            TokenType::EqualEqual => Object::Boolean(num1 == num2),
                            _ => Object::Number(0.0)
                        }
                    },
                    (val1, val2) => {
                        match operator.token_type {
                            TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual |
                            TokenType::Slash | TokenType::Star | TokenType::Minus => Interpreter::runtime_error(operator, "Operands must be numbers."),
                            TokenType::Plus => Interpreter::runtime_error(operator, "Operands must be two numbers or two strings."),
                            TokenType::BangEqual => Object::Boolean(!val1.is_equal(val2)),
                            TokenType::EqualEqual => Object::Boolean(val1.is_equal(val2)),
                            _ => Object::Nil
                        }
                    }
                }
            },
            _ => Object::Nil
        }
    }
}