use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc::solutions::day06;
use aoc::read_file;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = read_file("inputs", 6);
    
    c.bench_function("simd window 14 - vector 16", |b| b.iter(|| {
        day06::index_simd::<16>(&input, 14)
    }));

    c.bench_function("simd window size 4 - vector 4", |b| b.iter(|| {
        day06::index_simd::<4>(&input, 4)
    }));

    c.bench_function("xor window size 14", |b| b.iter(|| {
        day06::_index_popcnt(&input, 14)
    }));

    c.bench_function("xor window size 4", |b| b.iter(|| {
        day06::_index_popcnt(&input, 4)
    }));


    c.bench_function("standard 14", |b| b.iter(|| {
        day06::_index_classical(&input, 14)
    }));

    c.bench_function("standard 4", |b| b.iter(|| {
        day06::_index_classical(&input, 4)
    }));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);