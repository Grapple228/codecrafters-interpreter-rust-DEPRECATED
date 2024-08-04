use std::{collections::HashMap, error::Error};

use crate::{error::ErrorHandler, token::Token, value::Value};

pub struct Environment{
    values: HashMap<String, Value>
}

impl Environment {
    pub fn new() -> Self {
        Self { values: HashMap::new() }
    }

    pub fn define(&mut self, name:String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Value {
        let key = name.lexeme.clone();

        match self.values.get(&key){
            Some(value) => value.clone(),
            None => {
                ErrorHandler::error_token(name, format!("Undefined variable '{}'.", key));
                Value::Nil
            },
        }
    }
}