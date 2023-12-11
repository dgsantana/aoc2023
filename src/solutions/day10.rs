use crate::visualize_println;

use self::grid::Grid;

use super::Solution;

mod pipe;
mod grid;


fn part1(input: &str) -> i64 {
    let mut grid = Grid::from_str(input);
    visualize_println!("{}", grid);
    grid.replace_start_with_pipe();
    let cost = grid.determine_loop_and_cost();
    grid.cleanup_pipes();
    visualize_println!("{}", grid);
    cost as i64
}

fn part2(input: &str) -> i64 {
    let mut grid = Grid::from_str(input);
    visualize_println!("{}", grid);
    grid.replace_start_with_pipe();
    grid.determine_loop_and_cost();
    grid.cleanup_pipes();
    grid.determine_loop_bounds();
    let inside = grid.calculate_fill();
    visualize_println!("{}", grid);
    inside as i64
}

pub struct Day10;

impl Solution for Day10 {
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
                "-L|F7
7S-7|
L|7||
-L-J|
L|-JF"
            ),
            4
        );
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(
            part1(
                "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
            ),
            8
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"), 10);
    }
}
