use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn from_line(line: &str) -> Vec<Self> {
        line.chars().map(|c| c.into()).collect()
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction: {}", value),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Direction::Left => "L",
            Direction::Right => "R",
        };
        write!(f, "{}", value)
    }
}
