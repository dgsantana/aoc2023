use std::ops::RangeInclusive;

use console::Style;
use itertools::Itertools;

use super::Solution;

const SYMBOLS: &str = "!\"#$%&/()=?@{[]}'?«»<>|\\*+~^;,:-";
const DIRECTIONS: [(i32, i32); 8] = [
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
];

fn part1(input: &str) -> u32 {
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().len() as i32;
    let input = input
        .lines()
        .flat_map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let mut numbers: Vec<u32> = Vec::new();

    for y in 0..height {
        let mut numbers_in_line = Vec::new();
        let mut near_symbol = false;
        for x in 0..width {
            let i = (y * height + x) as usize;
            let Some(&c) = input.get(i) else {
                continue;
            };
            if *c == b'.' {
                if near_symbol && !numbers_in_line.is_empty() {
                    build_number(&numbers_in_line, &mut numbers);
                }
                numbers_in_line.clear();
                near_symbol = false;
            } else if SYMBOLS.contains(*c as char) {
                if !numbers_in_line.is_empty() {
                    build_number(&numbers_in_line, &mut numbers);
                }
                numbers_in_line.clear();
            } else if c.is_ascii_digit() {
                numbers_in_line.push((c - b'0') as u32);
                if !near_symbol && search_symbol(x, y, &input, height) {
                    near_symbol = true;
                }
            }
        }
        if !numbers_in_line.is_empty() && near_symbol {
            build_number(&numbers_in_line, &mut numbers);
        }
    }
    numbers.iter().sum()
}

fn search_symbol(x: i32, y: i32, input: &[&u8], height: i32) -> bool {
    let (x, y) = (x, y);
    DIRECTIONS
        .iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .map(|(nx, ny)| (ny * height + nx) as usize)
        .filter_map(|i| input.get(i))
        .filter(|&&c| SYMBOLS.contains(*c as char))
        .count()
        > 0
}

fn build_number(numbers_in_line: &[u32], numbers: &mut Vec<u32>) -> u32 {
    let max = numbers_in_line.len() - 1;
    let value = numbers_in_line
        .iter()
        .enumerate()
        .map(|(i, v)| 10_u32.pow((max - i) as u32) * v)
        .sum::<u32>();
    numbers.push(value);
    value
}

fn part2(input: &str) -> u32 {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let input = input
        .lines()
        .flat_map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let mut numbers: Vec<Number> = Vec::new();
    let gears_positions = input
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| if *c == b'*' { Some(i) } else { None })
        .collect::<Vec<usize>>();

    let mut gears = gears_positions
        .iter()
        .filter_map(|i| {
            let (x, y) = (i % width, i / height);
            let max_size = width * height;
            let margin = DIRECTIONS
                .iter()
                .filter_map(|(dx, dy)| {
                    let (x, y) = (x as i32, y as i32);
                    let height = height as i32;
                    let index = (height * (y + dy) + (x + dx)) as usize;
                    if index > 0
                        && index < max_size
                        && input.get(index).map(|c| c.is_ascii_digit()) == Some(true)
                    {
                        Some(index)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            if margin.len() > 1 {
                Some(Gear {
                    position: (x, y),
                    possible_index: margin,
                    numbers: Vec::new(),
                    product: 0,
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut is_number = false;
    for (i, &c) in input.iter().enumerate() {
        if i % width == 0 {
            is_number = false;
        }
        match is_number {
            true => {
                if c.is_ascii_digit() {
                    let number = numbers.last_mut().unwrap();
                    number.value = number.value * 10 + (c - b'0') as u32;
                    number.range = *number.range.start()..=i;
                } else {
                    is_number = false;
                }
            }
            false => {
                if c.is_ascii_digit() {
                    numbers.push(Number {
                        range: i..=i,
                        value: (c - b'0') as u32,
                        y: i / height,
                    });
                    is_number = true;
                }
            }
        }
    }

    let gears = gears
        .iter_mut()
        .filter_map(|gear| {
            for number in numbers.iter() {
                for points in gear.possible_index.iter() {
                    if number.range.contains(points) {
                        gear.numbers.push(number.clone());
                        break;
                    }
                }
            }
            if gear.numbers.len() == 2 {
                gear.product = gear.numbers.iter().map(|v| v.value).product::<u32>();
                Some(gear)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let result = gears.iter().map(|g| g.product).sum::<u32>();

    if cfg!(feature = "visualize") {
        let gear = Style::new().red().bold().apply_to("*");
        let gold = Style::new().bright().yellow().bold();
        let gray = Style::new().red().bold();
        for (i, &c) in input.iter().enumerate() {
            if i % width == 0 {
                print!(" ");
                let y = i / height;
                if y > 0 {
                    let sum = gears
                        .iter()
                        .filter(|g| g.position.1 == y - 1)
                        .map(|v| v.numbers[0].value * v.numbers[1].value)
                        .sum::<u32>();
                    let sum_parts: String = gears
                        .iter()
                        .filter(|g| g.position.1 == y - 1)
                        .map(|g| format!("{}*{}", g.numbers[0].value, g.numbers[1].value))
                        .join("+");
                    if sum > 0 {
                        print!("{} = {}", sum, sum_parts);
                    }
                }
                println!();
            }
            if gears
                .iter()
                .any(|g| g.position.0 == i % width && g.position.1 == i / height)
                || gears
                    .iter()
                    .any(|g| g.numbers.iter().any(|n| n.range.contains(&i)))
            {
                print!("{}", gold.apply_to(*c as char));
            } else if *c == b'*' {
                print!("{}", gear);
            } else if SYMBOLS.contains(*c as char) || *c == b'.' {
                print!(" ");
            } else if c.is_ascii_digit() {
                print!("{}", gray.apply_to(*c as char));
            } else {
                print!("{}", *c as char);
            }
        }
        println!();
    }
    result
}

#[derive(Clone, Debug)]
struct Gear {
    numbers: Vec<Number>,
    product: u32,
    #[allow(dead_code)]
    position: (usize, usize),
    possible_index: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq)]
struct Number {
    value: u32,
    range: RangeInclusive<usize>,
    y: usize,
}

pub struct Day3;

impl Solution for Day3 {
    fn solve_part1(input: &str) -> anyhow::Result<String> {
        Ok(part1(input).to_string())
    }

    fn solve_part2(input: &str) -> anyhow::Result<String> {
        Ok(part2(input).to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::read_sample_input;

    use super::*;

    #[test]
    fn test_part1() {
        let input = read_sample_input(3, 1);
        assert_eq!(part1(&input), 4361);
    }

    #[test]
    fn test_part2() {
        let input = read_sample_input(3, 2);
        assert_eq!(part2(&input), 467835);
    }

    #[test]
    fn test_part2_2() {
        let input = "23.4
..*.";
        assert_eq!(part2(input), 0);
    }

    #[test]
    fn test_part2_3() {
        let input = ".......5......
..7*..*.......
...*13*.......
.......15.....";
        assert_eq!(part2(input), 442);
    }
}
