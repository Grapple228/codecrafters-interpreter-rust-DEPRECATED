use crate::{expression::Expr, token::Token};

#[derive(Debug)]
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
        else_branch: Box<Stmt>
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
    pub fn accept<R>(&self, visitor: &mut impl StmtVisitor<R>) -> () {
        visitor.visit(self)
    }
}

pub trait StmtVisitor<R> {
    fn visit(&mut self, stmt: &Stmt) -> ();
}