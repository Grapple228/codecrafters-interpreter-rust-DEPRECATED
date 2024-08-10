use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub type MutEnv = Rc<RefCell<Environment>>;
use crate::{environment::{BuiltinSignature, Object}, error::ErrorHandler, token::{Token, TokenType}};

use super::{builtin::clock, BObject};

#[derive(Debug, Clone, PartialEq)]
pub struct Environment{
    pub values: HashMap<String, BObject>,
    enclosing: Option<MutEnv>,
}

impl Environment {
    fn define_builtins(&mut self) {
        self.define_builtin("clock", clock);
    }

    fn define_builtin(&mut self, identificator: &'static str, signature: BuiltinSignature){
        self.define(
            &Token::with_lexeme(identificator.to_owned(), TokenType::Identifier), 
            Box::new(Object::Builtin(identificator.to_owned(), signature))
        )
    }

    pub fn new() -> Self {
        let mut env = Environment{
            values: HashMap::new(), 
            enclosing: None
        };

        env.define_builtins();

        env
    }

    pub fn new_enclosing(enclosing: MutEnv) -> Self{
        Self { values: HashMap::new(), 
            enclosing: Some(enclosing)}
    }

    pub fn define(&mut self, name: &Token, value: BObject) {
        let lexeme = name.lexeme.to_owned();

        if self.values.contains_key(&lexeme){
            ErrorHandler::runtime_error(name, format!("Variable '{}' already defined.", lexeme));
        }

        self.values.insert(lexeme.clone(), value);
    }

    pub fn assign(&mut self, name: &Token, value: BObject) {
        let lexeme = name.lexeme.to_owned();

        if self.values.contains_key(&lexeme){
            self.values.insert(lexeme, value.to_owned());
            return;
        }

        match self.enclosing.as_deref() {
            Some(enclosing ) => {
                enclosing.borrow_mut().assign(name, value.to_owned());
            },
            None => {
                ErrorHandler::runtime_error(name, format!("Undefined variable '{}'.", lexeme));
            },
        }
    }

    pub fn get(&self, name: Token) -> BObject {
        let key = name.lexeme.to_owned();

        if self.values.contains_key(&key){
            let value = self.values.get(&key).unwrap();

            if value.is_equal(Object::Unitialized){
                ErrorHandler::runtime_error(&name, format!("Variable '{}' has not been initialized or assigned to.", key));
                return Box::new(Object::Unitialized)
            }

            return value.to_owned();
        } 

        let value = match self.enclosing.as_deref(){
            Some(enclosing ) => {
                enclosing.borrow().get(name)
            },
            None => {
                ErrorHandler::runtime_error(&name, format!("Undefined variable '{}'.", key));
                Box::new(Object::Nil)
            },
        };
        return value;
    }
}