use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc::solutions::day03;
use aoc::read_file;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = read_file("inputs", 3);
    
    c.bench_function("custom intersection", |b| b.iter(|| {
        let mut total_priority = 0;
        let mut sacks: Vec<_> = black_box(day03::read_rucksacks(&input));
        for rucksacks in sacks.chunks_mut(3) {
            let exclusive = day03::get_non_exclusive_item(rucksacks).unwrap();
            total_priority += exclusive.priority();
        }
        total_priority
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);