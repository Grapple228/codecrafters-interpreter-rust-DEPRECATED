use std::{fmt::Display, io::Error};

use crate::{environment::MutEnv, expression::Expr, statement::Stmt};

pub const NIL: Object = Object::Nil;
pub const TRUE: Object = Object::Boolean(true);
pub const FALSE: Object = Object::Boolean(false);

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Number(f64),
    Boolean(bool),
    String(String),
    Array(Vec<Object>),
    Nil,
    Unitialized,
    Return(Box<Object>),
    Function(Vec<Expr>, Box<Stmt>, MutEnv),
    Builtin(String, fn(Vec<Object>) -> Result<Object, Error>),
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
            Object::Function(parameters, body, _) => {
                write!(f, "fn({:?}) {}", csv_str(parameters), body)
            }
            Object::Builtin(name, _) => write!(f, "{}", name),
        }
    }
}