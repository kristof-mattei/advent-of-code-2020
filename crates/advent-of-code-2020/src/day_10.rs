use hashbrown::HashMap;

use crate::shared::{Day, PartSolution};

fn calculate_possibilities(mut input: Vec<u32>) -> u64 {
    input.sort_unstable();

    input.push(input.last().map(|x| x + 3).unwrap());

    let mut start_with_0 = Vec::with_capacity(input.len() + 1);
    start_with_0.push(0);
    start_with_0.append(&mut input);

    paths_to_end(&mut HashMap::new(), &start_with_0, 0)
}

fn paths_to_end(cache: &mut HashMap<usize, u64>, adapters: &[u32], current: usize) -> u64 {
    if let Some(v) = cache.get(&current) {
        return *v;
    }

    // last one?
    if current == (adapters.len() - 1) {
        // one step to get here
        return 1;
    }

    // check the next 3 next items and if we can reach them. If so, calculate next steps from there
    let current_value = adapters[current];

    let sum = adapters
        .iter()
        .enumerate()
        .skip(current + 1)
        .take_while(|&(_, v)| v - current_value <= 3)
        .map(|(n, _)| paths_to_end(cache, adapters, n))
        .sum();

    cache.insert(current, sum);

    sum
}

fn calculate_step_up_differences(mut input: Vec<u32>) -> (u32, u32, u32) {
    let mut stepup_1 = 0;
    let mut stepup_2 = 0;
    let mut stepup_3 = 0;

    input.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());

    let mut previous = 0;

    loop {
        match input.pop() {
            Some(next) => {
                match next - previous {
                    1 => stepup_1 += 1,
                    2 => stepup_2 += 1,
                    3 => stepup_3 += 1,
                    _ => {
                        panic!("This really shouldn't happen");
                    },
                }

                previous = next;
            },
            None => return (stepup_1, stepup_2, stepup_3 + 1),
        }
    }
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let input: Vec<u32> = include_str!("day_10/input.txt")
            .lines()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let (s1, _, s3) = calculate_step_up_differences(input);

        PartSolution::U32(s1 * s3)
    }

    fn part_2(&self) -> PartSolution {
        let input: Vec<u32> = include_str!("day_10/input.txt")
            .lines()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let total = calculate_possibilities(input);

        PartSolution::U64(total)
    }
}

#[cfg(test)]
mod tests {

    mod part_1 {
        use pretty_assertions::assert_eq;

        use crate::day_10::{Solution, calculate_step_up_differences};
        use crate::shared::{Day as _, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U32(1820), (Solution {}).part_1());
        }

        #[test]
        fn example_1() {
            let input: Vec<u32> = ["16", "10", "15", "5", "1", "11", "7", "19", "6", "12", "4"]
                .iter()
                .map(|s| s.parse::<u32>().unwrap())
                .collect();

            let (stepup_1, _stepup_2, stepup_3) = calculate_step_up_differences(input);

            assert_eq!(7, stepup_1);
            assert_eq!(5, stepup_3);
        }

        #[test]
        fn example_2() {
            let input: Vec<u32> = vec![
                "28", "33", "18", "42", "31", "14", "46", "20", "48", "47", "24", "23", "49", "45",
                "19", "38", "39", "11", "1", "32", "25", "35", "8", "17", "7", "9", "4", "2", "34",
                "10", "3",
            ]
            .iter()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

            let (stepup_1, _stepup_2, stepup_3) = calculate_step_up_differences(input);

            assert_eq!(22, stepup_1);
            assert_eq!(10, stepup_3);
        }
    }

    mod part_2 {
        use pretty_assertions::assert_eq;

        use crate::day_10::{Solution, calculate_possibilities};
        use crate::shared::{Day as _, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U64(3_454_189_699_072), (Solution {}).part_2());
        }

        #[test]
        fn example_1() {
            let input: Vec<u32> = ["16", "10", "15", "5", "1", "11", "7", "19", "6", "12", "4"]
                .iter()
                .map(|s| s.parse::<u32>().unwrap())
                .collect();

            let total = calculate_possibilities(input);

            assert_eq!(8, total);
        }

        #[test]
        fn example_2() {
            let input: Vec<u32> = vec![
                "28", "33", "18", "42", "31", "14", "46", "20", "48", "47", "24", "23", "49", "45",
                "19", "38", "39", "11", "1", "32", "25", "35", "8", "17", "7", "9", "4", "2", "34",
                "10", "3",
            ]
            .iter()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

            let total = calculate_possibilities(input);

            assert_eq!(19208, total);
        }
    }
}
