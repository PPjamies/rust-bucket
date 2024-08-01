use std::ops::Add;

use crate::bank_balance::BankBalance;
use crate::bucket::Bucket;
use crate::direction::Direction::{Down, Left, Right, Up};
use crate::person::Person;
use crate::plant::Plant;
use crate::shape::Shape;
use crate::value::{dump, eat_and_dump, Value};

mod bank_balance;
mod plant;
mod bucket;
mod candy;
mod direction;
mod person;
mod point;
mod shape;
mod value;

fn main() {
    // bank_balance.rs
    let bank_balance = BankBalance { val: 800.45 };
    let other_bank_balance = BankBalance { val: 1001.25 };
    println!("Total bank balance is: {}", bank_balance.add(other_bank_balance));

    // bucket.rs
    let bucket = Bucket { val: 12 };
    let mut is_same = bucket.is_same(Bucket { val: 12 });
    println!("are buckets the same: {}", is_same);

    is_same = bucket.is_same(Bucket { val: 14 });
    println!("are buckets the same: {}", is_same);

    // candy.rs
    let candy_bag = candy::create_candy_bag(
        "skittles".to_string(),
        20,
        5,
    );

    for scoop in candy_bag {
        println!("Candy left: {} ", scoop)
    }
    println!("No more candy left!!!");

    // direction.rs
    println!("All possible moves: {}, {}, {}, {}",
             direction::Direction::direction_as_string(Up),
             direction::Direction::direction_as_string(Down),
             direction::Direction::direction_as_string(Left),
             direction::Direction::direction_as_string(Right)
    );

    let mut direction: direction::Direction = Up;
    println!("My starting direction is: {:?}", direction);

    for _ in 0..8 {
        direction = direction.next();
        println!("I go: {:?}", direction)
    }

    // person.rs
    println!("\nTesting Structs...");
    let mut person = Person::new(
        "PJ",
        "Jindrich",
        &27,
        "female",
    );
    person.print();

    person.set_age(&28);
    person.print();

    let another_person = person.copy();
    another_person.print();

    println!("Prints only the age: {:?}", another_person);

    // plant.rs
    let lifespan = "40".to_string();
    let plant = Plant { lifespan: &lifespan };
    println!("{:?}", plant);

    // point.rs
    let tuple = (154, "Pizza", false);

    // tuple borrowed
    let (ref count, ref food, ref is_cold) = tuple;
    println!("Borrowed from tuple: {}, {}, {}", count, food, is_cold);

    // tuple moved after destructing
    let (count, food, is_hot) = tuple;
    println!("Extracted from tuple: {}, {}, {}", count, food, is_hot);

    let point = point::Point { x: 404.50, y: 159.62 };

    // point copied
    let point::Point { x, y } = point;
    println!("Copied from point ({}x,{}y)", x, y);

    let mut tuple = (25, "one".to_string());
    point::match_tuple(tuple); // no match

    tuple = (0, "one".to_string());
    point::match_tuple(tuple); // no match

    tuple = (0, "zero".to_string());
    point::match_tuple(tuple); // zero zero

    tuple = (1, "one".to_string());
    point::match_tuple(tuple); // one one

    // shape.rs
    println!("\nTesting Traits...");
    let integral_number: i32 = 10;
    let floating_number: f32 = 24.4;
    println!("show {}", integral_number.show());
    println!("show {}", floating_number.show());

    // value.rs
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