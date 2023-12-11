use std::{collections::HashSet, fmt::Display};

use super::pipe::{Direction, Pipe};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Fill {
    Inside,
    Outside,
    Pipe,
}

impl Display for Fill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Fill::Inside => 'I',
            Fill::Outside => 'O',
            Fill::Pipe => ' ',
        };
        write!(f, "{}", c)
    }
}

pub struct Grid {
    grid: Vec<Vec<Pipe>>,
    loop_pos: HashSet<(usize, usize)>,
    width: usize,
    height: usize,
    cost: Vec<Vec<u32>>,
    inside_outside: Vec<Vec<Fill>>,
    start: (usize, usize),
}

impl Grid {
    pub fn from_str(input: &str) -> Self {
        let mut grid = Vec::new();
        let mut start = (0, 0);
        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let pipe = match c {
                    'S' => {
                        start = (x, y);
                        Pipe::Start
                    }
                    '-' => Pipe::Horizontal,
                    '|' => Pipe::Vertical,
                    '.' => Pipe::Ground,
                    'L' => Pipe::BendNE,
                    'J' => Pipe::BendNW,
                    '7' => Pipe::BendSW,
                    'F' => Pipe::BendSE,
                    _ => panic!("Invalid input"),
                };
                row.push(pipe);
            }
            grid.push(row);
        }

        let width = grid[0].len();
        let height = grid.len();

        Grid {
            grid,
            loop_pos: HashSet::new(),
            width,
            height,
            cost: vec![vec![u32::MAX; width]; height],
            inside_outside: vec![vec![Fill::Pipe; width]; height],
            start,
        }
    }

    /// Build a 3x3 kernel around a pipe, putting Ground pipes where the kernel is out of bounds
    pub fn build_kernel3x3_for_pipe(&self, x: usize, y: usize) -> [[Pipe; 3]; 3] {
        let mut kernel = [[Pipe::Ground; 3]; 3];
        for (i, j) in [
            (0, 0),
            (1, 0),
            (2, 0),
            (0, 1),
            (2, 1),
            (0, 2),
            (1, 2),
            (2, 2),
        ]
        .iter()
        {
            let x = x as i64 + i - 1;
            let y = y as i64 + j - 1;
            if x < 0 || y < 0 || x >= self.width as i64 || y >= self.height as i64 {
                continue;
            }
            let i = *i as usize;
            let j = *j as usize;
            kernel[j][i] = self.grid[y as usize][x as usize];
        }
        kernel
    }

    /// Determine correct Start pipe from kernel 3x3
    /// This is done by checking the 3x3 kernel around the pipe in a +.
    pub fn replace_start_with_pipe(&mut self) {
        let (x, y) = self.start;
        let kernel = self.build_kernel3x3_for_pipe(x, y);
        let top = kernel[0][1];
        let left = kernel[1][0];
        let right = kernel[1][2];
        let bottom = kernel[2][1];
        self.grid[y][x] = match (top, left, right, bottom) {
            (Pipe::Vertical, _, Pipe::Horizontal, _) => Pipe::BendNE,
            (_, Pipe::Horizontal, _, Pipe::Vertical) => Pipe::BendSW,
            (Pipe::Vertical, Pipe::Horizontal, _, _) => Pipe::BendNW,
            (_, _, Pipe::Horizontal, Pipe::Vertical) => Pipe::BendSE,
            (_, Pipe::Horizontal, Pipe::Horizontal, _) => Pipe::Horizontal,
            (Pipe::Vertical, _, _, Pipe::Vertical) => Pipe::Vertical,
            (Pipe::BendNE, Pipe::Horizontal, _, _) => Pipe::BendSW,
            (Pipe::BendNE, _, Pipe::Horizontal, _) => Pipe::BendSE,
            (_, Pipe::Horizontal, _, Pipe::BendSW) => Pipe::BendNW,
            (_, _, Pipe::Horizontal, Pipe::BendSW) => Pipe::BendNE,
            (_, _, Pipe::BendNW, Pipe::Vertical) => Pipe::BendSE,
            (_, Pipe::BendSE, _, Pipe::Vertical) => Pipe::BendSW,
            _ => panic!(
                "Invalid input T:'{:?}' L:'{:?}' R:'{:?}' B:'{:?}'",
                top, left, right, bottom
            ),
        };
    }

    /// Determine the cost of each pipe in the loop from start
    ///
    /// .....    .....
    /// .S-7.    .012.
    /// .|.|.    .1.3.
    /// .L-J.    .234.
    /// .....    .....
    pub fn determine_loop_and_cost(&mut self) -> u32 {
        self.loop_pos.clear();
        self.loop_pos.insert(self.start);
        let (x, y) = self.start;
        self.cost[y][x] = 0;
        let (mut direction1, mut direction2) = self.grid[y][x].start_next_pipes();
        let mut x1 = x;
        let mut y1 = y;
        let mut x2 = x;
        let mut y2 = y;
        let mut cost = 1;
        loop {
            let (node1, node1_pos) = self.pipe(x1, y1, direction1);
            let (node2, node2_pos) = self.pipe(x2, y2, direction2);
            self.loop_pos.insert(node1_pos);
            self.loop_pos.insert(node2_pos);
            // println!("{}({}) {}({}) ", node1, direction1, node2, direction2);
            let (node1_cost, node2_cost) = (
                self.cost[node1_pos.1][node1_pos.0],
                self.cost[node2_pos.1][node2_pos.0],
            );
            if node1_cost != u32::MAX && node2_cost != u32::MAX {
                break;
            }
            if node1_cost == u32::MAX {
                self.cost[node1_pos.1][node1_pos.0] = cost;
            }
            if node2_cost == u32::MAX {
                self.cost[node2_pos.1][node2_pos.0] = cost;
            }
            (direction1, direction2) = (
                node1.next_direction(direction1),
                node2.next_direction(direction2),
            );
            // println!();
            (x1, y1) = node1_pos;
            (x2, y2) = node2_pos;
            cost += 1;
        }
        cost - 1
    }

    fn pipe(&self, x: usize, y: usize, direction: Direction) -> (Pipe, (usize, usize)) {
        let (x, y) = direction.as_indices(x, y);
        (self.grid[y][x], (x, y))
    }

    /// Removes all pipes that are not part of the loop
    pub fn cleanup_pipes(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.cost[y][x] == u32::MAX {
                    self.grid[y][x] = Pipe::Ground;
                }
            }
        }
    }

    /// Determine the bounds of the loop
    pub fn determine_loop_bounds(&self) -> ((usize, usize), (usize, usize)) {
        let mut min_x = self.width;
        let mut min_y = self.height;
        let mut max_x = 0;
        let mut max_y = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.cost[y][x] != u32::MAX {
                    min_x = min_x.min(x);
                    min_y = min_y.min(y);
                    max_x = max_x.max(x);
                    max_y = max_y.max(y);
                }
            }
        }
        ((min_x, min_y), (max_x, max_y))
    }

    pub fn calculate_fill(&mut self) -> usize {
        let ((min_x, min_y), (max_x, max_y)) = self.determine_loop_bounds();
        let mut inside = 0;
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.loop_pos.contains(&(x, y)) {
                    continue;
                }
                let mut count = 0;
                for x2 in x + 1..self.width {
                    let pipe = self.grid[y][x2];
                    if self.loop_pos.contains(&(x2, y)) && pipe.downwards() {
                        count += 1;
                    }
                }
                if count % 2 == 1 {
                    self.inside_outside[y][x] = Fill::Inside;
                    inside += 1;
                } else {
                    self.inside_outside[y][x] = Fill::Outside;
                }
            }
        }
        inside
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.inside_outside[y][x] != Fill::Pipe {
                    write!(f, "{}", self.inside_outside[y][x])?;
                    continue;
                }
                write!(f, "{}", self.grid[y][x])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
