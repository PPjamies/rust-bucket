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

#[test]
fn point_test() {
    let tuple = (154, "Pizza", false);

    // tuple borrowed
    let (ref count, ref food, ref is_cold) = tuple;
    println!("Borrowed from tuple: {}, {}, {}", count, food, is_cold);

    // tuple moved after destructing
    let (count, food, is_hot) = tuple;
    println!("Extracted from tuple: {}, {}, {}", count, food, is_hot);

    let point = Point { x: 404.50, y: 159.62 };

    // point copied
    let Point { x, y } = point;
    println!("Copied from point ({}x,{}y)", x, y);

    let mut tuple = (25, "one".to_string());
    match_tuple(tuple); // no match

    tuple = (0, "one".to_string());
    match_tuple(tuple); // no match

    tuple = (0, "zero".to_string());
    match_tuple(tuple); // zero zero

    tuple = (1, "one".to_string());
    match_tuple(tuple); // one one
}