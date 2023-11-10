use std::fmt::Display;

use num_bigint::BigUint;
use num_traits::One;

use rayon::prelude::*;

/// A struct containing a Vec of values that represent
/// how many of each digit a given number has (0 - 9)
#[derive(Debug, Clone)]
pub struct MPDigits {
    pub digits: Vec<usize>,
}

impl MPDigits {
    pub fn new() -> Self {
        MPDigits {
            digits: vec![0; 10],
        }
    }
}

// Default implemented here allows for nicer constructions
impl Default for MPDigits {
    fn default() -> Self {
        Self::new()
    }
}

// PartialEq allos for dedup operations
impl PartialEq for MPDigits {
    fn eq(&self, other: &Self) -> bool {
        self.digits == other.digits
    }
}

impl Display for MPDigits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output: String = self
            .digits
            .iter()
            .enumerate()
            .map(|(x, &i)| {
                (0..i)
                    .map(|_| (x + '0' as usize) as u8 as char)
                    .collect::<String>()
            })
            .collect();

        f.write_fmt(format_args!("{}", output))
    }
}

fn calculate_permutations(
    permutations: &mut Vec<MPDigits>,
    permutation: &mut MPDigits,
    k: usize,
    n: usize,
    usable_digits: Vec<usize>,
) {
    // Cases where we have two 2s (4) or two 3s (9) are skipped
    if permutation.digits[2] > 1
    || permutation.digits[3] > 1
    // Cases where we have a 2 and 3 (6) or 2 and 4 (8) are skipped
    || permutation.digits[2] == 1 && (permutation.digits[3] >= 1 || permutation.digits[4] >= 1)
    {
        return;
    }
    // If we've run out of digits to append, then we've created a new permutation
    else if k == 0 {
        permutations.push(permutation.to_owned());
        return;
    }
    // Otherwise, we create new permutations
    for i in 0..n {
        let mut new_permutation = permutation.clone();
        new_permutation.digits[usable_digits[i]] += 1;
        calculate_permutations(
            permutations,
            &mut new_permutation,
            k - 1,
            n,
            usable_digits.to_owned(),
        )
    }
}

/// Returns all permutations of every
/// number with a given number of digits.
#[inline(always)]
pub fn create_permutations(length: usize) -> Vec<MPDigits> {
    let mut permutations = Vec::new();

    let usable_digits: Vec<usize> = vec![2, 3, 4, 6, 7, 8, 9];

    let mut permutation = MPDigits::default();

    calculate_permutations(
        &mut permutations,
        &mut permutation,
        length,
        usable_digits.len(),
        usable_digits,
    );

    permutations
}

fn extract_digits(num: &BigUint, digits: &mut MPDigits) {
    for i in 0..digits.digits.len() {
        digits.digits[i] = 0;
    }
    num.to_string().chars().for_each(|x| {
        digits.digits[x as usize - '0' as usize] += 1;
    });
}

pub async fn compute_persistences(mut nums: Vec<MPDigits>) -> Vec<(MPDigits, u8)> {
    let persistences: Vec<(MPDigits, u8)> = nums
        .par_iter_mut()
        .map(|digits| {
            // Copy down the original digits used
            let original_digits = digits.clone();
            // Instatiate a new persistence and num value
            let mut persistence: u8 = 0;
            // Loop through until num is one digit long
            loop {
                let len: usize = digits.digits.iter().sum();
                // Break case
                if len <= 1 {
                    break;
                } else {
                    persistence += 1;
                }

                // Checks for skip cases
                // If a number has a zero, its persistence will end on this iteration
                if digits.digits[0] > 0 {
                    return (original_digits, persistence);
                }
                // If a number has an even digit and a five, the next iteration will have a zero.
                if (digits.digits[2] > 0
                    || digits.digits[4] > 0
                    || digits.digits[6] > 0
                    || digits.digits[8] > 0)
                    && digits.digits[5] > 0
                {
                    return (original_digits, persistence + 1);
                }

                // Initialize number to be found
                let mut num: BigUint = One::one();

                // Perform multiplication
                for (index, pow) in digits.digits.iter().enumerate().skip(2) {
                    if *pow == 0 {
                        continue;
                    }
                    num *= BigUint::from(index).pow(*pow as u32);
                }

                // Extract the digits generated
                extract_digits(&num, digits);
                // Repeat loop
            }
            // Return our original queried digits with their persistence
            (original_digits, persistence)
        })
        .collect();

    persistences
}
