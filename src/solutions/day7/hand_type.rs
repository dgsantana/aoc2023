use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::{visualize_print, visualize_println};

use super::card::Card;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandType {
    FiveOfAKind(Card),
    FourOfAKind(Card),
    FullHouse(Card, Card),
    ThreeOfAKind(Card),
    TwoPair(Card, Card),
    OnePair(Card),
    HighCard(Card),
}

impl HandType {
    pub fn value(&self) -> u32 {
        match self {
            HandType::FiveOfAKind(_) => 1 << 6,
            HandType::FourOfAKind(_) => 1 << 5,
            HandType::FullHouse(_, _) => 1 << 4,
            HandType::ThreeOfAKind(_) => 1 << 3,
            HandType::TwoPair(_, _) => 1 << 2,
            HandType::OnePair(_) => 1 << 1,
            HandType::HighCard(_) => 1 << 0,
        }
    }

    pub fn from_cards(cards: &[Card; 5]) -> Self {
        let mut cards = *cards;
        // Sort the cards ascending
        cards.sort();
        let unique_cards = cards.iter().cloned().collect::<HashSet<_>>();
        let unique_cards_len = unique_cards.len();
        let jokers = cards.iter().filter(|c| **c == Card::Joker).count();

        let mut counts = HashMap::new();
        for card in cards.iter() {
            *counts.entry(*card).or_insert(0_u8) += 1;
        }

        visualize_print!(
            "{} -> {:?} ({}) {:?}",
            cards
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(""),
            unique_cards,
            unique_cards_len,
            counts,
        );

        if unique_cards_len <= 1 {
            visualize_println!(" Five of a Kind");
            return HandType::FiveOfAKind(cards[0]);
        }

        if jokers > 0 {
            return Self::handle_jokers(jokers, &cards, unique_cards, counts);
        }

        // Four of a Kind
        if unique_cards_len == 2 {
            let four_of_a_kind = counts.iter().find(|(_, v)| **v == 4).map(|v| *v.0);
            let three_of_a_kind = counts.iter().find(|(_, v)| **v == 3).map(|v| *v.0);
            let two_of_a_kind = counts.iter().find(|(_, v)| **v == 2).map(|v| *v.0);
            if let Some(four_of_a_kind) = four_of_a_kind {
                visualize_println!(" Four of a Kind {}", four_of_a_kind);
                return HandType::FourOfAKind(four_of_a_kind);
            } else if let (Some(three_of_a_kind), Some(two_of_a_kind)) =
                (three_of_a_kind, two_of_a_kind)
            {
                visualize_println!(
                    " Full House with {}(3) and {}(2)",
                    three_of_a_kind,
                    two_of_a_kind
                );
                return HandType::FullHouse(three_of_a_kind, two_of_a_kind);
            } else {
                panic!("Invalid input: {:?}", cards);
            }
        }

        // Three of a Kind or Two Pairs
        if unique_cards_len == 3 {
            let three_of_a_kind = counts.iter().find(|(_, v)| **v == 3).map(|v| *v.0);
            let two_of_a_kind = counts
                .iter()
                .filter(|(_, v)| **v == 2)
                .map(|v| *v.0)
                .collect::<Vec<_>>();
            // AAA21
            if let Some(three_of_a_kind) = three_of_a_kind {
                visualize_println!(" Three of a Kind {}", three_of_a_kind);
                return HandType::ThreeOfAKind(three_of_a_kind);
            }
            // AA221
            if two_of_a_kind.len() == 2 {
                visualize_println!(
                    " Two Pairs with {} and {}",
                    two_of_a_kind[0],
                    two_of_a_kind[1]
                );
                return HandType::TwoPair(two_of_a_kind[0], two_of_a_kind[1]);
            } else {
                panic!("Invalid input: {:?}", cards);
            }
        }

        // A pair AA234
        if unique_cards_len == 4 {
            let two_of_a_kind = counts.iter().find(|(_, v)| **v == 2).map(|v| *v.0).unwrap();
            visualize_println!(" One pair {}", two_of_a_kind);
            return HandType::OnePair(two_of_a_kind);
        }

        // Five of a Kind where the fifth card is the highest card due to sort ascending
        visualize_println!(" High card {}", cards[4]);
        HandType::HighCard(cards[4])
    }

    /// This is a dumb way to handle jokers. I'm sure there's a better way.
    fn handle_jokers(
        jokers: usize,
        cards: &[Card; 5],
        mut unique_cards: HashSet<Card>,
        mut counts: HashMap<Card, u8>,
    ) -> Self {
        let no_jokers = &cards[jokers..];

        unique_cards.remove(&Card::Joker);
        counts.remove(&Card::Joker);

        let unique_cards_len = unique_cards.len();
        let last_card = no_jokers.last().cloned().unwrap();

        visualize_print!(" No jokers: {:?}", no_jokers);
        visualize_print!(" Counts: {:?}", counts);

        // AAAAJ or AAAJJ or AAJJJ or AJJJJ
        if unique_cards_len <= 1 {
            visualize_println!(" Five of a Kind");
            return HandType::FiveOfAKind(no_jokers[0]);
        }

        // AAAA8 or AAAJ8 or AAJJ8 or A8888
        if unique_cards_len <= 2
            && no_jokers
                .windows(4 - jokers)
                .any(|cards| cards.iter().dedup().count() == 1)
        {
            visualize_println!(" Four of a Kind");
            return HandType::FourOfAKind(last_card);
        }
        // AAA8J
        if jokers == 1 && unique_cards_len == 2 {
            visualize_println!(" Full House");
            return HandType::FullHouse(last_card, no_jokers[0]);
        }
        // 2344J
        if jokers == 1 && unique_cards_len == 3 {
            visualize_println!(" Three of a Kind");
            let higher_card = counts.iter().find(|(_, v)| **v == 2).map(|v| *v.0).unwrap();
            return HandType::ThreeOfAKind(higher_card);
        }
        // 234JJ
        if jokers == 2 && unique_cards_len == 3 {
            visualize_println!(" Three of a Kind");
            return HandType::ThreeOfAKind(last_card);
        }
        // AJJ28
        if jokers == 2 && unique_cards_len == 3 {
            visualize_println!(" Two Pair (2)");
            return HandType::TwoPair(no_jokers[1], last_card);
        }
        // ATJ29
        if jokers == 1 && unique_cards_len == 4 {
            visualize_println!(" One Pair {}", last_card);
            return HandType::OnePair(last_card);
        }
        // This should not happen
        unreachable!("Invalid input: {:?}", cards);
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for HandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandType::FiveOfAKind(card) => write!(f, "Five of a kind {}", card),
            HandType::FourOfAKind(card) => write!(f, "Four of a kind {}", card),
            HandType::FullHouse(card1, card2) => write!(f, "Full house {} and {}", card1, card2),
            HandType::ThreeOfAKind(card) => write!(f, "Three of a kind {}", card),
            HandType::TwoPair(card1, card2) => write!(f, "Two pair {} and {}", card1, card2),
            HandType::OnePair(card) => write!(f, "One pair {}", card),
            HandType::HighCard(card) => write!(f, "High card {}", card),
        }
    }
}
