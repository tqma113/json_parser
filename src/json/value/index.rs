use super::Value;

pub trait Index {
    #[doc(hidden)]
    fn get<'v>(&self, v: &'v Value) -> Option<&'v Value>;
}

impl Index for usize {
    fn get<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match *v {
            Value::Array(ref vec) => vec.get(*self),
            _ => None,
        }
    }
}

impl Index for str {
    fn get<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match *v {
            Value::Object(ref map) => map.get(self),
            _ => None,
        }
    }
}

impl Index for String {
    fn get<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        self[..].index_into(v)
    }
}