use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mul_persistence_rust::*;
use std::time::Duration;
use tokio::runtime::Runtime;

fn bench_new_solution(c: &mut Criterion) {
    let mut group = c.benchmark_group("Multiplicative Persistence New Solution");

    group.sample_size(50);

    group.measurement_time(Duration::from_secs(10));

    group.bench_function(
        format!("Check digits of length 2 - 7 without tokio concurrency"),
        move |b| {
            b.to_async(Runtime::new().unwrap()).iter(|| async {
                for i in 2..7 {
                    let permutations = create_permutations(black_box(i));
                    compute_persistences(permutations).await;
                }
            });
        },
    );

    group.bench_function(
        format!("Check digits of length 2 - 7 with tokio concurrency"),
        move |b| {
            b.to_async(Runtime::new().unwrap()).iter(|| async move {
                let mut future = tokio::spawn(async { create_permutations(2) });
                for i in 3..7 {
                    let permutations = future.await.unwrap();
                    future = tokio::spawn(async move { create_permutations(i) });
                    compute_persistences(permutations).await;
                }
            });
        },
    );
}

criterion_group!(benches, bench_new_solution);
criterion_main!(benches);
