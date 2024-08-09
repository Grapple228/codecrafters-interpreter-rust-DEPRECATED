use std::fmt::Display;

use crate::{error::ParserError, expression::Expr, token::Token};

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt{
    Block{
        statements: Vec<Box<Stmt>>
    },
    Class{
        name: Token,
        superclass: Box<Expr>,
        methods: Vec<Box<Stmt>>
    },
    Expression{
        expression: Box<Expr>
    },
    Function{
        name: Token,
        params: Vec<Token>,
        body: Vec<Box<Stmt>>
    },
    If{
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>
    },
    Print{
        expression: Box<Expr>
    },
    Return{
        keyword: Token,
        value: Box<Expr>
    },
    Var{
        name: Token,
        initializer: Box<Expr>
    },
    While{
        condition: Box<Expr>,
        body: Box<Stmt>
    },
}

impl Stmt {
    pub fn wrap(self) -> Result<Box<Stmt>, ParserError>{
        Ok(Box::new(self))
    }

    pub fn accept<R>(&self, visitor: &mut impl StmtVisitor<R>) -> () {
        visitor.visit(self)
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}

pub trait StmtVisitor<R> {
    fn visit(&mut self, stmt: &Stmt) -> ();
}
