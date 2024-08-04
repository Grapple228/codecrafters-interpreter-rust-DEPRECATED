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

impl AcceptVisitor for Expr {
    fn accept<R>(&self, visitor: &impl Visitor<R>) -> R {
        visitor.visit(self)
    }
}

pub trait AcceptVisitor {
    fn accept<R>(&self, visitor: &impl Visitor<R>) -> R;
}

pub trait Visitor<R> {
    fn visit(&self, expr: &Expr) -> R;
}