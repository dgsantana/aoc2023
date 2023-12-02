mod solutions;

use solutions::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: cargo run -- <day> <part>");
        std::process::exit(1);
    }

    let day: u32 = args[1].parse().expect("Invalid day argument");
    let part: u32 = args[2].parse().expect("Invalid part argument");

    if !(1..=25).contains(&day) {
        eprintln!("Invalid day argument: {}", day);
        std::process::exit(1);
    }

    if !(1..=2).contains(&part) {
        eprintln!("Invalid part argument: {}", part);
        std::process::exit(1);
    }

    let input = read_data(day);

    if day == 1 {
        let result = if part == 1 {
            Day1::solve_part1(&input)
        } else {
            Day1::solve_part2(&input)
        };
        println!("{:?}", result);
    } else if day == 2 {
        let result = if part == 1 {
            Day2::solve_part1(&input)
        } else {
            Day2::solve_part2(&input)
        };
        println!("{:?}", result);
    }
}
