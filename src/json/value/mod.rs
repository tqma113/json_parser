use std::collections::HashMap;

pub mod number;
pub mod index;

pub use number::Number;
pub use number::Num;
pub use index::Index;

pub enum Value {
    Null,
    Boolean(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl Value {
    pub fn get<I: Index>(&self, i: I) -> Option<&Value> {
        i.get(self)
    }

    pub fn is_null(&self) -> bool {
        match *self {
            Value::Null => true,
            _ => false
        }
    }

    pub fn as_boolean(&self) -> Boolean {
        match *self {
            Value::Boolean(ref b) => Some(b),
            _ => false
        }
    }

    pub fn is_boolean(&self) -> bool {
        match *self {
            Value::Boolean(_) => true,
            _ => false
        }
    }

    pub fn as_number(&self) -> bool {
        match *self {
            Value::Number(_) => true,
            _ => false
        }
    }

    pub fn as_u64(&self) -> bool {
        match *self {
            Value::Number(&u64) => true,
            _ => false
        }
    }

    pub fn as_i64(&self) -> bool {
        match *self {
            Value::Number(&i64) => true,
            _ => false
        }
    }

    pub fn as_f64(&self) -> bool {
        match *self {
            Value::Number(&f64) => true,
            _ => false
        }
    }

    pub fn is_number(&self) -> bool {
        match *self {
            Value::Number(_) => true,
            _ => false
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Value::String(ref s) => Some(s),
            _ => None
        }
    }

    pub fn is_string(&self) -> bool {
        match *self {
            Value::String(_) => true,
            _ => false
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match *self {
            Value::Array(ref array) => Some(&*array),
            _ => None,
        }
    }

    pub fn is_array(&self) -> bool {
        match *self {
            Value::Array(_) => true,
            _ => false,
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, Value>> {
        match *self {
            Value::Object(ref map) => Some(map),
            _ => None,
        }
    }

    pub fn is_object(&self) -> bool {
        match *self {
            Value::Object(_) => true,
            _ => false,
        }
    }
}
