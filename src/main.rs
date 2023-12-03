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
    } else if day == 3 {
        let result = if part == 1 {
            Day3::solve_part1(&input)
        } else {
            Day3::solve_part2(&input)
        };
        println!("{:?}", result);
    } else if day == 4 {
        let result = if part == 1 {
            Day4::solve_part1(&input)
        } else {
            Day4::solve_part2(&input)
        };
        println!("{:?}", result);
    } else if day == 5 {
        let result = if part == 1 {
            Day5::solve_part1(&input)
        } else {
            Day5::solve_part2(&input)
        };
        println!("{:?}", result);
    } else if day == 6 {
        let result = if part == 1 {
            Day6::solve_part1(&input)
        } else {
            Day6::solve_part2(&input)
        };
        println!("{:?}", result);
    } else if day == 7 {
        let result = if part == 1 {
            Day7::solve_part1(&input)
        } else {
            Day7::solve_part2(&input)
        };
        println!("{:?}", result);
    } else if day == 8 {
        let result = if part == 1 {
            Day8::solve_part1(&input)
        } else {
            Day8::solve_part2(&input)
        };
        println!("{:?}", result);
    } else if day == 9 {
        let result = if part == 1 {
            Day9::solve_part1(&input)
        } else {
            Day9::solve_part2(&input)
        };
        println!("{:?}", result);
    } else if day == 10 {
        let result = if part == 1 {
            Day10::solve_part1(&input)
        } else {
            Day10::solve_part2(&input)
        };
        println!("{:?}", result);
    } else if day == 11 {
        let result = if part == 1 {
            Day11::solve_part1(&input)
        } else {
            Day11::solve_part2(&input)
        };
        println!("{:?}", result);
    } else if day == 12 {
        let result = if part == 1 {
            Day12::solve_part1(&input)
        } else {
            Day12::solve_part2(&input)
        };
        println!("{:?}", result);
    } else if day == 13 {
        let result = if part == 1 {
            Day13::solve_part1(&input)
        } else {
            Day13::solve_part2(&input)
        };
        println!("{:?}", result);
    } else if day == 14 {
        let result = if part == 1 {
            Day14::solve_part1(&input)
        } else {
            Day14::solve_part2(&input)
        };
        println!("{:?}", result);
    } else if day == 15 {
        let result = if part == 1 {
            Day15::solve_part1(&input)
        } else {
            Day15::solve_part2(&input)
        };
        println!("{:?}", result);
    } else if day == 16 {
        let result = if part == 1 {
            Day16::solve_part1(&input)
        } else {
            Day16::solve_part2(&input)
        };
        println!("{:?}", result);
    } else if day == 17 {
        let result = if part == 1 {
            Day17::solve_part1(&input)
        } else {
            Day17::solve_part2(&input)
        };
        println!("{:?}", result);
    } else if day == 18 {
        let result = if part == 1 {
            Day18::solve_part1(&input)
        } else {
            Day18::solve_part2(&input)
        };
        println!("{:?}", result);
    } else if day == 19 {
        let result = if part == 1 {
            Day19::solve_part1(&input)
        } else {
            Day19::solve_part2(&input)
        };
        println!("{:?}", result);
    } else if day == 20 {
        let result = if part == 1 {
            Day20::solve_part1(&input)
        } else {
            Day20::solve_part2(&input)
        };
        println!("{:?}", result);
    } else if day == 21 {
        let result = if part == 1 {
            Day21::solve_part1(&input)
        } else {
            Day21::solve_part2(&input)
        };
        println!("{:?}", result);
    } else if day == 22 {
        let result = if part == 1 {
            Day22::solve_part1(&input)
        } else {
            Day22::solve_part2(&input)
        };
        println!("{:?}", result);
    } else if day == 23 {
        let result = if part == 1 {
            Day23::solve_part1(&input)
        } else {
            Day23::solve_part2(&input)
        };
        println!("{:?}", result);
    } else if day == 24 {
        let result = if part == 1 {
            Day24::solve_part1(&input)
        } else {
            Day24::solve_part2(&input)
        };
        println!("{:?}", result);
    } else if day == 25 {
        let result = if part == 1 {
            Day25::solve_part1(&input)
        } else {
            Day25::solve_part2(&input)
        };
        println!("{:?}", result);
    }
}
