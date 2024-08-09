use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub type MutEnv = Rc<RefCell<Environment>>;
use crate::{error::ErrorHandler, object::Object, token::Token};

#[derive(Debug, Clone, PartialEq)]
pub struct Environment{
    pub values: HashMap<String, Object>,
    enclosing: Option<MutEnv>,
}

impl Environment {
    pub fn new() -> Self {
        Self { values: HashMap::new(), 
            enclosing: None}
    }

    pub fn new_enclosing(enclosing: MutEnv) -> Self{
        Self { values: HashMap::new(), 
            enclosing: Some(enclosing)}
    }

    pub fn define(&mut self, name: &Token, value: Object) {
        let lexeme = &name.lexeme;

        if self.values.contains_key(lexeme){
            ErrorHandler::runtime_error(name, format!("Variable '{}' already defined.", lexeme));
        }

        self.values.insert(lexeme.clone(), value);
    }

    pub fn assign(&mut self, name: &Token, value: Object) {
        let lexeme = &name.lexeme;

        if self.values.contains_key(&lexeme.clone()){
            self.values.insert(lexeme.clone(), value.clone());
            return;
        }

        match self.enclosing.as_deref() {
            Some(enclosing ) => {
                enclosing.borrow_mut().assign(name, value.clone());
            },
            None => {
                ErrorHandler::runtime_error(name, format!("Undefined variable '{}'.", lexeme));
            },
        }
    }

    pub fn get(&self, name: Token) -> Object {
        let key = name.lexeme.clone();

        if self.values.contains_key(&key){
            let value = self.values.get(&key).unwrap();

            if value.is_equal(Object::Unitialized){
                ErrorHandler::runtime_error(&name, format!("Variable '{}' has not been initialized or assigned to.", key));
                return Object::Unitialized
            }

            return value.to_owned();
        } 

        let value = match self.enclosing.as_deref(){
            Some(enclosing ) => {
                enclosing.borrow().get(name)
            },
            None => {
                ErrorHandler::runtime_error(&name, format!("Undefined variable '{}'.", key));
                Object::Nil
            },
        };
        return value;
    }
}