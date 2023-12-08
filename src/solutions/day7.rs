use self::hand::Hand;

use super::Solution;

mod card;
mod hand;
mod hand_type;

/// Parse the input into a vector of cards and the bid.
///
/// Then we have 7 types of hands e.g.:
/// AAAAA - > Five of a kind
/// AA8AA - > Four of a kind
/// 23332 - > Full house
/// TTT98 - > Three of a kind
/// 23432 - > Two pair
/// A23A4 - > One pair
/// 23456 - > High card
fn part1(input: &str) -> u64 {
    let mut hands = input
        .lines()
        .map(|line| Hand::from_str(line, false))
        .collect::<Vec<_>>();
    // Rank the hands
    hands.sort();

    hands
        .iter_mut()
        .enumerate()
        .for_each(|(i, v)| v.rank = 1 + i as u64);
    if cfg!(feature = "visualize") {
        for hand in hands.iter() {
            println!("{}", hand);
        }
    }
    hands.iter().map(|h| h.rank * h.bid).sum()
}

fn part2(input: &str) -> u64 {
    let mut hands = input
        .lines()
        .map(|line| Hand::from_str(line, true))
        .collect::<Vec<_>>(); // Rank the hands
    hands.sort();

    hands
        .iter_mut()
        .enumerate()
        .for_each(|(i, v)| v.rank = 1 + i as u64);
    if cfg!(feature = "visualize") {
        for hand in hands.iter() {
            println!("{}", hand);
        }
    }
    hands.iter().map(|h| h.rank * h.bid).sum()
}

pub struct Day7;

impl Solution for Day7 {
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
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            ),
            6440
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            ),
            5905
        );
    }
}
