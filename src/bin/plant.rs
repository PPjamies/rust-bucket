#[derive(Debug)]
pub struct Plant<'a> {
    pub lifespan: &'a str,
}

#[test]
fn plant_test() {
    let lifespan = "40".to_string();
    let plant = Plant { lifespan: &lifespan };
    println!("{:?}", plant);
}