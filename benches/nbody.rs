use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use nbody_rs::ParticleList;

fn criterion_benchmark(c: &mut Criterion) {
    let particles = ParticleList::new(1024);

    c.bench_with_input(
        BenchmarkId::new("next_state_par", 1024u64),
        &particles,
        |b, list| {
            b.iter(|| {
                let _ = list.clone().next_state_par();
            })
        },
    );

    // c.bench_function("gravity_func", |b| {
    //     b.iter(|| gravity_func(black_box(vec2::Vec2 { x: 0.5, y: 0.5 })))
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
