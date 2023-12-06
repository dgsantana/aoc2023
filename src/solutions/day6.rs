use super::Solution;

// distance = holding_time * total_time - holding_time^2
// -holding_time^2 = holding_time * total_time - distance
// holding_time = (total_time - sqrt(total_time^2 - 4 * distance)) / 2
// holding_time = (total_time + sqrt(total_time^2 - 4 * distance)) / 2

fn part1(input: &str) -> u32 {
    let Some((times, distances)) = input.split_once('\n') else {
        return 0;
    };
    let times = times[6..]
        .split_ascii_whitespace()
        .filter_map(|v| v.parse::<u32>().ok())
        .collect::<Vec<_>>();
    let distances = distances[10..]
        .split_ascii_whitespace()
        .filter_map(|v| v.parse::<u32>().ok())
        .collect::<Vec<_>>();

    let mut total_permutations = 1;
    for i in 0..times.len() {
        let time = times[i];
        let distance = distances[i];
        let mut permutations = Vec::new();
        let mut holding_time = 1;
        if cfg!(feature = "visualize") {
            print!("Permutation Race {time}ms {distance}mm: ");
        }
        loop {
            if holding_time > time {
                break;
            }
            let time_left = time - holding_time;
            let race_result = holding_time * time_left;
            if race_result > distance {
                if cfg!(feature = "visualize") {
                    print!("{holding_time}->{race_result} ");
                }
                permutations.push(holding_time);
            }
            holding_time += 1;
        }
        let min = permutations.iter().min().unwrap_or(&0);
        let max = permutations.iter().max().unwrap_or(&0);
        if cfg!(feature = "visualize") {
            println!();
            println!(
                "Race {time}ms {distance}mm -> [{}..{}] ({})",
                min,
                max,
                permutations.len()
            );
        }
        total_permutations *= permutations.len() as u32;
    }
    total_permutations
}

fn part2(input: &str) -> usize {
    let Some((times, distances)) = input.split_once('\n') else {
        return 0;
    };
    let time = times[6..]
        .replace(' ', "")
        .trim()
        .parse::<usize>()
        .expect("Failed to parse time");
    let distance = distances[10..]
        .replace(' ', "")
        .trim()
        .parse::<usize>()
        .expect("Failed to parse distance");

    if cfg!(feature = "visualize") {
        print!("Permutation Race {time}ms {distance}mm: ");
    }

    loop {
        if holding_time > time {
            break;
        }
        let time_left = time - holding_time;
        let race_result = holding_time * time_left;
        if race_result > distance {
    if cfg!(feature = "visualize") {
        println!(
            "Race {time}ms {distance}mm -> [{}..{}] ({})",
            min_holding_time, max_holding_time, permutations
        );
    }
    permutations
}

pub struct Day6;

impl Solution for Day6 {
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
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part1(input), 288);
    }

    #[test]
    fn test_part2() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part2(input), 71503);
    }
}
