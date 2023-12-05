use console::Style;

use super::Solution;

fn part1(input: &str) -> u32 {
    let start = std::time::Instant::now();
    let mut cards = parse_cards(input);
    for card in cards.iter_mut() {
        if card.matches > 0 {
            card.points = 1_u32 << (card.matches - 1);
        }
    }
    let result = cards.iter().map(|c| c.points).sum();
    let duration = start.elapsed();
    println!("Part 1 took {:?}", duration);

    // Visualize the cards
    if cfg!(feature = "visualize") {
        let gold = Style::new().bright().yellow().bold();
        let red = Style::new().red().bold();
        let green = Style::new().green().bold();

        for card in cards.iter() {
            print!("Card {:03}: ", card.number);
            for number in &card.winning_numbers {
                print!("{:02} ", gold.apply_to(number));
            }
            print!("| ");
            for number in &card.numbers {
                if card.winning_numbers.contains(number) {
                    print!("{:02} ", green.apply_to(number));
                } else {
                    print!("{:02} ", red.apply_to(number));
                }
            }
            println!("=> {}", card.points);
        }
    }
    result
}

fn part2(input: &str) -> u32 {
    let start = std::time::Instant::now();
    let cards = parse_cards(input);
    let number_of_cards = cards.len();
    let mut card_copies = vec![1_u32; number_of_cards];
    for (i, card) in cards.iter().enumerate().filter(|(_, c)| c.matches > 0) {
        let index_start = i + 1;
        if index_start >= number_of_cards {
            eprintln!("No more cards to copy");
            break;
        }
        let index_end = (i + card.matches).min(number_of_cards - 1);
        let copies = card_copies[i];
        card_copies[index_start..=index_end]
            .iter_mut()
            .for_each(|c| *c += copies);
    }
    let result = card_copies.iter().sum();
    let duration = start.elapsed();
    println!("Part 2 took {:?}", duration);

    // Visualize the cards
    if cfg!(feature = "visualize") {
        let gold = Style::new().bright().yellow().bold();
        let red = Style::new().red().bold();
        let green = Style::new().green().bold();

        for (i, card) in cards.iter().enumerate() {
            print!("Card {:03}: ", card.number);
            for number in &card.winning_numbers {
                print!("{:02} ", gold.apply_to(number));
            }
            print!("| ");
            for number in &card.numbers {
                if card.winning_numbers.contains(number) {
                    print!("{:02} ", green.apply_to(number));
                } else {
                    print!("{:02} ", red.apply_to(number));
                }
            }
            println!("=> Copies {}", card_copies[i]);
        }
    }
    result
}

fn parse_cards(input: &str) -> Vec<Card> {
    input
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            let number = (i + 1) as u32;
            let (winning_part, numbers_part) = line.split_once(" | ")?;

            let winning_numbers = winning_part
                .split_ascii_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect::<Vec<u32>>();

            let numbers = numbers_part
                .split_ascii_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect::<Vec<u32>>();

            let matches = numbers
                .iter()
                .filter(|n| winning_numbers.contains(n))
                .count();

            Some(Card {
                number,
                winning_numbers,
                numbers,
                matches,
                points: 0,
            })
        })
        .collect()
}

#[derive(Clone, Debug)]
struct Card {
    number: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
    matches: usize,
    points: u32,
}

pub struct Day4;

impl Solution for Day4 {
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
        let input = &read_sample_input(4, 2);
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_part2() {
        let input = &read_sample_input(4, 2);
        assert_eq!(part2(input), 30);
    }
}
