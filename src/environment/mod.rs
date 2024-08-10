mod builtin;
pub mod environment;

use std::{cell::RefCell, fmt::Display, rc::Rc};

use environment::{Environment, MutEnv};

use crate::{expression::Expr, interpreter::{self, Interpreter}, statement::Stmt, token::Token};

pub type BuiltinSignature = fn(Vec<Object>) -> Object;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Number(f64),
    Boolean(bool),
    String(String),
    Array(Vec<Object>),
    Nil,
    Unitialized,
    Return(Box<Object>),
    Function{
        name: Box<Token>,
        params: Vec<Token>,
        body: Vec<Box<Stmt>>
    },
    Builtin(String, BuiltinSignature),
}

pub fn csv_str<T: Display>(arr: &[T]) -> String {
    arr.iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

impl Object {
    pub fn is_thuthy(self) -> bool {
        match self {
            Object::Nil | Object::Unitialized => false,
            Object::Boolean(v) => v.clone(),
            _ => true,
        }
    }

    pub fn is_equal(&self, other: Object) -> bool {
        match (self, other) {
            (Object::Nil, Object::Nil) => true,
            (Object::Unitialized, Object::Unitialized) => true,
            (Object::Nil, _) => false,
            (Object::Unitialized, _) => false,
            (Object::Number(a1), Object::Number(a2)) => a1.clone() == a2,
            (Object::Boolean(a1), Object::Boolean(a2)) => a1.clone() == a2,
            (Object::String(a1), Object::String(a2)) => *a1 == a2,
            _ => false
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Object::Number(i) => write!(f, "{}", i),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::String(s) => write!(f, "{}", s),
            Object::Array(a) => write!(f, "[{}]", csv_str(a)),
            Object::Nil => write!(f, "nil"),
            Object::Unitialized => write!(f, "unitialized"),
            Object::Return(object) => write!(f, "return {}", object),
            Object::Function{params, body, name} => {
                write!(f, "fn {}({:?}) {:?}", name.lexeme, csv_str(params), body)
            }
            Object::Builtin(name, _) => write!(f, "{}", name),
        }
    }

    
}

impl ObjectCaller<Object> for Object{
    fn is_callable(&self) -> bool{
        match self {
            Object::Function{..} => true,
            Object::Builtin(_, _) => true,
            _ => false
        }
    }
    fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Object {
        match self {
            Object::Function{body, name, params} => {
                let mut env = Environment::new_enclosing(interpreter.globals.clone());

                let mut i = 0;
                while i < params.len() {
                    env.define(params.get(i).unwrap(), arguments.get(i).unwrap().to_owned());
                    i += 1;
                }

                interpreter.execute_block(body, Rc::new(RefCell::new(env)));
                Object::Nil
            },
            Object::Builtin(_, func) => func(arguments),
            _ => Object::Nil
        }
    }
    
    fn arity(&self) -> usize {
        match self {
            Object::Function{params, ..} => params.len(),
            _ => 0
        }
    }
}

pub trait ObjectCaller<R> {
    fn is_callable(&self) -> bool;
    fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> R;
    fn arity(&self) -> usize;
}