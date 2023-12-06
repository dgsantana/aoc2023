use super::Solution;

// distance = holding_time * total_time - holding_time^2
// -holding_time^2 = holding_time * total_time - distance
// holding_time = (total_time - sqrt(total_time^2 - 4 * distance)) / 2
// holding_time = (total_time + sqrt(total_time^2 - 4 * distance)) / 2

fn part1(input: &str) -> u64 {
    let Some((times, distances)) = input.split_once('\n') else {
        return 0;
    };
    let times = times[6..]
        .split_ascii_whitespace()
        .filter_map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>();
    let distances = distances[10..]
        .split_ascii_whitespace()
        .filter_map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let total_permutations: u64 = times.iter().zip(distances.iter()).map(|(time, distance)| {
        let (min_holding_time, max_holding_time, permutations) = if cfg!(feature = "brute_force") {
            calculate_race_brute_force(*time, *distance)
        } else {
            calculate_race_optimize(*time, *distance)
        };
        if cfg!(feature = "visualize") {
            println!(
                "Race {time}ms {distance}mm -> [{}..{}] ({})",
                min_holding_time,
                max_holding_time,
                permutations
            );
        }
        permutations
    }).product();

    total_permutations
}

fn part2(input: &str) -> u64 {
    let Some((times, distances)) = input.split_once('\n') else {
        return 0;
    };
    let time = times[6..]
        .replace(' ', "")
        .trim()
        .parse::<u64>()
        .expect("Failed to parse time");
    let distance = distances[10..]
        .replace(' ', "")
        .trim()
        .parse::<u64>()
        .expect("Failed to parse distance");

    if cfg!(feature = "visualize") {
        print!("Permutation Race {time}ms {distance}mm: ");
    }

    let (min_holding_time, max_holding_time, permutations) = if cfg!(feature = "brute_force") {
        calculate_race_brute_force(time, distance)
    } else {
        calculate_race_optimize(time, distance)
    };

    if cfg!(feature = "visualize") {
        println!(
            "Race {time}ms {distance}mm -> [{}..{}] ({})",
            min_holding_time, max_holding_time, permutations
        );
    }
    permutations
}

/// Simple brute force solution
///
fn calculate_race_brute_force(t: u64, d: u64) -> (u64, u64, u64) {
    let mut permutations = 0;
    let mut min_holding_time = 0;
    let mut max_holding_time = 0;
    let mut holding_time = 1;

    loop {
        if holding_time > t {
            break;
        }
        let time_left = t - holding_time;
        let race_result = holding_time * time_left;
        if race_result > d {
            if min_holding_time == 0 {
                min_holding_time = holding_time;
            }
            max_holding_time = holding_time;
            permutations += 1;
        }
        holding_time += 1;
    }
    (min_holding_time, max_holding_time, permutations)
}

/// This solves the quadratic equation for our race problem.
///
/// ax^2 + bx + c = 0
///
/// The equation is:
///
/// distance = holding_time * total_time - holding_time^2
///
/// We can rearrange this to:
///
/// -holding_time^2 = holding_time * total_time - distance
///
/// holding_time^2 = distance - holding_time * total_time
///
/// holding_time^2 + holding_time * total_time - distance = 0
///
/// Where:
///
/// a = 1, b = total_time, c = -distance
///
/// We can then solve for holding_time using the quadratic equation:
///
///
/// holding_time = (total_time - sqrt(total_time^2 - 4 * distance)) / 2
///
/// holding_time = (total_time + sqrt(total_time^2 - 4 * distance)) / 2
///
///
/// Assuming that the discriminant is positive, we can calculate the two solutions
/// and return the one that is greater than the holding time
/// This is a lot faster than the brute force method
fn calculate_race_optimize(t: u64, d: u64) -> (u64, u64, u64) {
    let discriminant = (t * t) as f64 - 4.0 * d as f64;
    if discriminant < 0.0 {
        eprintln!(
            "Discriminant is negative: {}. No solution available.",
            discriminant
        );
        return (0, 0, 0);
    }

    let sqrt_discriminant = discriminant.sqrt();
    let t_f = t as f64;
    let a1 = (t_f + sqrt_discriminant) / 2.0;
    let a2 = (t_f - sqrt_discriminant) / 2.0;
    let aa1 = (t_f + sqrt_discriminant) as u64 / 2;
    let aa2 = (t_f - sqrt_discriminant) as u64 / 2;
    // let min = 
    let max_holding_time = a1.floor() as u64;
    let min_holding_time = a2.floor() as u64;
    let result = if discriminant == 0.0 {
        // Only one solution
        eprintln!("Discriminant is zero. Only one solution available.");
        (a1 as u64, 0, 0)
    } else {
        (
            min_holding_time,
            max_holding_time,
            (min_holding_time..max_holding_time).count() as u64,
        )
    };
    if cfg!(feature = "visualize") {
        println!("Discriminant: {}", discriminant);
        println!("Sqrt Discriminant: {}", sqrt_discriminant);
        println!("Solution 1: {} ({})", a1, aa1);
        println!("Solution 2: {} ({})", a2, aa2);
        println!("Permutations: {}", result.2);
    }
    result
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
