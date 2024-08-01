#[derive(Debug)]
pub enum Value {
    Number(f64),
    Str(String),
    Bool(bool),
}

pub fn eat_and_dump(v: Value) {
    use Value::*;
    match v {
        Number(n) => println!("number is {}", n),
        Str(s) => println!("string is {}", s),
        Bool(b) => println!("boolean is {}", b)
    }
}

pub fn dump(v: &Value) {
    use Value::*;
    match v {
        Number(n) => println!("number is {}", n),
        Str(s) => println!("string is {}", s),
        Bool(b) => println!("boolean is {}", b)
    }
}