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

#[test]
fn value_test() {
    let n = Value::Number(200.01);
    let s = Value::Str("Hello!".to_string());
    let b = Value::Bool(true);

    println!("Reading and eating...");
    eat_and_dump(n);
    eat_and_dump(s);
    eat_and_dump(b);

    // throw errors
    //println!("\nVariables DO NOT exist after dumping {:?}, {:?}, {:?}", n, s, b);

    let nn = Value::Number(404.50);
    let ss = Value::Str("Greetings!".to_string());
    let bb = Value::Bool(false);

    println!("\nReading, no eating...");
    dump(&nn);
    dump(&ss);
    dump(&bb);

    println!("\nVariables still exist after dumping {:?}, {:?}, {:?}", nn, ss, bb);
}