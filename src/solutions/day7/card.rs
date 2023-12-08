use std::fmt::Display;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    pub fn from_char(c: char) -> Self {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card: {}", c),
        }
    }
}

macro_rules! card_from {
    ($tt:ty) => {
        impl From<$tt> for Card {
            fn from(value: $tt) -> Self {
                match value {
                    1 => Card::Joker,
                    2 => Card::Two,
                    3 => Card::Three,
                    4 => Card::Four,
                    5 => Card::Five,
                    6 => Card::Six,
                    7 => Card::Seven,
                    8 => Card::Eight,
                    9 => Card::Nine,
                    10 => Card::Ten,
                    11 => Card::Jack,
                    12 => Card::Queen,
                    13 => Card::King,
                    14 => Card::Ace,
                    _ => unreachable!(),
                }
            }
        }
    };
}

card_from!(u8);
card_from!(u32);
card_from!(&u32);

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Card::Joker => console::style("J").red(),
            Card::Two => console::style("2").green(),
            Card::Three => console::style("3").green(),
            Card::Four => console::style("4").green(),
            Card::Five => console::style("5").green(),
            Card::Six => console::style("6").green(),
            Card::Seven => console::style("7").green(),
            Card::Eight => console::style("8").green(),
            Card::Nine => console::style("9").green(),
            Card::Ten => console::style("T").green(),
            Card::Jack => console::style("J").green(),
            Card::Queen => console::style("Q").green(),
            Card::King => console::style("K").green(),
            Card::Ace => console::style("A").green(),
        };
        write!(f, "{}", c)
    }
}
