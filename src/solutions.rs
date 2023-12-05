mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub use day1::Day1;
pub use day2::Day2;
pub use day3::Day3;
pub use day4::Day4;
pub use day5::Day5;
pub use day6::Day6;
pub use day7::Day7;
pub use day8::Day8;
pub use day9::Day9;
pub use day10::Day10;
pub use day11::Day11;
pub use day12::Day12;
pub use day13::Day13;
pub use day14::Day14;
pub use day15::Day15;
pub use day16::Day16;
pub use day17::Day17;
pub use day18::Day18;
pub use day19::Day19;
pub use day20::Day20;
pub use day21::Day21;
pub use day22::Day22;
pub use day23::Day23;
pub use day24::Day24;
pub use day25::Day25;

pub fn read_data(day: u32) -> String {
    let path = format!("inputs/day{}.txt", day);
    std::fs::read_to_string(path).unwrap()
}

pub trait Solution {
    fn solve_part1(input: &str) -> anyhow::Result<String>;
    fn solve_part2(input: &str) -> anyhow::Result<String>;
}