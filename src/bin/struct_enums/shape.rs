pub trait Shape {
    fn show(&self) -> String;
}

impl Shape for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {:?}", self)
    }
}

impl Shape for f32 {
    fn show(&self) -> String {
        format!("eight-byte float {:?}", self)
    }
}