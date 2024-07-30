trait Shape {
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

fn main() {
    println!("\nTesting Traits...");
    let integral_number: i32 = 10;
    let floating_number: f32 = 24.4;
    println!("show {}", integral_number.show());
    println!("show {}", floating_number.show());
}