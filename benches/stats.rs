use std::fs::read_to_string;

use iai_callgrind::{black_box, library_benchmark, library_benchmark_group, main};

use aoc2023::*;

fn gen_input(day: u8) -> String {
    let input = read_to_string(format!("../inputs/day{day}.txt")).unwrap();
    input.to_string()
}

macro_rules! bench_day {
    ($benchname:ident, $day:literal, $daystruct:tt, $part:literal) => {
        #[library_benchmark]
        #[bench::short(gen_input($day))]
        fn $benchname(input: String) {
            if $part == 1 {
                let res = $daystruct::solve_part1(&input);
                let _ = black_box(res);
                return;
            } else if $part == 2 {
                let res = $daystruct::solve_part2(&input);
                let _ = black_box(res);
                return;
            }
        }
    };
}
bench_day!(y2023d01_1, 1, Day6, 1);
bench_day!(y2023d01_2, 1, Day6, 2);
bench_day!(y2023d02_1, 2, Day6, 1);
bench_day!(y2023d02_2, 2, Day6, 2);
bench_day!(y2023d03_1, 3, Day6, 1);
bench_day!(y2023d03_2, 3, Day6, 2);
bench_day!(y2023d04_1, 4, Day6, 1);
bench_day!(y2023d04_2, 4, Day6, 2);
bench_day!(y2023d05_1, 5, Day6, 1);
bench_day!(y2023d05_2, 5, Day6, 2);
bench_day!(y2023d06_1, 6, Day6, 1);
bench_day!(y2023d06_2, 6, Day6, 2);

library_benchmark_group!(
    name = bench_year2023;
    benchmarks = y2023d01_1, y2023d01_2, y2023d02_1, y2023d02_2, y2023d03_1, y2023d03_2, y2023d04_1, y2023d04_2, y2023d05_1, y2023d05_2, y2023d06_1, y2023d06_2
);

main!(library_benchmark_groups = bench_year2023);