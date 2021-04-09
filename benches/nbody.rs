use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
// use nbody_rs::vec2::Vec2;
use nbody_rs::{get_gravity_at_raw_par, get_gravity_at_raw_seq, Universe};

// fn bench_get_gravity_raw(c: &mut Criterion) {
//     let mut group = c.benchmark_group("Get Gravity Raw");
//
//     // let p = Vec2 { x: 0.5, y: 0.5 };
//
//     for exp in 8..16 {
//         let size = usize::pow(2, exp);
//         let universe = Universe::new(size);
//         group.bench_with_input(BenchmarkId::new("Seq", size), &universe, |b, u| {
//             b.iter(|| get_gravity_at_raw_seq(p, &u.bodies.0))
//         });
//         group.bench_with_input(BenchmarkId::new("Par", size), &universe, |b, u| {
//             b.iter(|| get_gravity_at_raw_par(p, &u.bodies.0))
//         });
//     }
//
//     group.finish();
// }

fn bench_next_state(c: &mut Criterion) {
    let mut group = c.benchmark_group("next_state");

    let size = usize::pow(2, 12);
    let universe = Universe::new(size);

    group.bench_with_input(BenchmarkId::new("Seq", size), &universe, |b, u| {
        b.iter(|| {
            u.clone().next_state_seq();
        })
    });
    group.bench_with_input(BenchmarkId::new("Par", size), &universe, |b, u| {
        b.iter(|| {
            u.clone().next_state_par();
        })
    });
    group.finish();
}

criterion_group!(benches, bench_next_state);
criterion_main!(benches);
