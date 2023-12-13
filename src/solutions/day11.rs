use itertools::Itertools;
use std::{fmt::Display, vec};

use super::Solution;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Space {
    Empty,
    Galaxy(usize, usize, usize),
}

impl Space {
    fn from_char(c: char, index: &mut usize, x: usize, y: usize) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => {
                *index += 1;
                Self::Galaxy(*index, x, y)
            }
            _ => panic!("Invalid space"),
        }
    }

    fn get_x(&self) -> usize {
        match self {
            Self::Empty => panic!("Empty space has no coordinates"),
            Self::Galaxy(_, x, _) => *x,
        }
    }

    fn get_y(&self) -> usize {
        match self {
            Self::Empty => panic!("Empty space has no coordinates"),
            Self::Galaxy(_, _, y) => *y,
        }
    }
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Galaxy(_, _, _) => write!(f, "#"),
        }
    }
}

#[derive(Debug)]
struct Universe {
    grid: Vec<Vec<Space>>,
    rows: Vec<usize>,
    cols: Vec<usize>,
    width: usize,
    height: usize,
    distances: Vec<(Space, Space, usize)>,
}

impl Universe {
    fn from_str(input: &str) -> Self {
        let mut galaxies = 0;
        let grid = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| Space::from_char(c, &mut galaxies, x, y))
                    .collect()
            })
            .collect::<Vec<Vec<Space>>>();
        let width = grid[0].len();
        let height = grid.len();
        Self {
            grid,
            rows: vec![1; height],
            cols: vec![1; width],
            width,
            height,
            distances: Vec::new(),
        }
    }

    /// Expands the universe for each row or column that are empty.
    ///
    /// So an empty row causes to duplicate that row. An empty column duplicates that column.
    fn expand_universe(&mut self) {
        let mut new_grid = Vec::new();
        for row in self.grid.iter() {
            if self.is_empty_row(row) {
                new_grid.push(row.clone());
                new_grid.push(row.clone());
            } else {
                new_grid.push(row.clone());
            }
        }
        for col in (0..self.width).rev() {
            if new_grid.iter().all(|row| row[col] == Space::Empty) {
                for row in new_grid.iter_mut() {
                    row.insert(col, Space::Empty);
                }
            }
        }
        // Update width and height and galaxy coordinates
        self.width = new_grid[0].len();
        self.height = new_grid.len();
        for (y, row) in new_grid.iter_mut().enumerate() {
            for (x, space) in row.iter_mut().enumerate() {
                if let Space::Galaxy(index, _, _) = space {
                    *space = Space::Galaxy(*index, x, y);
                }
            }
        }
        self.grid = new_grid;
    }

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

        for (y, row) in self.grid.iter_mut().enumerate() {
            for (x, space) in row.iter_mut().enumerate() {
                if let Space::Galaxy(index, _, _) = space {
                    let new_x = self.rows[0..=y].iter().sum::<usize>();
                    let new_y = self.cols[0..=x].iter().sum::<usize>();
                    *space = Space::Galaxy(*index, new_x, new_y);
                }
            }
        }
    }

    fn is_empty_row(&self, row: &[Space]) -> bool {
        row.iter().all(|space| *space == Space::Empty)
    }

    /// Calculates the shortest paths between all galaxies.
    /// Only move up, down, left or right.
    fn calculate_shortest_paths(&mut self) {
        let mut distances = Vec::new();

        let galaxies = self
            .grid
            .iter()
            .flatten()
            .filter(|space| matches!(space, Space::Galaxy(_, _, _)))
            .collect::<Vec<&Space>>();

        for galaxy in galaxies.iter() {
            println!("{:?}", galaxy);
        }

        for galaxy1 in galaxies.iter() {
            if let Space::Galaxy(_, _, _) = galaxy1 {
                for galaxy2 in galaxies.iter() {
                    if galaxy1 == galaxy2 {
                        continue;
                    }
                    if Self::has_galaxy(&distances, **galaxy1, **galaxy2) {
                        continue;
                    }
                    if let Space::Galaxy(_, _, _) = galaxy2 {
                        let distance = self.calculate_distance(
                            galaxy1.get_x(),
                            galaxy1.get_y(),
                            galaxy2.get_x(),
                            galaxy2.get_y(),
                        );
                        distances.push((**galaxy1, **galaxy2, distance));
                    }
                }
            }
        }

        self.distances = distances;
    }

    fn has_galaxy(distances: &[(Space, Space, usize)], galaxy1: Space, galaxy2: Space) -> bool {
        distances
            .iter()
            .any(|(g1, g2, _)| *g1 == galaxy1 && *g2 == galaxy2 || *g1 == galaxy2 && *g2 == galaxy1)
    }

    /// Calculates the shortest distance between two galaxies.
    /// We can only move up, down, left or right, at each step.
    ///
    /// This is almost like drawing a line between the two galaxies, but we can only move
    /// horizontally or vertically in each step, aka line drawing a pixel line.
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
    println!("{}", universe);
    println!("expanding...");
    universe.expand_universe();
    println!("{}", universe);
    universe.calculate_shortest_paths();
    universe.distances.iter().map(|(_, _, d)| *d as i64).sum()
}

fn part2(input: &str) -> i64 {
    let mut universe = Universe::from_str(input);
    println!("{}", universe);
    println!("expanding...");
    universe.older_expand_universe(1000000);
    println!("{}", universe);
    universe.calculate_shortest_paths();
    universe.distances.iter().map(|(_, _, d)| *d as i64).sum()
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
        let mut universe = Universe::from_str("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....");
        println!("{}", universe);
        println!("expanding...");
        universe.older_expand_universe(10);
        println!("{}", universe);
        universe.calculate_shortest_paths();
        let sum: i64 = universe.distances.iter().map(|(_, _, d)| *d as i64).sum();
        assert_eq!(
            sum,
            1030
        );
    }

    #[test]
    fn test_part2_by100() {
        let mut universe = Universe::from_str("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....");
        println!("{}", universe);
        println!("expanding...");
        universe.older_expand_universe(100);
        println!("{}", universe);
        universe.calculate_shortest_paths();
        let sum: i64 = universe.distances.iter().map(|(_, _, d)| *d as i64).sum();
        assert_eq!(
            sum,
            8410
        );
    }
}
