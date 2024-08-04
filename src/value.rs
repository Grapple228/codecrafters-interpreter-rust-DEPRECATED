use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Value{
    Number(f64),
    Bool(bool),
    String(String),
    Nil,
}

impl Value{
    pub fn to_string(&self) -> String {
        match self {
            Value::Number(value) => format!("{:?}", value),
            Value::Bool(value) => value.to_string(),
            Value::String(value) => value.to_owned(),
            Value::Nil => String::from("null"),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}