#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub fn match_tuple(tuple: (i32, String)) {
    let text = match tuple {
        (0, s) if s == "zero" => format!("zero {}", s),
        (1, ref s) if s == "one" => format!("one {}", s),
        _ => format!("no match")
    };
    println!("{:?}", text)
}