use std::fmt::Display;

use crate::{error::ParserError, token::Token, environment::Object};

pub type MyResult<T> = std::result::Result<Box<T>, ParserError>;

#[derive(Debug, Clone, PartialEq)]
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
        value: Object
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

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Expr {
    pub fn wrap(self) -> MyResult<Expr>{
        Ok(Box::new(self))
    }

    pub fn accept<R>(&self, visitor: &mut impl ExprVisitor<R>) -> R {
        visitor.visit(self)
    }
}

pub trait ExprVisitor<R> {
    fn visit(&mut self, a: &Expr) -> R;
}