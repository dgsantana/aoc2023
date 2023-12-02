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

#[allow(unused)]
pub fn read_sample_input(day: u32, part: u32) -> String {
    let path = format!("inputs/day{}_sample_p{}.txt", day, part);
    std::fs::read_to_string(path).unwrap()
}

pub fn read_data(day: u32) -> String {
    let path = format!("inputs/day{}.txt", day);
    std::fs::read_to_string(path).unwrap()
}

pub trait Solution {
    fn solve_part1(input: &str) -> anyhow::Result<String>;
    fn solve_part2(input: &str) -> anyhow::Result<String>;
}