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
    use super::*;

    const TEST_DATA: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_DATA), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_DATA), 2286);
    }
}
