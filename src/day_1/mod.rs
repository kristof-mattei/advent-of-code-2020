use std::collections::HashMap;

use crate::shared::{Day, PartSolution};

fn find_sum_of_2_is(sum: u32, rest: &[u32]) -> Option<(u32, u32)> {
    let mut missing_to_value: HashMap<u32, u32> = HashMap::new();

    for n in rest {
        match missing_to_value.get(n) {
            Some(x) => {
                println!("{:?}", missing_to_value);
                return Some((*x, *n));
            }
            None => {
                // no point in having negative values
                if sum >= *n {
                    missing_to_value.insert(sum - n, *n);
                }
            }
        }
    }

    None
}

fn find_sum_of_3_is_2020(numbers: &[u32]) -> (u32, u32, u32) {
    for n in numbers {
        let mut vec_without_n = numbers.to_owned();
        vec_without_n.retain(|r| *r != *n);

        match find_sum_of_2_is(2020 - *n, &vec_without_n) {
            None => (),
            Some((part2, part3)) => {
                return (*n, part2, part3);
            }
        }
    }

    panic!("No sum of 2020 found");
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines = include_str!("input.txt");

        let numbers: Vec<u32> = lines.lines().map(|s| s.parse::<u32>().unwrap()).collect();

        let (entry1, entry2) = find_sum_of_2_is(2020, &numbers).unwrap();

        PartSolution::U32(entry1 * entry2)
    }

    fn part_2(&self) -> PartSolution {
        let lines = include_str!("input.txt");

        let numbers: Vec<u32> = lines.lines().map(|s| s.parse::<u32>().unwrap()).collect();

        let (entry1, entry2, entry3) = find_sum_of_3_is_2020(&numbers);

        PartSolution::U32(entry1 * entry2 * entry3)
    }
}

#[cfg(test)]
mod tests {
    mod part_1 {
        use crate::{
            day_1::Solution,
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U32(1_019_571), (Solution {}).part_1());
        }
    }

    mod part_2 {
        use crate::{
            day_1::Solution,
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U32(100_655_544), (Solution {}).part_2());
        }
    }
}
