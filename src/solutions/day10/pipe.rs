use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn as_indices(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pipe {
    Start,
    Ground,
    Vertical,
    Horizontal,
    BendNE,
    BendNW,
    BendSE,
    BendSW,
}

impl Pipe {

    pub fn downwards(&self) -> bool {
        matches!(self, Pipe::Vertical | Pipe::BendSE | Pipe::BendSW)
    }

    pub fn start_next_pipes(&self) -> (Direction, Direction) {
        let (branch1, branch2) = match &self {
            Pipe::Vertical => (Direction::Up, Direction::Down),
            Pipe::Horizontal => (Direction::Left, Direction::Right),
            Pipe::BendNE => (Direction::Up, Direction::Right),
            Pipe::BendNW => (Direction::Up, Direction::Left),
            Pipe::BendSE => (Direction::Down, Direction::Right),
            Pipe::BendSW => (Direction::Down, Direction::Left),
            _ => unreachable!(),
        };
        // println!("{} -> {} {}", &self, branch1, branch2);
        (branch1, branch2)
    }

    pub fn next_direction(&self, previous_direction: Direction) -> Direction {
        let next = match &self {
            Pipe::Vertical => match previous_direction {
                Direction::Up => Direction::Up,
                Direction::Down => Direction::Down,
                _ => panic!("{:?} -> {}", &self, previous_direction),
            },
            Pipe::Horizontal => match previous_direction {
                Direction::Left => Direction::Left,
                Direction::Right => Direction::Right,
                _ => panic!("{:?} -> {}", &self, previous_direction),
            },
            Pipe::BendNE => match previous_direction {
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                _ => panic!("{:?} -> {}", &self, previous_direction),
            },
            Pipe::BendNW => match previous_direction {
                Direction::Down => Direction::Left,
                Direction::Right => Direction::Up,
                _ => panic!("{:?} -> {}", &self, previous_direction),
            },
            Pipe::BendSE => match previous_direction {
                Direction::Up => Direction::Right,
                Direction::Left => Direction::Down,
                _ => panic!("{:?} -> {}", &self, previous_direction),
            },
            Pipe::BendSW => match previous_direction {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Down,
                _ => panic!("{:?} -> {}", &self, previous_direction),
            },
            _ => panic!("{} -> {}", &self, previous_direction),
        };
        // println!("{} -> {} -> {}", previous_direction, &self, next);
        next
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Pipe::Start => 'S',
            Pipe::Ground => ' ',
            Pipe::Vertical => '│',
            Pipe::Horizontal => '─',
            Pipe::BendNE => '└',
            Pipe::BendNW => '┘',
            Pipe::BendSE => '┌',
            Pipe::BendSW => '┐',
        };
        write!(f, "{}", c)
    }
}