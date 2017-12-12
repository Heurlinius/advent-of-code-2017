use std::collections::{HashSet, HashMap};

macro_rules! detect_loop {
    ($banks:expr) => {{
        let mut cycles = 0usize;
        let mut seen = HashSet::new();
        loop {
            cycles += 1;
            balance_memory(&mut $banks);
            if !seen.insert($banks.clone()) {
                break;
            }
        }
        cycles
    }}
}

macro_rules! count_cycles {
    ($banks:expr) => {{
        let mut cycle = 0usize;
        let mut seen = HashMap::new();
        let mut result = 0usize;
        loop {
            cycle += 1;
            balance_memory(&mut $banks);
            if seen.contains_key(&$banks) {
                result = cycle - seen[&$banks];
                break;
            } else {
                seen.insert($banks.clone(), cycle);
            }
        }
        result
    }}
}

fn balance_memory(banks: &mut [u8]) {
    let (mut i, mut value) = banks.iter()
        .enumerate()
        .fold((0, 0), |(max_idx, max_val), (idx, &val)|
            if val > max_val {
                (idx, val)
            } else {
                (max_idx, max_val)
            });
    // Empty the starting bank
    banks[i] = 0;
    while value > 0 {
        // Advance to the next bank
        i = (i + 1) % banks.len();
        banks[i] += 1;
        value -= 1;
    }
}

fn main() {
    let input = [10, 3, 15, 10, 5, 15, 5, 15, 9, 2, 5, 8, 5, 2, 3, 6];

    let mut banks = input.clone();
    let part1 = detect_loop!(banks);
    println!("Part 1 = {}", part1);

    let mut banks = input.clone();
    let part2 = count_cycles!(banks);
    println!("Part 2 = {}", part2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_balance1() {
        let mut banks = [1, 2, 3];
        balance_memory(&mut banks);
        assert_eq!(banks, [2, 3, 1]);
    }

    #[test]
    fn test_balance2() {
        let mut banks = [1, 3, 3];
        balance_memory(&mut banks);
        assert_eq!(banks, [2, 1, 4]);
    }

    #[test]
    fn test_loop() {
        let mut banks = [0, 2, 7, 0];
        let cycles = detect_loop!(banks);
        assert_eq!(cycles, 5);
    }

    #[test]
    fn test_count() {
        let mut banks = [0, 2, 7, 0];
        let cycles = count_cycles!(banks);
        assert_eq!(cycles, 4);
    }
}
