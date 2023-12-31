use crate::visualize_println;

use self::{
    direction::Direction,
    node::{encode_string, Node},
};

use super::Solution;

mod direction;
mod node;

fn part1(input: &str) -> u64 {
    let (directions, _, nodes) = parse_input(input);

    let start = "AAA";
    let goal = "ZZZ";
    let start = encode_string(start);
    let goal = encode_string(goal);

    // find_path_steps(start, goal, &directions, &nodes, &nodes_refs)
    step_all(start, goal, &directions, &nodes)
}

fn part2(input: &str) -> u64 {
    let (directions, positions, nodes) = parse_input(input);
    let start = "A";
    let goal = "Z";
    let start = encode_string(start);
    let goal = encode_string(goal);

    let next_nodes = nodes
        .iter()
        .filter(|n| n.match_name(start))
        .map(|n| &n.node_ref)
        .cloned()
        .collect::<Vec<_>>();

    // Calculate the LCM of all the steps to reach the goals
    // This was a bit of a pain to figure out and it was a bit
    // luck, but it works.
    next_nodes
        .iter()
        .map(|n| step_all(positions[n.index], goal, &directions, &nodes))
        .fold(1, lcm)
}

fn parse_input(input: &str) -> (Vec<Direction>, Vec<u32>, Vec<Node>) {
    let _start_time = std::time::Instant::now();
    let mut lines = input.lines().filter(|line| !line.is_empty());
    let directions = Direction::from_line(lines.next().unwrap());
    let mut nodes = lines.map(Node::from_line).collect::<Vec<_>>();
    let positions = nodes.iter().map(|n| n.name).collect::<Vec<_>>();

    for (index, node) in nodes.iter_mut().enumerate() {
        node.node_ref.index = index;
        node.node_ref.left = positions
            .iter()
            .position(|name| name == &node.left_string)
            .unwrap();
        node.node_ref.right = positions
            .iter()
            .position(|name| name == &node.right_string)
            .unwrap();
    }
    visualize_println!(
        "Time parse and build linked list: {:?}",
        _start_time.elapsed()
    );
    (directions, positions, nodes)
}

/// Finds the shortest path to the goal.
///
/// I left the code for the brute force solution here,
/// but I never pass less then 3 letters code for the node.
/// So this works like following a single path.
///
/// # Arguments
///
/// * `start` - The starting node name. Uses end_with to find the node.
/// * `goal` - The goal node. Uses end_with to find the node.
/// * `directions` - The directions to follow.
/// * `nodes` - The nodes.
/// * `nodes_refs` - The nodes references, basically a linked list of indices.
///
/// # Returns
///
/// The number of steps to reach the goal.
fn step_all(start: u32, goal: u32, directions: &[Direction], nodes: &[Node]) -> u64 {
    let goals_index = nodes
        .iter()
        .filter(|n| n.match_name(goal))
        .map(|n| n.node_ref.index)
        .collect::<Vec<_>>();

    let mut next_nodes = nodes
        .iter()
        .filter(|n| n.match_name(start))
        .map(|n| &n.node_ref)
        .collect::<Vec<_>>();

    visualize_println!(
        "Start: {:?}",
        next_nodes
            .iter()
            .map(|n| nodes[n.index].name())
            .collect::<Vec<_>>()
    );
    visualize_println!(
        "Possible Goals: {:?}",
        nodes
            .iter()
            .filter(|n| n.match_name(goal))
            .map(|n| n.name())
            .collect::<Vec<_>>()
    );

    let mut direction_index = 0;
    let mut steps = 1;
    loop {
        if direction_index >= directions.len() {
            direction_index = 0;
        }
        let direction = &directions[direction_index];

        let _current_index = next_nodes.iter().map(|n| n.index).collect::<Vec<_>>();

        next_nodes.iter_mut().for_each(|n| {
            *n = match direction {
                Direction::Left => &nodes[n.left].node_ref,
                Direction::Right => &nodes[n.right].node_ref,
            };
        });

        if cfg!(feature = "visualize") {
            let starts = _current_index
                .iter()
                .map(|i| nodes[*i].name())
                .collect::<Vec<_>>();
            let ends = next_nodes
                .iter()
                .map(|n| {
                    if nodes[n.index].match_name(goal) {
                        console::style(nodes[n.index].name()).green()
                    } else {
                        console::style(nodes[n.index].name()).dim()
                    }
                })
                .collect::<Vec<_>>();
            let pairs = starts
                .iter()
                .zip(ends.iter())
                .map(|(a, b)| format!("{a}->{b}"))
                .collect::<Vec<_>>()
                .join(" ");
            let count_goals = next_nodes
                .iter()
                .filter(|n| goals_index.contains(&n.index))
                .count();
            if next_nodes
                .iter()
                .filter(|n| goals_index.contains(&n.index))
                .count()
                >= 3
            {
                println!(
                    "Step:{:09} {:02} | Direction:{} | Nodes:{}",
                    steps,
                    count_goals,
                    console::style(direction).blue(),
                    pairs
                );
            }
        }

        if next_nodes.iter().all(|n| goals_index.contains(&n.index)) {
            break;
        }

        steps += 1;
        direction_index += 1;
    }
    visualize_println!("Steps: {}", steps);
    steps
}

/// Calculates the least common multiple (LCM) of two numbers.
///
/// # Arguments
///
/// * `a` - The first number.
/// * `b` - The second number.
///
/// # Returns
///
/// The least common multiple of `a` and `b`.
#[inline(always)]
fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

/// Calculates the greatest common divisor (GCD) of two numbers.
///
/// # Arguments
///
/// * `a` - The first number.
/// * `b` - The second number.
///
/// # Returns
///
/// The GCD of `a` and `b`.
#[inline(always)]
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub struct Day8;

impl Solution for Day8 {
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
        assert_eq!(
            part1(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            ),
            6
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            ),
            6
        );
    }
}
