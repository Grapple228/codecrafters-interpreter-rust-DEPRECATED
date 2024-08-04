use crate::{error::ErrorHandler, expression::{AcceptVisitor, Expr, Visitor}, token::{self, TokenType}, value::Value};

pub struct Interpreter{

}

impl Interpreter {
    pub fn new() -> Self {
        Self {  }
    }
    
    pub fn evaluate(&self, expr: &Box<Expr>) -> Value {
        expr.accept(self)
    }

    fn is_truthy(&self, value: Value) -> bool {
        match value {
            Value::Nil => false,
            Value::Bool(v) => v,
            _ => true,

        }
    }
}

impl Visitor<Value> for Interpreter {
    fn visit(&self, expr: &Expr) -> Value {
        match expr {
            Expr::Literal { value } => value.clone(),
            Expr::Grouping { expression } => self.evaluate(expression),
            Expr::Unary { operator, right } => {
                let right = self.evaluate(right);

                match operator.token_type {
                    TokenType::Bang => {
                        Value::Bool(!self.is_truthy(right))
                    },
                    TokenType::Minus => match right{
                        Value::Number(num) => Value::Number(-num),
                        _ => {
                            ErrorHandler::error(operator.line, String::from("Operand must be a number."));
                            Value::Nil
                        },
                    } ,
                    _ => Value::Nil
                }
            },
            Expr::Binary { left, operator, right } => {
                let left = self.evaluate(left);
                let right = self.evaluate(right);

                //println!("{} {} {}", left, operator.lexeme, right);

                match (left, right) {
                    (Value::String(str1), Value::String(str2)) => {
                        match operator.token_type{
                            TokenType::Plus => Value::String(str1 + &str2),
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
                            TokenType::Slash | TokenType::Star | TokenType::Minus => 
                            {
                                ErrorHandler::error(operator.line, String::from("Operands must be numbers."));
                                Value::Nil
                            },
                            TokenType::Plus => {
                                ErrorHandler::error(operator.line, String::from("Operands must be two numbers or two strings."));
                                Value::Nil
                            },
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