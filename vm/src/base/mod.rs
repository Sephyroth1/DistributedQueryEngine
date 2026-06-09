pub enum Value {
    INT(i64),
    FLOAT(f64),
    STRING(String),
    BOOL(bool),
    NULL,
}

pub enum Instruction {
    SCAN(Value),
    LOAD(Value),
}
