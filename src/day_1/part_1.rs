use std::collections::HashMap;

fn find_sum_of_2_is_2020(numbers: &[u32]) -> (u32, u32) {
    let mut missing_to_value: HashMap<u32, u32> = HashMap::new();

    for n in numbers {
        match missing_to_value.get(n) {
            Some(x) => {
                return (*x, *n);
            }
            None => {
                missing_to_value.insert(2020 - n, *n);
            }
        }
    }

    panic!("No sum of 2020 found");
}

// https://adventofcode.com/2020/day/1
pub fn find_solution() -> u32 {
    let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

    let numbers: Vec<u32> = lines
        .into_iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let (entry1, entry2) = find_sum_of_2_is_2020(&numbers);

    entry1 * entry2
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(1_019_571, find_solution());
    }
}
