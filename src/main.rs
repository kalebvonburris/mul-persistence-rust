use std::time::SystemTime;

use mul_persistence_rust::*;
use tokio::sync::oneshot;

const DIGITS: &[usize] = &[2, 3, 4, 6, 7, 8, 9];
const PERMUTATIONS_PER_ITERATION: usize = 10_000;

#[tokio::main]
async fn main() {
    // Create a queue for generating new numbers to compute
    let mut queue = PermutationQueue::new(DIGITS.to_vec());

    // Numbers of length 1 are already solved
    // Numbers of length 2 will be gathered and computed in a moment
    let mut len = 2;
    // Value for storing our best found solution
    let mut highest_persistence: (MPDigits, u8) = (MPDigits::default(), 1);

    let (mut sender, mut reciever) = oneshot::channel();

    // Joinhandle for calcuating permutations concurrently with solving them
    let mut future = tokio::spawn(async move {
        // Calculate permutations
        let permutations = queue.yield_permutations(PERMUTATIONS_PER_ITERATION);
        let _ = sender.send((queue, permutations));
    });

    let mut length_time = SystemTime::now();
    loop {
        // Await new digits to calculate
        future.await.unwrap();
        let (mut queue, mut persistences) = reciever.try_recv().unwrap();
        let new_length = queue.length;
        (sender, reciever) = oneshot::channel();
        // Start another thread to find more permutations
        future = tokio::spawn(async move {
            // Calculate permutations
            let permutations = queue.yield_permutations(PERMUTATIONS_PER_ITERATION);
            let _ = sender.send((queue, permutations));
        });
        // Remove duplicate tasks
        persistences.dedup();
        // Compute the permutations
        let solutions = compute_persistences(persistences).await;
        // Check if any solutions are superior to the best found before
        for solution in solutions {
            if solution.1 > highest_persistence.1 {
                highest_persistence = solution.clone();
            }
        }

        if new_length > len {
            // Print out the data regarding the current iteration
            println!(
                "{}: persistence {}. Working on length {}. {} microseconds spent working on this length.",
                highest_persistence.0, highest_persistence.1, new_length, length_time.elapsed().unwrap().as_micros()
            );
            len = new_length;
            length_time = SystemTime::now();
        }
    }
}
