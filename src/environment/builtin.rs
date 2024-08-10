use std::time::{SystemTime, UNIX_EPOCH};

use crate::environment::Object;

pub fn clock(_: Vec<Object>) -> Object {
    let time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
    Object::Number(time.as_millis() as f64)
}