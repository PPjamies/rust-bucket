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