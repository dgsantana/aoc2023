use anyhow::Result;

use super::Solution;

/// Advent of Code 2023 Day 1
/// --- Day 1: Trebuchet?! ---
/// Something is wrong with global snow production, and you've been selected to take a look.
/// The Elves have even given you a map; on it, they've used stars to mark the top fifty
/// locations that are likely to be having problems.
///
/// You've been doing this long enough to know that to restore snow operations,
/// you need to check all fifty stars by December 25th.
///
/// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar;
/// the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
///
/// You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you
///  ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say
/// the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").
///
/// As they're making the final adjustments, they discover that their calibration document (your puzzle input)
/// has been amended by a very young Elf who was apparently just excited to show off her art skills.
/// Consequently, the Elves are having trouble reading the values on the document.
///
/// The newly-improved calibration document consists of lines of text; each line originally contained a
/// specific calibration value that the Elves now need to recover. On each line, the calibration value
/// can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.
///
/// For example:
/// 1abc2
/// pqr3stu8vwx
/// a1b2c3d4e5f
/// treb7uchet
///
/// In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.
/// Consider your entire calibration document. What is the sum of all of the calibration values?
fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter(|c| c.is_numeric());
            let first = digits.next().and_then(|v| v.to_digit(10)).unwrap();
            let last = digits.last().and_then(|v| v.to_digit(10)).unwrap_or(first);
            first * 10 + last
        })
        .sum()
}

/// --- Part Two ---
/// Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters:
/// one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".
///
/// Equipped with this new information, you now need to find the real first and last digit on each line. For example:
///
/// two1nine
/// eightwothree
/// abcone2threexyz
/// xtwone3four
/// 4nineeightseven2
/// zoneight234
/// 7pqrstsixteen
/// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.
///
/// What is the sum of all of the calibration values?
fn part2(input: &str) -> u32 {
    const DIGITS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    fn find_pattern(mut range: impl Iterator<Item = usize>, line: &str) -> Option<u32> {
        range.find_map(|i| {
            let window = &line[i..];
            DIGITS
                .iter()
                .enumerate()
                .find(|(_, &word)| window.starts_with(word))
                .map(|(index, _)| index as u32 + 1)
                .or_else(|| window.chars().next().and_then(|v| v.to_digit(10)))
        })
    }

    fn parse_line(line: &str) -> Option<u32> {
        let range = 0..line.len();
        let first = find_pattern(range.clone(), line)?;
        let last = find_pattern(range.rev(), line)?;

        let value = 10 * first + last;
        if cfg!(feature = "visualize") {
            println!("{value} => {line}");
        }
        Some(value)
    }

    input.lines().filter_map(parse_line).sum()
}

pub struct Day1;

impl Solution for Day1 {
    fn solve_part1(input: &str) -> Result<String> {
        Ok(part1(input).to_string())
    }

    fn solve_part2(input: &str) -> Result<String> {
        Ok(part2(input).to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::read_sample_input;

    use super::*;

    #[test]
    fn test_part1() {
        let input = read_sample_input(1, 1);
        assert_eq!(part1(&input), 142);
    }

    #[test]
    fn test_part2() {
        let input = read_sample_input(1, 2);
        assert_eq!(part2(&input), 281);
    }
}
