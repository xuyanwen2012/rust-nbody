use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use nbody_rs::ParticleList;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("next_state_par");

    for exp in 10..15 {
        let size = usize::pow(2, exp);
        let particles = ParticleList::new(size);

        group.bench_with_input(BenchmarkId::from_parameter(size), &particles, |b, list| {
            b.iter(|| {
                let _ = list.clone().next_state_par();
            })
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
