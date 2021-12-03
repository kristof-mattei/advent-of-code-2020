use std::collections::HashMap;

use crate::utils::read_file;

fn find_sum_of_2_is_2020(numbers: &[u32]) -> Option<(u32, u32)> {
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
pub fn find_solution() -> Result<u32, Box<dyn std::error::Error>> {
    let split = read_file("./src/day_1/input.txt".into())?;

    let numbers: Vec<u32> = lines
        .into_iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let (entry1, entry2) = find_sum_of_2_is_2020(&numbers)
        .ok_or_else::<String, _>(|| "Didn't find a sum of x + y = 2020".into())?;

    Ok(entry1 * entry2)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(1_019_571, find_solution().unwrap());
    }
}
