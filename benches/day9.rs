use aoc2023::Solution;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const INPUT: &str = include_str!("../inputs/day9.txt");

fn part1(c: &mut Criterion) {
    c.bench_function("day9 part 1", |b| {
        b.iter(|| aoc2023::Day9::solve_part1(black_box(INPUT)))
    });
}

fn part2(c: &mut Criterion) {
    c.bench_function("day9 part 2", |b| {
        b.iter(|| aoc2023::Day9::solve_part2(black_box(INPUT)))
    });
}

criterion_group!(day9_benches, part1, part2);
criterion_main!(day9_benches);
