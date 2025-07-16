use std::collections::HashSet;

use crate::shared::{Day, PartSolution};

fn k_sums_unsorted(mut nums: Vec<i32>, target: i32, k: usize) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    k_sum(&nums, target, k)
}

fn k_sum(nums: &[i32], target: i32, k: usize) -> Vec<Vec<i32>> {
    // assert!(nums.is_sorted());

    let mut result = Vec::<Vec<i32>>::new();

    if nums.is_empty() {
        return result;
    }

    // ensure we can actually make a matching k with the values in
    // our nums
    let average_value = target / i32::try_from(k).unwrap();

    // ensure average value falls between
    if nums[0] > average_value || average_value > nums[nums.len() - 1] {
        return result;
    }

    if k == 2 {
        return two_sum(nums, target);
    }

    for i in 0..nums.len() {
        // detect duplicates
        if i == 0 || nums[i - 1] != nums[i] {
            let subsets = k_sum(&nums[i + 1..], target - nums[i], k - 1);

            for mut subset in subsets {
                subset.push(nums[i]);

                result.push(subset);
            }
        }
    }

    result
}

/// nums needs to be sorted!
fn two_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
    // assert!(nums.is_sorted());

    let mut result = vec![];
    let mut hash_set = HashSet::new();

    for num in nums {
        if result.last().is_none_or(|v: &Vec<i32>| &v[0] != num)
            && hash_set.contains(&(target - num))
        {
            result.push(vec![*num, target - num]);
        }

        hash_set.insert(num);
    }

    result
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines = include_str!("day_01/input.txt");

        let numbers = lines.lines().map(|s| s.parse::<i32>().unwrap()).collect();

        let results = k_sums_unsorted(numbers, 2020, 2);

        PartSolution::I32(results[0][0] * results[0][1])
    }

    fn part_2(&self) -> PartSolution {
        let lines = include_str!("day_01/input.txt");

        let numbers = lines.lines().map(|s| s.parse::<i32>().unwrap()).collect();

        let results = k_sums_unsorted(numbers, 2020, 3);

        PartSolution::I32(results[0][0] * results[0][1] * results[0][2])
    }
}

#[cfg(test)]
mod tests {
    mod part_1 {
        use crate::day_01::{Solution, k_sums_unsorted};
        use crate::shared::{Day as _, PartSolution};

        #[test]
        fn example() {
            let input = vec![1721, 979, 366, 299, 675, 1456];

            let results = k_sums_unsorted(input, 2020, 2);

            assert_eq!(results.len(), 1);
            assert_eq!(results.first().map_or(0, Vec::len), 2);

            assert_eq!(results[0][0] * results[0][1], 514_579);
        }

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::I32(1_019_571), (Solution {}).part_1());
        }
    }

    mod part_2 {
        use crate::day_01::{Solution, k_sums_unsorted};
        use crate::shared::{Day as _, PartSolution};

        #[test]
        fn example() {
            let input = vec![1721, 979, 366, 299, 675, 1456];

            let results = k_sums_unsorted(input, 2020, 3);

            assert_eq!(results.len(), 1);
            assert_eq!(results.first().map_or(0, Vec::len), 3);

            assert_eq!(results[0][0] * results[0][1] * results[0][2], 241_861_950);
        }

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::I32(100_655_544), (Solution {}).part_2());
        }
    }
}
