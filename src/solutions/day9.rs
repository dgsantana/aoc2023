use std::{
    ops::{Deref, DerefMut},
    vec,
};

use super::Solution;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Values(Vec<i64>);

impl Deref for Values {
    type Target = Vec<i64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Values {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct ValueHistory {
    steps: Vec<Values>,
    firsts: Vec<i64>,
    lasts: Vec<i64>,
    first: i64,
    last: i64,
}

impl ValueHistory {
    fn from_str(input: &str) -> Self {
        let base = Values(input.split(' ').filter_map(|s| s.parse().ok()).collect());
        let first = *base.first().unwrap();
        let last = *base.last().unwrap();
        Self {
            steps: vec![base],
            firsts: vec![first],
            lasts: vec![last],
            first: 0,
            last: 0,
        }
    }

    fn diff(&mut self) {
        fn inner_diff(values: &[i64]) -> Values {
            let mut result = Vec::new();
            for i in 0..values.len() - 1 {
                result.push(values[i + 1] - values[i]);
            }
            Values(result)
        }

        let mut base = self.steps.last().unwrap().clone();
        while base.iter().any(|v| *v != 0) {
            let diff = inner_diff(&base);
            #[cfg(feature = "visualize")]
            {
                self.steps.push(diff.clone());
            }
            self.lasts.push(*diff.last().unwrap());
            self.firsts.push(*diff.first().unwrap());
            base = diff;
        }

        #[cfg(feature = "visualize")]
        {
            for (i, s) in self.steps.iter().enumerate() {
                visualize_println!("step {}: {:?}", i, s);
            }
        }

        #[cfg(feature = "visualize")]
        if let Some(s) = self.steps.last_mut() {
            s.push(0);
            s.insert(0, 0);
        }
        let mut current_last = 0;
        let mut current_first = 0;
        for i in (1..self.lasts.len()).rev() {
            let previous_last = self.lasts[i - 1];
            let previous_first = self.firsts[i - 1];
            current_last += previous_last;
            current_first = previous_first - current_first;
            #[cfg(feature = "visualize")]
            {
                self.steps[i - 1].push(current_last);
                self.steps[i - 1].insert(0, current_first);
            }
        }
        self.last = current_last;
        self.first = current_first;

        #[cfg(feature = "visualize")]
        {
            for (i, s) in self.steps.iter().enumerate() {
                visualize_println!("step {}: {:?}", i, s);
            }
        }
    }
}

fn part1(input: &str) -> i64 {
    let mut history = input
        .lines()
        .map(ValueHistory::from_str)
        .collect::<Vec<_>>();
    history.iter_mut().for_each(|h| h.diff());
    let result = history.iter().map(|v| v.last).sum::<i64>();
    result
}

fn part2(input: &str) -> i64 {
    let mut history = input
        .lines()
        .map(ValueHistory::from_str)
        .collect::<Vec<_>>();
    history.iter_mut().for_each(|h| h.diff());
    let result = history.iter().map(|v| v.first).sum::<i64>();
    result
}

pub struct Day9;

impl Solution for Day9 {
    fn solve_part1(_input: &str) -> anyhow::Result<String> {
        Ok(part1(_input).to_string())
    }

    fn solve_part2(_input: &str) -> anyhow::Result<String> {
        Ok(part2(_input).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ),
            114
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ),
            2
        );
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(part2("10  13  16  21  30  45"), 5);
    }
}
