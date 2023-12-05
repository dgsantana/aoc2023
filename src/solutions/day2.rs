use super::Solution;

const CUBES: [u32; 3] = [12, 13, 14];
const CUBES_NAMES: [&str; 3] = ["red", "green", "blue"];

fn part1(input: &str) -> u32 {
    let mut possible_games = Vec::new();

    fn validate_game_set(set: &str) -> bool {
        set.split(',')
            .filter_map(|s| s.trim().split_once(' ').map(|(n, c)| (n.trim(), c.trim())))
            .all(|(number, cube_name)| {
                if let Some(i) = CUBES_NAMES.iter().position(|&name| name == cube_name) {
                    if let Ok(number) = number.parse::<u32>() {
                        return number <= CUBES[i];
                    }
                }
                false
            })
    }

    for (id, game_line) in input.lines().enumerate() {
        let Some((game, line)) = game_line.split_once(':') else {
            continue;
        };
        let game = {
            if !game.starts_with("Game ") {
                continue;
            }
            let game = game[5..].trim();
            if let Ok(game) = game.parse::<u32>() {
                if game != id as u32 + 1 {
                    continue;
                }
                game
            } else {
                continue;
            }
        };
        let line = line.trim();
        let game_sets = line.split(';');

        if game_sets.map(validate_game_set).all(|v| v) {
            possible_games.push(game);
        }
    }
    let result = possible_games.iter().sum();
    result
}

fn part2(input: &str) -> u32 {
    let mut power_sum = 0;

    fn min_power_of_game_set(game: &str) -> u32 {
        let mut min_cubes = [1; 3];
        game.split(';').for_each(|set| {
            set.split(',')
                .filter_map(|s| s.trim().split_once(' ').map(|(n, c)| (n.trim(), c.trim())))
                .for_each(|(number, cube_name)| {
                    if let Some(i) = CUBES_NAMES.iter().position(|&name| name == cube_name) {
                        if let Ok(number) = number.parse::<u32>() {
                            if number > min_cubes[i] {
                                min_cubes[i] = number;
                            }
                        }
                    }
                })
        });
        min_cubes.iter().product()
    }

    for game_line in input.lines() {
        let Some((_game, line)) = game_line.split_once(':') else {
            continue;
        };
        let line = line.trim();
        let min_power_of_game_set = min_power_of_game_set(line);
        power_sum += min_power_of_game_set;
    }
    power_sum
}

pub struct Day2;

impl Solution for Day2 {
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
        let input = read_sample_input(2, 1);
        assert_eq!(part1(&input), 8);
    }

    #[test]
    fn test_part2() {
        let input = read_sample_input(2, 1);
        assert_eq!(part2(&input), 2286);
    }
}
