use std::fmt::Display;

use crossterm::style::Stylize;
use itertools::Itertools;

use super::Solution;

const CARDS: &str = "AKQJT98765432";
const CARDS_HEX: &str = "DCBA987654321";

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
    let mut hands = input.lines().map(hand).collect::<Vec<_>>();
    // Rank the hands
    hands.sort();
    // Sort the hands by the card converting the card to a number, assuming the cards is a hex number
    // hands.sort_by(|a, b| {
    //     u64::from_str_radix(&a.cards, 16)
    //         .unwrap()
    //         .cmp(&u64::from_str_radix(&b.cards, 16).unwrap())
    // });

    hands
        .iter_mut()
        .enumerate()
        .for_each(|(i, v)| v.rank += i as u64);
    for hand in hands.iter() {
        println!("{}", hand);
    }
    hands.iter().fold(0, |acc, hand| acc + hand.bid * hand.rank)
}

fn hand(input: &str) -> Hand {
    let (cards, bid) = input.split_once(' ').expect("Invalid input");
    let bid = bid.parse::<u64>().expect("Invalid bid");
    let values = cards
        .chars()
        .filter_map(|c| CARDS.find(c))
        .map(|i| 2u32.pow(14 - i as u32))
        .sorted_by(|a, b| b.cmp(&a))
        .collect::<Vec<_>>();
    let cards = values
        .iter()
        .map(|v| CARDS.chars().nth(14 - v.trailing_zeros() as usize).unwrap())
        .collect::<String>();
    let cards_in_hex = values
        .iter()
        .map(|v| {
            CARDS_HEX
                .chars()
                .nth(14 - v.trailing_zeros() as usize)
                .unwrap()
        })
        .collect::<String>();
    let unique_values = values.iter().unique().collect::<Vec<_>>();
    let (kind, remaing_value) = match unique_values.len() {
        1 => (HandType::FiveOfAKind(*unique_values[0]), 0),
        2 => {
            if values[0] == values[3] || values[1] == values[4] {
                // Handle AAAA2 or A2222
                let remaing_values = unique_values
                    .iter()
                    .filter(|&&v| values[1] != *v)
                    .copied()
                    .sum();
                (HandType::FourOfAKind(values[1]), remaing_values)
            } else {
                // Handle AAABB or AABBB
                (HandType::FullHouse(values[2], values[0]), 0)
            }
        }
        3 => {
            if values[0] == values[2] || values[1] == values[3] || values[2] == values[4] {
                // Handle AAA23 or AK222 or A999K
                let repeated_value = unique_values
                    .iter()
                    .find(|&&v| values.iter().filter(|&v2| v2 == v).count() == 3)
                    .copied()
                    .unwrap();
                let remaing_values = unique_values
                    .iter()
                    .filter(|&&v| v != repeated_value)
                    .copied()
                    .sum();
                (HandType::ThreeOfAKind(*repeated_value), remaing_values)
            } else {
                // Handle AA223 or AAK77 or A9922
                let repeated_values = unique_values
                    .iter()
                    .filter(|&&v| values.iter().filter(|&v2| v2 == v).count() == 2)
                    .copied()
                    .collect::<Vec<_>>();
                let remaing_values = unique_values
                    .iter()
                    .filter(|&&v| !repeated_values.contains(&v))
                    .copied()
                    .sum();
                (
                    HandType::TwoPair(*repeated_values[0], *repeated_values[1]),
                    remaing_values,
                )
            }
        }
        4 => {
            // Handle all cases for 1 pair, by finding the repeated value
            let repeated_value = unique_values
                .iter()
                .find(|&&v| values.iter().filter(|&v2| v2 == v).count() == 2)
                .copied()
                .unwrap();
            let remaing_values = unique_values
                .iter()
                .filter(|&&v| v != repeated_value)
                .copied()
                .sum();
            (HandType::OnePair(*repeated_value), remaing_values)
        }
        5 => {
            let remaing_values = unique_values.iter().copied().sum();
            (HandType::HighCard(values[0]), remaing_values)
        }
        _ => unreachable!(),
    };

    let card_hex = u64::from_str_radix(&cards_in_hex, 16).unwrap();

    Hand {
        rank: 1,
        kind,
        cards: cards.to_string(),
        value: card_hex,
        bid,
    }
}

fn part2(input: &str) -> u64 {
    0
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    rank: u64,
    kind: HandType,
    cards: String,
    value: u64,
    bid: u64,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.kind != other.kind {
            return self.kind.cmp(&other.kind);
        } else {
            return self.value.cmp(&other.value);
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04}:{}({:05X}) {:04} ({:05} : {:05}) -> {}",
            self.rank,
            &self.cards,
            console::style(self.value).dim(),
            console::style(self.bid).dim(),
            console::style(self.kind.value()).green(),
            console::style(self.value).red(),
            &self.kind,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HandType {
    FiveOfAKind(u32),
    FourOfAKind(u32),
    FullHouse(u32, u32),
    ThreeOfAKind(u32),
    TwoPair(u32, u32),
    OnePair(u32),
    HighCard(u32),
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_order = match self {
            HandType::FiveOfAKind(_) => 6,
            HandType::FourOfAKind(_) => 5,
            HandType::FullHouse(_, _) => 4,
            HandType::ThreeOfAKind(_) => 3,
            HandType::TwoPair(_, _) => 2,
            HandType::OnePair(_) => 1,
            HandType::HighCard(_) => 0,
        };
        let other_order = match other {
            HandType::FiveOfAKind(_) => 6,
            HandType::FourOfAKind(_) => 5,
            HandType::FullHouse(_, _) => 4,
            HandType::ThreeOfAKind(_) => 3,
            HandType::TwoPair(_, _) => 2,
            HandType::OnePair(_) => 1,
            HandType::HighCard(_) => 0,
        };
        if self_order != other_order {
            return self_order.cmp(&other_order);
        }
        match (self, other) {
            (HandType::FullHouse(_, _), HandType::FullHouse(_, _))
            | (HandType::TwoPair(_, _), HandType::TwoPair(_, _)) => {
                let sum1 = self.value();
                let sum2 = other.value();
                sum1.cmp(&sum2)
            }
            _ => self.value().cmp(&other.value()),
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for HandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn v2c(value: u32) -> char {
            CARDS
                .chars()
                .nth(14 - value.trailing_zeros() as usize)
                .unwrap()
        }
        match self {
            HandType::FiveOfAKind(v) => write!(f, "Five of a kind {}", v2c(*v)),
            HandType::FourOfAKind(v) => write!(f, "Four of a kind {}", v2c(*v)),
            HandType::FullHouse(v1, v2) => write!(f, "Full house {} and {}", v2c(*v1), v2c(*v2)),
            HandType::ThreeOfAKind(v) => write!(f, "Three of a kind {}", v2c(*v)),
            HandType::TwoPair(v1, v2) => write!(f, "Two pair {} and {}", v2c(*v1), v2c(*v2)),
            HandType::OnePair(v) => write!(f, "One pair {}", v2c(*v)),
            HandType::HighCard(v) => write!(f, "High card {}", v2c(*v)),
        }
    }
}

impl HandType {
    fn value(&self) -> u32 {
        match self {
            HandType::FiveOfAKind(v) => *v,
            HandType::FourOfAKind(v) => *v,
            HandType::FullHouse(v1, v2) => *v1 + *v2,
            HandType::ThreeOfAKind(v) => *v,
            HandType::TwoPair(v1, v2) => *v1 + *v2,
            HandType::OnePair(v) => *v,
            HandType::HighCard(v) => *v,
        }
    }
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
            0
        );
    }
}
