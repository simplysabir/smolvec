use criterion::{black_box, criterion_group, criterion_main, Criterion};
pub use smolvec::SmolVec;

fn bench_push_small(c: &mut Criterion) {
    c.bench_function("push_small_vec", |b| {
        b.iter(|| {
            let mut vec = SmolVec::new();
            for i in 0..4 {
                vec.push(black_box(i));
            }
        })
    });
    c.bench_function("push_std_vec", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for i in 0..4 {
                vec.push(black_box(i));
            }
        })
    });
}

fn bench_push_large(c: &mut Criterion) {
    c.bench_function("push_large_small_vec", |b| {
        b.iter(|| {
            let mut vec = SmolVec::new();
            for i in 0..1000 {
                vec.push(black_box(i));
            }
        })
    });
    c.bench_function("push_large_std_vec", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for i in 0..1000 {
                vec.push(black_box(i));
            }
        })
    });
}

fn bench_pop(c: &mut Criterion) {
    c.bench_function("pop_small_vec", |b| {
        b.iter(|| {
            let mut vec = SmolVec::new();
            for i in 0..4 {
                vec.push(i);
            }
            for _ in 0..4 {
                black_box(vec.pop());
            }
        })
    });
    c.bench_function("pop_std_vec", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for i in 0..4 {
                vec.push(i);
            }
            for _ in 0..4 {
                black_box(vec.pop());
            }
        })
    });
}

criterion_group!(benches, bench_push_small, bench_push_large, bench_pop);
criterion_main!(benches);
