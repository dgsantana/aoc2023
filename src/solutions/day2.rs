use super::Solution;

/// --- Day 2: Cube Conundrum ---
/// You're launched high into the atmosphere! The apex of your trajectory just
/// barely reaches the surface of a large island floating in the sky. You gently
/// land in a fluffy pile of leaves. It's quite cold, but you don't see much
/// snow. An Elf runs over to greet you.
///
/// The Elf explains that you've arrived at Snow Island and apologizes for the
/// lack of snow. He'll be happy to explain the situation, but it's a bit of a
/// walk, so you have some time. They don't get many visitors up here; would you
/// like to play a game in the meantime?
///
/// As you walk, the Elf shows you a small bag and some cubes which are either
/// red, green, or blue. Each time you play this game, he will hide a secret
/// number of cubes of each color in the bag, and your goal is to figure out
/// information about the number of cubes.
///
/// To get information, once a bag has been loaded with cubes, the Elf will reach
/// into the bag, grab a handful of random cubes, show them to you, and then put
/// them back in the bag. He'll do this a few times per game.
///
/// You play several games and record the information from each game (your puzzle
/// input). Each game is listed with its ID number (like the 11 in Game 11: ...)
/// followed by a semicolon-separated list of subsets of cubes that were revealed
/// from the bag (like 3 red, 5 green, 4 blue).
///
/// For example, the record of a few games might look like this:
///
/// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
/// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
/// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
/// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
/// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
///
/// In game 1, three sets of cubes are revealed from the bag (and then put back
/// again). The first set is 3 blue cubes and 4 red cubes; the second set is 1
/// red cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green
/// cubes.
///
/// The Elf would first like to know which games would have been possible if the
/// bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
///
/// In the example above, games 1, 2, and 5 would have been possible if the bag
/// had been loaded with that configuration. However, game 3 would have been
/// impossible because at one point the Elf showed you 20 red cubes at once;
/// similarly, game 4 would also have been impossible because the Elf showed you
/// 15 blue cubes at once. If you add up the IDs of the games that would have
/// been possible, you get 8.
///
/// Determine which games would have been possible if the bag had been loaded
/// with only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum
/// of the IDs of those games?

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
        println!("Game {}", game);
        let line = line.trim();
        let game_sets = line.split(';');

        if game_sets.map(validate_game_set).all(|v| v) {
            possible_games.push(game);
        }
    }
    possible_games.iter().sum()
}

/// --- Part Two ---
/// The Elf says they've stopped producing snow because they aren't getting any
/// water! He isn't sure why the water stopped; however, he can show you how to
/// get to the water source to check it out for yourself. It's just up ahead!
///
/// As you continue your walk, the Elf poses a second question: in each game you
/// played, what is the fewest number of cubes of each color that could have been
/// in the bag to make the game possible?
///
/// Again consider the example games from earlier:
///
/// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
/// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
/// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
/// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
/// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
/// In game 1, the game could have been played with as few as 4 red, 2 green,
/// and 6 blue cubes. If any color had even one fewer cube, the game would have
/// been impossible.
/// Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue
/// cubes.
/// Game 3 must have been played with at least 20 red, 13 green, and 6 blue
/// cubes.
/// Game 4 required at least 14 red, 3 green, and 15 blue cubes.
/// Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.
/// The power of a set of cubes is equal to the numbers of red, green, and blue
/// cubes multiplied together. The power of the minimum set of cubes in game 1
/// is 48. In games 2-5 it was 12, 1560, 630, and 36, respectively. Adding up
/// these five powers produces the sum 2286.
///
/// For each game, find the minimum set of cubes that must have been present.
/// What is the sum of the power of these sets?
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
        let min_power_of_game_set = min_power_of_game_set(line);
        println!("Game {} -> {}", game, min_power_of_game_set);
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
