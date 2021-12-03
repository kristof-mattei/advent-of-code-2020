use std::collections::HashMap;

use crate::utils::read_file;

fn find_sum_of_2_is_2020(fixed: u32, rest: Vec<u32>) -> Option<(u32, u32)> {
    let mut missing_to_value: HashMap<i32, i32> = HashMap::new();

    for n in rest {
        match missing_to_value.get(&(n as i32)) {
            Some(x) => {
                return Some(((*x) as u32, n));
            }
            None => {
                missing_to_value.insert(2020_i32 - fixed as i32 - n as i32, n as i32);
            }
        }
    }

    None
}

fn find_sum_of_3_is_2020(numbers: &[u32]) -> (u32, u32, u32) {
    for n in numbers {
        let mut vec_without_n = numbers.to_owned();
        vec_without_n.retain(|r| *r != *n);

        match find_sum_of_2_is_2020(*n, vec_without_n) {
            None => (),
            Some((part2, part3)) => {
                return (*n, part2, part3);
            }
        }
    }

    panic!("No sum of 2020 found")
}

// https://adventofcode.com/2020/day/1
pub fn find_solution() -> Result<u32, Box<dyn std::error::Error>> {
    let split = read_file("./src/day_1/input.txt".into())?;

    let numbers: Vec<u32> = lines
        .into_iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let (entry1, entry2, entry3) = find_sum_of_3_is_2020(&numbers)
        .ok_or_else::<String, _>(|| "Didn't find a sum of x + y + z = 2020".into())?;

    Ok(entry1 * entry2 * entry3)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn outcome() {
        assert_eq!(100_655_544, find_solution().unwrap());
    }
}
