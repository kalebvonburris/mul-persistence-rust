use std::{time::SystemTime, alloc::System};

use mul_persistence_rust::*;

#[tokio::main]
async fn main() {
    // Numbers of length 1 are already solved
    // Numbers of length 2 will be gathered and computed in a moment
    let mut len = 3;
    // Value for storing our best found solution
    let mut highest_persistence: (MPDigits, u8) = (MPDigits::default(), 1);
    // Joinhandle for calcuating permutations concurrently with solving them
    let mut future = tokio::spawn(async { create_permutations(2) });
    let mut permutation_time = SystemTime::now();
    loop {
        // Await new digits to calculate
        let mut permutations = future.await.unwrap();
        let end_permutation_time = permutation_time.elapsed().unwrap().as_secs_f64();
        // Start another thread to find more permutations
        future = tokio::spawn(async move { create_permutations(len) });
        let computation_time = SystemTime::now();
        // Remove duplicate tasks
        permutations.dedup();
        // Figure out how many permutations are left to compute
        let size = permutations.len();
        // Compute the permutations
        let solutions = compute_persistences(permutations).await;
        // Check if any solutions are superior to the best found before
        for solution in solutions {
            if solution.1 > highest_persistence.1 {
                highest_persistence = solution.clone();
            }
        }
        // Print out the data regarding the current iteration
        println!(
            "{}: {} for length {}. {} permutations solved. {}s spent waiting on permutations, {}s spent waiting on computation.",
            highest_persistence.0, highest_persistence.1, len, size, end_permutation_time, computation_time.elapsed().unwrap().as_secs_f64()
        );
        permutation_time = SystemTime::now();
        // Iterate
        len += 1;
    }
}
