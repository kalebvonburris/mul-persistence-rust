use std::collections::HashMap;

fn count_valid_combinations(n: usize) -> usize {
    let digits = vec![2, 3, 4, 6, 7, 8, 9];
    let mut table: Vec<HashMap<Vec<usize>, usize>> = vec![HashMap::new(); n + 1];

    // Base case for n = 1
    for &digit in &digits {
        let mut base_case = vec![0; digits.len()];
        base_case[digits.iter().position(|&d| d == digit).unwrap()] = 1;
        table[1].insert(base_case, 1);
    }

    // Recursive cases for n > 1
    for length in 2..=n {
        for combination in generate_combinations(&digits, length) {
            let count = count_for_combination(&combination, &table, length - 1, &digits);
            table[length].insert(combination, count);
        }
    }

    // Summing up all combinations for n
    table[n].values().sum()
}

fn generate_combinations(digits: &[i32], length: usize) -> Vec<Vec<usize>> {
    // Generate all valid combinations of digits for a given length
    // This function needs to be implemented to efficiently generate combinations
    Vec::new() // Placeholder
}

fn count_for_combination(
    combination: &[usize], 
    table: &[HashMap<Vec<usize>, usize>], 
    prev_length: usize, 
    digits: &[i32]
) -> usize {
    // Count the number of valid combinations that lead to the current combination
    // This function needs to be implemented to count based on previous table entries
    0 // Placeholder
}

fn main() {
    let max_n = 50;
    for i in 1..(max_n + 1) {
        let count = count_valid_combinations(i);
        print!("({},{})", i, count);
    }
    println!()    
}
