use std::collections::HashMap;

use crate::shared::{Day, PartSolution};

fn find_sum_of_2_is(sum: u64, rest: &[u64]) -> Result<(), String> {
    let mut missing_to_value: HashMap<u64, u64> = HashMap::new();

    for n in rest {
        match missing_to_value.get(n) {
            Some(_) => {
                return Ok(());
            },
            None => {
                // no point in having negative values
                if sum >= *n {
                    missing_to_value.insert(sum - n, *n);
                }
            },
        }
    }

    Err(format!(
        "No combination found in the last {} that equals {}",
        rest.len(),
        sum
    ))
}

fn slide_until_sum_of_any_2_in_last_x_is_not_current_value(input: &[u64], last_x: usize) -> u64 {
    let mut offset = 0;

    for to_test in input.windows(last_x) {
        let target_sum = *input.get(offset + last_x).unwrap();

        if find_sum_of_2_is(target_sum, to_test).is_ok() {
            offset += 1;
        } else {
            return target_sum;
        }
    }

    panic!("Shouldn't get here");
}

fn find_contiguous_set_of_numbers_that_sum_up_to(input: &[u64], target: u64) -> (u64, u64) {
    let mut offset = 0;
    let mut to_take = 2;

    loop {
        let range: Vec<&u64> = input.iter().skip(offset).take(to_take).collect();

        let sum: u64 = range.iter().fold(0, |acc, current| acc + **current);

        match target.cmp(&sum) {
            std::cmp::Ordering::Less => {
                offset += 1;
                to_take = 2;
            },
            std::cmp::Ordering::Equal => {
                let min = *range.iter().min().unwrap();
                let max = *range.iter().max().unwrap();
                return (*min, *max);
            },
            std::cmp::Ordering::Greater => {
                to_take += 1;
            },
        }
    }
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("day_09/input.txt")
            .lines()
            .map(Into::into)
            .collect();
        let input: Vec<u64> = lines.iter().map(|s| s.parse::<u64>().unwrap()).collect();

        let solution = slide_until_sum_of_any_2_in_last_x_is_not_current_value(&input, 25);

        PartSolution::U64(solution)
    }

    fn part_2(&self) -> PartSolution {
        const TARGET: u64 = 138_879_426; // from day 9 part 1;
        let lines: Vec<String> = include_str!("day_09/input.txt")
            .lines()
            .map(Into::into)
            .collect();
        let input: Vec<u64> = lines.iter().map(|s| s.parse::<u64>().unwrap()).collect();

        let (lowest, highest) = find_contiguous_set_of_numbers_that_sum_up_to(&input, TARGET);

        PartSolution::U64(lowest + highest)
    }
}

#[cfg(test)]
mod tests {

    mod part_1 {
        use crate::day_09::{Solution, slide_until_sum_of_any_2_in_last_x_is_not_current_value};
        use crate::shared::{Day as _, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U64(138_879_426), (Solution {}).part_1());
        }

        #[test]
        fn example() {
            let input: Vec<u64> = vec![
                "35", "20", "15", "25", "47", "40", "62", "55", "65", "95", "102", "117", "150",
                "182", "127", "219", "299", "277", "309", "576",
            ]
            .iter()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

            let value = slide_until_sum_of_any_2_in_last_x_is_not_current_value(&input, 5);

            assert_eq!(127, value);
        }
    }

    mod part_2 {
        use crate::day_09::{Solution, find_contiguous_set_of_numbers_that_sum_up_to};
        use crate::shared::{Day as _, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U64(23_761_694), (Solution {}).part_2());
        }

        #[test]
        fn example() {
            let input: Vec<u64> = vec![
                "35", "20", "15", "25", "47", "40", "62", "55", "65", "95", "102", "117", "150",
                "182", "127", "219", "299", "277", "309", "576",
            ]
            .iter()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

            let (lowest, highest) = find_contiguous_set_of_numbers_that_sum_up_to(&input, 127);

            assert_eq!(15, lowest);
            assert_eq!(47, highest);
        }
    }
}
