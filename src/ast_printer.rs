use crate::expression::{Expr, Visitor};
use crate::expression::AcceptVisitor;
use crate::value::Value;
pub struct AstPrinter{}

impl AstPrinter {
    pub fn print(&self, expr: Box<Expr>) -> String{
        expr.accept(self)
    }

    pub fn new() -> Self {
        Self {  }
    }
}

impl AstPrinter {
    fn parenthesize(&self, name: String, expressions: Vec<&Box<Expr>>) -> String{
        let mut builder = String::from("(");
        builder.push_str(&name);
        
        for expr in expressions.iter(){
            builder.push_str(" ");
            builder.push_str(expr.accept(self).as_str());
        }
        builder.push_str(")");

        return builder;
    }
}

impl Visitor<String> for AstPrinter {
    fn visit(&self, expr: &Expr) -> String {
            match expr {
                Expr::Assign { name, value } => todo!(),
                Expr::Binary { left, operator, right } => {
                    self.parenthesize(operator.lexeme.clone(), vec![left, right])
                },
                Expr::Call { callee, paren, arguments } => todo!(),
                Expr::Get { object, name } => todo!(),
                Expr::Grouping { expression } => {
                    self.parenthesize(String::from("group"), 
                                      vec![expression])
                },
                Expr::Literal { value } => {
                    if *value == Value::Nil{
                        return String::from("nil");
                    }
                    value.to_string()
                },
                Expr::Logical { left, operator, right } => todo!(),
                Expr::Set { left, name, right } => todo!(),
                Expr::Super { keyword, method } => todo!(),
                Expr::This { keyword } => todo!(),
                Expr::Unary { operator, right } => {
                    self.parenthesize(operator.lexeme.clone(), vec![right])
                },
                Expr::Variable { name } => todo!(),
            }
        }
}