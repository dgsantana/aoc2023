use aoc2023::Solution;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const INPUT: &str = include_str!("../inputs/day6.txt");

fn part1(c: &mut Criterion) {
    c.bench_function("day6 part 1", |b| {
        b.iter(|| aoc2023::Day6::solve_part1(black_box(INPUT)))
    });
}

fn part2(c: &mut Criterion) {
    c.bench_function("day6 part 2", |b| {
        b.iter(|| aoc2023::Day6::solve_part2(black_box(INPUT)))
    });
}

criterion_group!(day6_benches, part1, part2);
criterion_main!(day6_benches);
