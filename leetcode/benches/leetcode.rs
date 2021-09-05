use criterion::{black_box, criterion_group, criterion_main, Criterion};

use leetcode::s0035_search_insert_position::Solution;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("search_insert_first 1", |b| {
        b.iter(|| {
            let nums = vec![1, 3, 5, 6];
            Solution::search_insert(black_box(nums), 5);
        })
    });
    c.bench_function("search_insert_first 2", |b| {
        b.iter(|| {
            let nums = vec![1, 3, 5, 6];
            Solution::search_insert_first(black_box(nums), 5);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
