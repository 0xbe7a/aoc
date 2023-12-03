use criterion::{criterion_group, criterion_main, Criterion};
use aoc::solutions::*;
use aoc::read_file;

macro_rules! bench_day {
    ($c: expr, $day:path, $day_num:literal) => {{
        use $day::*;
        let input = read_file("inputs", $day_num);

        $c.bench_function(&format!("Day {} - Part 1", $day_num), |b| b.iter(|| {
            part_one(&input)
        }));

        $c.bench_function(&format!("Day {} - Part 2", $day_num), |b| b.iter(|| {
            part_two(&input)
        }));
    }};
}

pub fn criterion_benchmark(c: &mut Criterion) {
    bench_day!(c, day01, 1);
    bench_day!(c, day02, 2);
    bench_day!(c, day03, 3);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);