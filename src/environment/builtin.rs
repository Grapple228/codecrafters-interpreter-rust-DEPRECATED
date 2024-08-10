use std::time::{SystemTime, UNIX_EPOCH};

use crate::environment::Object;

use super::{Args, BObject};

pub fn clock(_: Args) -> BObject {
    let time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
    Box::new(Object::Number(time.as_millis() as f64))
}