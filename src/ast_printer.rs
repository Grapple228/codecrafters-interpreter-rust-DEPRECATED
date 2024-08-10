use crate::{environment::Object, expression::{Expr, ExprVisitor}};
pub struct AstPrinter{}

impl AstPrinter {
    pub fn print(&mut self, expr: Box<Expr>) -> String{
        expr.accept(self)
    }

    pub fn new() -> Self {
        Self {  }
    }
}

impl AstPrinter {
    fn parenthesize(&mut self, name: String, expressions: Box<[&Box<Expr>]>) -> String{
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

impl ExprVisitor<String> for AstPrinter {
    fn visit(&mut self, expr: &Expr) -> String {
            match expr {
                Expr::Binary { left, operator, right } => {
                    self.parenthesize(operator.lexeme.clone(), vec![left, right].into_boxed_slice())
                },
                Expr::Grouping { expression } => {
                    self.parenthesize(String::from("group"), 
                                      vec![expression].into_boxed_slice())
                },
                Expr::Literal { value } => {
                    value.to_string()
                },
                Expr::Unary { operator, right } => {
                    self.parenthesize(operator.lexeme.clone(), vec![right].into_boxed_slice())
                },
                _ => { todo!() }
            }
        }
}