use crate::{token::Token, value::Value};

pub trait Visitor<R> {
    fn visit(&self, expr: &Expr) -> R;
}

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

pub trait AcceptVisitor {
    fn accept<R>(&self, visitor: &impl Visitor<R>) -> R;
}

impl AcceptVisitor for Expr {
    fn accept<R>(&self, visitor: &impl Visitor<R>) -> R {
        visitor.visit(self)
    }
}