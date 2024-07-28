#[derive(Debug)]
struct Plant<'a> {
    lifespan: &'a str,
}

fn main() {
    let lifespan = "40".to_string();
    let plant = Plant { lifespan: &lifespan };
    println!("{:?}", plant)
}