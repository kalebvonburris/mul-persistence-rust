use criterion::{criterion_group, criterion_main, Criterion};
use mul_persistence_rust::*;
use tokio::{runtime::Runtime, sync::oneshot};

fn bench_new_solution(c: &mut Criterion) {
    let mut group = c.benchmark_group("Multiplicative Persistence New Solution");

    group.sample_size(50);

    group.bench_function(
        format!("Check digits of length 2 - 15 with tokio concurrency"),
        move |b| {
            b.to_async(Runtime::new().unwrap()).iter(|| async move {
                let mut future = tokio::spawn(async { create_permutations(2) });
                for i in 3..15 {
                    let permutations = future.await.unwrap();
                    future = tokio::spawn(async move { create_permutations(i) });
                    compute_persistences(permutations).await;
                }
            });
        },
    );

    group.bench_function(
        format!("Check digits of length 2 - 15 with tokio concurrency and queue"),
        move |b| {
            b.to_async(Runtime::new().unwrap()).iter(|| async move {
                let mut queue = PermutationQueue::new(vec![2, 3, 4, 6, 7, 8, 9]);
                let iteration_size = 5000;
                let (mut sender, mut reciever) = oneshot::channel();
                let mut future = tokio::spawn(async move {
                    let permutations = queue.yield_permutations(iteration_size);
                    let _ = sender.send((queue, permutations));
                });

                let mut length = 2;
                while length <= 15 {
                    future.await.unwrap();
                    let (mut queue, permutations) = reciever.try_recv().unwrap();
                    length = queue.length;
                    (sender, reciever) = oneshot::channel();
                    future = tokio::spawn(async move {
                        let permutations = queue.yield_permutations(iteration_size);
                        let _ = sender.send((queue, permutations));
                    });
                    compute_persistences(permutations).await;
                }
            });
        },
    );
}

criterion_group!(benches, bench_new_solution);
criterion_main!(benches);
