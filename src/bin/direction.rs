use crate::direction::Direction::{Down, Left, Right, Up};

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn direction_as_string(direction: Direction) -> &'static str {
        use Direction::*;
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

    pub fn next(&self) -> Direction {
        use Direction::*;
        match *self {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up,
        }
    }
}

#[test]
fn direction_test() {
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