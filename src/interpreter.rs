use crate::{environment::Environment, error::ErrorHandler, expression::{Expr, ExprVisitor}, statement::{Stmt, StmtVisitor}, token::{Token, TokenType}, value::Value};

pub struct Interpreter{
    environment: Box<Environment>
}

impl Interpreter {
    pub fn new() -> Self {
        Self { environment: Box::new(Environment::new()) }
    }
    
    pub fn evaluate_expr(&mut self, expr: &Box<Expr>) -> Value {
        expr.accept(self)
    }

    pub fn evaluate_stmt(&mut self, stmt: &Box<Stmt>) -> (){
        stmt.accept(self)
    }

    fn is_truthy(&self, value: Value) -> bool {
        match value {
            Value::Nil | Value::Unitialized => false,
            Value::Bool(v) => v,
            _ => true,

        }
    }

    fn runtime_error(operator: &Token, message: &'static str) -> Value{
        ErrorHandler::runtime_error(operator, String::from(message));
        Value::Nil
    }

    fn execute_block(&mut self, statements: &Vec<Box<Stmt>>, environment: Box<Environment>){
        let previous = self.environment.clone();

        self.environment = environment;

        for stmt in statements{
            self.evaluate_stmt(stmt)
        }

        self.environment = previous
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit(&mut self, stmt: &Stmt) -> () {
        match stmt {
            Stmt::Print { expression } => {
                let value = self.evaluate_expr(expression);
                println!("{}", value.interp_to_string())
            },
            Stmt::Expression { expression } => {
                self.evaluate_expr(expression);
            },
            Stmt::Var { name, initializer } => {
                let value = self.evaluate_expr(initializer);
                self.environment.define(name, value)
            },
            Stmt::Block { statements } => {
                self.execute_block(statements, 
                    Box::new(Environment::new_enclosing(self.environment.clone())))
            },
            Stmt::If { condition, then_branch, else_branch } => {
                let condition_result = self.evaluate_expr(condition);

                if self.is_truthy(condition_result){
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

impl ExprVisitor<Value> for Interpreter {
    fn visit(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Assign { name, value } => {
                let value = self.evaluate_expr(value);
                self.environment.assign(name, value.clone());
                return value;
            },
            Expr::Variable { name } => self.environment.get(name.clone()),
            Expr::Literal { value } => value.clone(),
            Expr::Grouping { expression } => self.evaluate_expr(expression),
            Expr::Unary { operator, right } => {
                let right = self.evaluate_expr(right);

                match operator.token_type {
                    TokenType::Bang => {
                        Value::Bool(!self.is_truthy(right))
                    },
                    TokenType::Minus => match right{
                        Value::Number(num) => Value::Number(-num),
                        _ => Interpreter::runtime_error(operator, "Operand must be a number."),
                    } ,
                    _ => Value::Nil
                }
            },
            Expr::Binary { left, operator, right } => {
                let left = self.evaluate_expr(left);
                let right = self.evaluate_expr(right);

                match (left, right) {
                    (Value::String(str1), Value::String(str2)) => {
                        match operator.token_type{
                            TokenType::Plus => Value::String(str1 + &str2),
                            TokenType::Slash | TokenType::Star | TokenType::Minus => Interpreter::runtime_error(operator, "Operands must be numbers."),
                            TokenType::BangEqual => Value::Bool(str1 != str2),
                            TokenType::EqualEqual => Value::Bool(str1 == str2),
                            _ => Value::Nil
                        }
                    },
                    (Value::Number(num1), Value::Number(num2)) => {
                        match operator.token_type {
                            TokenType::Plus => Value::Number(num1 + num2),
                            TokenType::Minus => Value::Number(num1 - num2),
                            TokenType::Slash => Value::Number(num1 / num2),
                            TokenType::Star => Value::Number(num1 * num2),
                            TokenType::Greater => Value::Bool(num1 > num2),
                            TokenType::GreaterEqual => Value::Bool(num1 >= num2),
                            TokenType::Less => Value::Bool(num1 < num2),
                            TokenType::LessEqual => Value::Bool(num1 <= num2),
                            TokenType::BangEqual => Value::Bool(num1 != num2),
                            TokenType::EqualEqual => Value::Bool(num1 == num2),
                            _ => Value::Number(0.0)
                        }
                    },
                    (val1, val2) => {
                        match operator.token_type {
                            TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual |
                            TokenType::Slash | TokenType::Star | TokenType::Minus => Interpreter::runtime_error(operator, "Operands must be numbers."),
                            TokenType::Plus => Interpreter::runtime_error(operator, "Operands must be two numbers or two strings."),
                            TokenType::BangEqual => Value::Bool(!val1.is_equal(val2)),
                            TokenType::EqualEqual => Value::Bool(val1.is_equal(val2)),
                            _ => Value::Nil
                        }
                    }
                }
            },
            _ => Value::Nil
        }
    }
}