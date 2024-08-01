#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // into_iter() returns any T, &T, &mut T
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filter_test() {
    let shoes = vec![
        Shoe { size: 7, style: String::from("sneaker") },
        Shoe { size: 8, style: String::from("sandal") },
        Shoe { size: 7, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 7);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 7, style: String::from("sneaker") },
            Shoe { size: 7, style: String::from("boot") },
        ]
    );
}