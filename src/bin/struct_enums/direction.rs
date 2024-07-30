use crate::Direction::{Down, Left, Right, Up};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn direction_as_string(direction: Direction) -> &'static str {
        match direction {
            Up => "Up",
            Down => "Down",
            Left => "Left",
            Right => "Right"
        }
    }

    fn as_str(&self) -> &str {
        use Direction::*;
        match *self {
            Up => "Up",
            Down => "Down",
            Left => "Left",
            Right => "Right"
        }
    }

    fn next(&self) -> Direction {
        use Direction::*;
        match *self {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up,
        }
    }
}

fn main() {
    println!("All possible moves: {}, {}, {}, {}",
             Direction::direction_as_string(Up),
             Direction::direction_as_string(Down),
             Direction::direction_as_string(Left),
             Direction::direction_as_string(Right)
    );

    let mut direction: Direction = Up;
    println!("My starting direction is: {:?}", direction);

    for _ in 0..8 {
        direction = direction.next();
        println!("I go: {:?}", direction)
    }
}