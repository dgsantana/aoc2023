use std::fmt::Display;

use itertools::Itertools;

use super::card::Card;
use super::hand_type::HandType;

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    pub rank: u64,
    pub kind: HandType,
    pub cards: [Card; 5],
    pub bid: u64,
}

impl Hand {
    pub fn from_str(input: &str, replace_jokers: bool) -> Self {
        let Some((cards_str, bid)) = input.split_once(' ') else {
            panic!("Invalid input: {}", input);
        };
        let cards: [Card; 5] = cards_str
            .chars()
            .map(Card::from_char)
            .collect::<Vec<_>>()
            .try_into()
            .expect("Must be 5 cards");
        let bid = bid.parse::<u64>().unwrap();
        let cards = if replace_jokers {
            let mut cards = cards;
            for card in cards.iter_mut() {
                if *card == Card::Jack {
                    *card = Card::Joker;
                }
            }
            cards
        } else {
            cards
        };
        let kind = HandType::from_cards(&cards);
        Self {
            rank: 0,
            kind,
            cards,
            bid,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.kind.cmp(&other.kind) {
            std::cmp::Ordering::Equal => self.cards.cmp(&other.cards),
            other => other,
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
        let cards = &self
            .cards
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("");
        let sorted_cards = &self
            .cards
            .iter()
            .sorted()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("");
        write!(
            f,
            "{:04}: {} [{}] -> {:05} {}",
            self.rank,
            cards,
            sorted_cards,
            console::style(self.bid).dim(),
            &self.kind,
        )
    }
}
