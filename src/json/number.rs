pub struct Number {
    num: Num
}

enum Num {
    PosInt(u64),
    NegInt(i64),
    Float(f64),
}
