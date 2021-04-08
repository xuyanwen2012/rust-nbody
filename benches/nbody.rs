use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
// use nbody_rs::ParticleList;
use nbody_rs::Universe;

fn criterion_benchmark(c: &mut Criterion) {
    let size = usize::pow(2, 16);
    let universe = Universe::new(size);

    c.bench_with_input(
        BenchmarkId::new("input_example", size),
        &universe,
        |b, u| {
            b.iter(|| {
                u.clone().next_state_par();
            });
        },
    );
    //
    // let mut group = c.benchmark_group("next_state_par");
    //
    // for exp in 10..15 {
    //     let size = usize::pow(2, exp);
    //     let particles = Universe::new(size);
    //
    //     group.bench_with_input(BenchmarkId::from_parameter(size), &particles, |b, list| {
    //         b.iter(|| {
    //             let _ = list.clone().next_state_par();
    //         })
    //     });
    // }
    // group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
