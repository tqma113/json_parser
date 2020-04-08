pub struct Number {
    pub num: Num
}

pub enum Num {
    PosInt(u64),
    NegInt(i64),
    Float(f64),
}
