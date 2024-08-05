use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Value{
    Number(f64),
    Bool(bool),
    String(String),
    Nil,
    Unitialized
}

impl Value{
    pub fn is_equal(&self, other: Value) -> bool{
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Unitialized, Value::Unitialized) => true,
            (Value::Nil, _) => false,
            (Value::Unitialized, _) => false,
            (Value::Number(a1), Value::Number(a2)) => a1.clone() == a2,
            (Value::Bool(a1), Value::Bool(a2)) => a1.clone() == a2,
            (Value::String(a1), Value::String(a2)) => *a1 == a2,
            _ => false
        }
    }

    pub fn interp_to_string(&self) -> String {
        match self {
            Value::Number(value) => format!("{}", value),
            Value::Bool(value) => value.to_string(),
            Value::String(value) => value.to_owned(),
            Value::Nil => String::from("nil"),
            Value::Unitialized => String::from("Unitialized"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Number(value) => format!("{:?}", value),
            Value::Bool(value) => value.to_string(),
            Value::String(value) => value.to_owned(),
            Value::Nil => String::from("null"),
            Value::Unitialized => String::from("Unitialized"),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}