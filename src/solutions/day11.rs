use itertools::Itertools;
use std::{fmt::Display, vec};

use super::Solution;

#[derive(Debug, Eq, Clone, Copy)]
enum Space {
    Empty,
    Galaxy(usize, usize),
}

impl PartialEq for Space {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Galaxy(x1, y1), Self::Galaxy(x2, y2)) => x1 == x2 && y1 == y2,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

#[derive(Debug, Eq, Clone, Copy)]
struct Pair {
    galaxy1: Space,
    galaxy2: Space,
}

impl Pair {
    fn new(galaxy1: Space, galaxy2: Space) -> Option<Self> {
        if galaxy1 == galaxy2 {
            return None;
        }
        Some(Self { galaxy1, galaxy2 })
    }
}

impl PartialEq for Pair {
    /// We don't care about the order of the galaxies, only check if the pair contains the same galaxies
    /// Which is the same as checking if the coordinates are the same.
    fn eq(&self, other: &Self) -> bool {
        self.galaxy1 == other.galaxy1 && self.galaxy2 == other.galaxy2
            || self.galaxy1 == other.galaxy2 && self.galaxy2 == other.galaxy1
    }
}

impl Space {
    fn from_char(c: char, x: usize, y: usize) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Galaxy(x, y),
            _ => panic!("Invalid space"),
        }
    }

    fn get_x(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::Galaxy(x, _) => *x,
        }
    }

    fn get_y(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::Galaxy(_, y) => *y,
        }
    }
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Galaxy(_, _) => write!(f, "#"),
        }
    }
}

#[derive(Debug)]
struct Universe {
    grid: Vec<Vec<Space>>,
    galaxies: Vec<Space>,
    rows: Vec<usize>,
    cols: Vec<usize>,
    width: usize,
    #[allow(unused)]
    height: usize,
}

impl Universe {
    fn from_str(input: &str) -> Self {
        let grid = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| Space::from_char(c, x, y))
                    .collect()
            })
            .collect::<Vec<Vec<Space>>>();
        let galaxies = grid
            .iter()
            .flatten()
            .filter(|space| matches!(space, Space::Galaxy(_, _)))
            .cloned()
            .collect::<Vec<Space>>();

        let width = grid[0].len();
        let height = grid.len();
        Self {
            grid,
            galaxies,
            rows: vec![1; height],
            cols: vec![1; width],
            width,
            height,
        }
    }

    /// Expands the universe for each row or column that are empty.
    ///
    /// So an empty row causes to duplicate that row. An empty column duplicates that column.
    fn older_expand_universe(&mut self, times: usize) {
        for (row_index, row) in self.grid.iter().enumerate() {
            if self.is_empty_row(row) {
                self.rows[row_index] = times;
            }
        }
        for col in (0..self.width).rev() {
            if self.grid.iter().all(|row| row[col] == Space::Empty) {
                self.cols[col] = times;
            }
        }

        self.galaxies.iter_mut().for_each(|galaxy| {
            let x = galaxy.get_x();
            let y = galaxy.get_y();
            let new_x = self.rows[0..=y].iter().sum::<usize>();
            let new_y = self.cols[0..=x].iter().sum::<usize>();
            *galaxy = Space::Galaxy(new_x, new_y);
        });
    }

    fn is_empty_row(&self, row: &[Space]) -> bool {
        row.iter().all(|space| *space == Space::Empty)
    }

    /// Calculates the shortest paths between all galaxies.
    fn calculate_shortest_paths(&mut self) -> usize {
        let mut distance_total = 0;

        self.galaxies
            .iter()
            .combinations(2)
            .filter_map(|v| Pair::new(*v[0], *v[1]))
            .dedup()
            .for_each(|pair| {
                let distance = self.calculate_distance(
                    pair.galaxy1.get_x(),
                    pair.galaxy1.get_y(),
                    pair.galaxy2.get_x(),
                    pair.galaxy2.get_y(),
                );
                distance_total += distance;
            });

        distance_total
    }

    /// Calculates the shortest distance between two galaxies.
    /// We can only move up, down, left or right, at each step.
    fn calculate_distance(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
        let dx = if x1 > x2 { x1 - x2 } else { x2 - x1 };
        let dy = if y1 > y2 { y1 - y2 } else { y2 - y1 };
        dx + dy
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part1(input: &str) -> i64 {
    let mut universe = Universe::from_str(input);
    universe.older_expand_universe(2);
    universe.calculate_shortest_paths() as i64
}

fn part2(input: &str) -> i64 {
    let mut universe = Universe::from_str(input);
    universe.older_expand_universe(1000000);
    universe.calculate_shortest_paths() as i64
}

pub struct Day11;

impl Solution for Day11 {
    fn solve_part1(input: &str) -> anyhow::Result<String> {
        Ok(part1(input).to_string())
    }

    fn solve_part2(input: &str) -> anyhow::Result<String> {
        Ok(part2(input).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
            ),
            374
        );
    }

    #[test]
    fn test_part2_by10() {
        let mut universe = Universe::from_str(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        println!("{}", universe);
        println!("expanding...");
        universe.older_expand_universe(10);
        println!("{}", universe);
        let sum = universe.calculate_shortest_paths();
        assert_eq!(sum, 1030);
    }

    #[test]
    fn test_part2_by100() {
        let mut universe = Universe::from_str(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        universe.older_expand_universe(100);
        println!("{}", universe);
        let sum = universe.calculate_shortest_paths();
        assert_eq!(sum, 8410);
    }
}
