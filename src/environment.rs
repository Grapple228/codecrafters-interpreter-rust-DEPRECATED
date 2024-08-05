use std::collections::HashMap;

use crate::{error::ErrorHandler, token::Token, value::Value};

#[derive(Clone)]
pub struct Environment{
    values: HashMap<String, Value>,
    enclosing: Option<Box<Environment>>
}

impl Environment {
    pub fn new() -> Self {
        Self { values: HashMap::new(), 
            enclosing: None }
    }

    pub fn new_enclosing(enclosing: Box<Environment>) -> Self{
        Self { values: HashMap::new(), 
            enclosing: Some(enclosing)}
    }

    pub fn define(&mut self, name: &Token, value: Value) {
        let lexeme = &name.lexeme;

        if self.values.contains_key(lexeme){
            ErrorHandler::runtime_error(name, format!("Variable '{}' already defined.", lexeme));
        }

        self.values.insert(lexeme.clone(), value);
    }

    pub fn assign(&mut self, name: &Token, value: Value) {
        let lexeme = &name.lexeme;

        if self.values.contains_key(&lexeme.clone()){
            self.values.insert(lexeme.clone(), value);
            return;
        }
        
        match self.enclosing.as_mut() {
            Some(enclosing ) => {
                enclosing.as_mut().assign(name, value)
            },
            None => {
                ErrorHandler::runtime_error(name, format!("Undefined variable '{}'.", lexeme));
            },
        }

    }

    pub fn get(&self, name: Token) -> Value {
        let key = name.lexeme.clone();

        if self.values.contains_key(&key){
            let value = self.values.get(&key).unwrap();

            if value.is_equal(Value::Unitialized){
                ErrorHandler::runtime_error(&name, format!("Variable '{}' has not been initialized or assigned to.", key));
                return Value::Unitialized
            }

            return value.clone();
        } 

        return match &self.enclosing{
            Some(enclosing ) => {
                enclosing.get(name)
            },
            None => {
                ErrorHandler::runtime_error(&name, format!("Undefined variable '{}'.", key));
                Value::Nil
            },
        }
    }
}