use std::collections::HashSet;

use crate::shared::{Day, PartSolution};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Operation {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn map_operation(operation: &str, argument: i32) -> Operation {
    match operation {
        "acc" => Operation::Acc(argument),
        "jmp" => Operation::Jmp(argument),
        "nop" => Operation::Nop(argument),
        _ => panic!("Yea, no"),
    }
}

pub fn parse_lines(lines: &[String]) -> Vec<Operation> {
    let mut instructions = Vec::new();
    for line in lines {
        let split: Vec<&str> = line.split(' ').collect();

        let argument = split[1].parse::<i32>().unwrap();
        let operation = map_operation(split[0], argument);

        instructions.push(operation);
    }

    instructions
}

#[derive(PartialEq, Debug)]
enum Ended {
    EndlessLoop(i32),
    TheEnd(i32),
}

fn execute_until_same_line_reached(operations: &[Operation]) -> Ended {
    let length = operations.len();

    let mut has_visited: HashSet<usize> = HashSet::new();

    let mut index: i32 = 0;

    let mut accumulator = 0;

    loop {
        if index == length as i32 {
            return Ended::TheEnd(accumulator);
        }
        index = index.wrapping_rem_euclid(length as i32);

        if !has_visited.insert(index as usize) {
            return Ended::EndlessLoop(accumulator);
        };

        match operations.get(index as usize) {
            Some(operation) => match operation {
                Operation::Acc(acc) => {
                    accumulator += acc;
                    index += 1;
                },
                Operation::Jmp(jmp) => {
                    index += jmp;
                },
                Operation::Nop(_) => {
                    index += 1;
                },
            },
            None => return Ended::TheEnd(accumulator),
        }
    }
}

fn build_new_vector(operations: &[Operation], to_swap_index: usize) -> Vec<Operation> {
    let mut copy: Vec<_> = operations.to_vec();

    let item_to_swap = copy.get_mut(to_swap_index).unwrap();

    let flipped_operation = match *item_to_swap {
        Operation::Nop(x) => Operation::Jmp(x),
        Operation::Jmp(x) => Operation::Nop(x),
        Operation::Acc(_) => panic!("index shouldn't refer to acc"),
    };

    *item_to_swap = flipped_operation;

    copy
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let operations = parse_lines(&lines);

        if let Ended::EndlessLoop(acc) = execute_until_same_line_reached(&operations) {
            PartSolution::I32(acc)
        } else {
            panic!("Application ended")
        }
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let operations = parse_lines(&lines);

        let to_swap: Vec<usize> = operations
            .iter()
            .enumerate()
            .filter_map(|(index, f)| match f {
                Operation::Nop(_) | Operation::Jmp(_) => Some(index),
                Operation::Acc(_) => None,
            })
            .collect();

        for to_swap_index in to_swap {
            let beginning = build_new_vector(&operations, to_swap_index);

            match execute_until_same_line_reached(&beginning) {
                Ended::TheEnd(acc) => return PartSolution::I32(acc),
                Ended::EndlessLoop(_) => continue,
            }
        }

        panic!("No non-terminating combination found");
    }
}

#[cfg(test)]
mod tests {

    mod part_1 {
        use crate::{
            day_08::{execute_until_same_line_reached, parse_lines, Ended, Operation, Solution},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::I32(1584), (Solution {}).part_1());
        }

        #[test]
        fn test_nop_postive() {
            let input = vec!["nop +1".to_string()];

            let expected = Operation::Nop(1);
            let parsed = parse_lines(&input);

            assert_eq!(parsed[0], expected);
        }

        #[test]
        fn test_nop_negative() {
            let input = vec!["nop -20".to_string()];

            let expected = Operation::Nop(-20);
            let parsed = parse_lines(&input);

            assert_eq!(parsed[0], expected);
        }

        #[test]
        fn test_jmp_positive() {
            let input = vec!["jmp +3".to_string()];

            let expected = Operation::Jmp(3);
            let parsed = parse_lines(&input);

            assert_eq!(parsed[0], expected);
        }

        #[test]
        fn test_jmp_negative() {
            let input = vec!["jmp -4".to_string()];

            let expected = Operation::Jmp(-4);
            let parsed = parse_lines(&input);

            assert_eq!(parsed[0], expected);
        }

        #[test]
        fn test_acc_positive() {
            let input = vec!["acc +5".to_string()];

            let expected = Operation::Acc(5);
            let parsed = parse_lines(&input);

            assert_eq!(parsed[0], expected);
        }

        #[test]
        fn test_acc_negative() {
            let input = vec!["acc -6".to_string()];

            let expected = Operation::Acc(-6);
            let parsed = parse_lines(&input);

            assert_eq!(parsed[0], expected);
        }

        #[test]
        fn modulo_test() {
            let items: Vec<char> = ('a'..='j').collect();

            let length = items.len() as i32;

            for i in -10..=length {
                let index = i.wrapping_rem_euclid(length);

                println!("{i} ({index}): {}", items[index as usize]);
            }
        }

        #[test]
        fn test_example() {
            let input: Vec<String> = vec![
                "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
                "acc +6",
            ]
            .iter()
            .map(|s| (*s).to_string())
            .collect();

            let operations = parse_lines(&input);

            let acc = execute_until_same_line_reached(&operations);

            assert_eq!(Ended::EndlessLoop(5), acc);
        }
    }

    mod part_2 {
        use crate::{
            day_08::{
                build_new_vector, execute_until_same_line_reached, parse_lines, Ended, Solution,
            },
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::I32(920), (Solution {}).part_2());
        }

        #[test]
        fn sample_data() {
            let input: Vec<String> = vec![
                "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
                "acc +6",
            ]
            .iter()
            .map(|s| (*s).to_string())
            .collect();

            let operations = parse_lines(&input);

            let new_vector = build_new_vector(&operations, 7);

            let acc = execute_until_same_line_reached(&new_vector);

            assert_eq!(Ended::TheEnd(8), acc);
        }

        #[test]
        fn pieces() {
            const SPLIT_AT: usize = 5;
            let vec1: Vec<i32> = (0..=10).collect();
            let vec2: Vec<&i32> = vec1.iter().take(SPLIT_AT).collect();

            let vec3: Vec<&i32> = vec1
                .iter()
                .skip(SPLIT_AT + 1)
                .take(vec1.len() - SPLIT_AT - 1)
                .collect();

            println!("{vec1:?}");
            println!("{vec2:?}");
            println!("{vec3:?}");
        }
    }
}
