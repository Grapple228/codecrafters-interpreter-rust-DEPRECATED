use crate::environment::{BObject, Object};

pub struct Return{
}

static mut RETURN: Option<BObject> = None;

impl Return{
    pub fn get() -> BObject {
        match unsafe { RETURN.to_owned() } {
            Some(v) => {
                Return::add(Box::new(Object::Nil));
                v
            },
            None => Box::new(Object::Nil),
        }
    }

    pub fn add(value: BObject) {
        unsafe { RETURN = Some(value) }
    }
}