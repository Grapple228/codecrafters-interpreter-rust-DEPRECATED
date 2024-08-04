use crate::{token::Token, value::Value};

#[derive(Debug)]
pub enum Expr {
    Assign{
        name: Token,
        value: Box<Expr>
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call{
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Box<Expr>>
    },
    Get{
        object: Box<Expr>,
        name: Token
    },
    Grouping{
        expression: Box<Expr>
    },
    Literal{
        value: Value
    },
    Logical{
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Set{
        left: Box<Expr>,
        name: Token,
        right: Box<Expr>,
    },
    Super{
        keyword: Token,
        method: Token
    },
    This{
        keyword: Token,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable{
        name: Token
    },
}

impl Expr {
    pub fn accept<R>(&self, visitor: &impl ExprVisitor<R>) -> R {
        visitor.visit(self)
    }
}

pub trait ExprVisitor<R> {
    fn visit(&self, a: &Expr) -> R;
}