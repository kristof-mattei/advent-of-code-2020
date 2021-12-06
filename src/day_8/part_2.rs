use std::collections::HashSet;

use super::part_1::{parse_lines, Operation};

fn execute_until_same_line_reached(operations: &[Operation]) -> Result<i32, String> {
    let length = operations.len();

    let mut has_visited: HashSet<usize> = HashSet::new();

    let mut index: i32 = 0;

    let mut accumulator = 0;

    loop {
        if index == length as i32 {
            return Ok(accumulator);
        }
        index = index.wrapping_rem_euclid(length as i32);

        if !has_visited.insert(index as usize) {
            return Err("Program did not terminate".into());
        };

        match operations.get(index as usize) {
            Some(operation) => match operation {
                Operation::Acc(acc) => {
                    accumulator += acc;
                    index += 1;
                }
                Operation::Jmp(jmp) => {
                    index += jmp;
                }
                Operation::Nop(_) => {
                    index += 1;
                }
            },
            None => return Ok(accumulator),
        }
    }
}

// https://adventofcode.com/2020/day/8
pub fn find_solution() -> i32 {
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
            Ok(acc) => return acc,
            _ => continue,
        }
    }

    panic!("No non-terminating combination found")
}

fn build_new_vector(operations: &[Operation], to_swap_index: usize) -> Vec<Operation> {
    // take the part up to the index
    let mut copy: Vec<_> = operations.iter().take(to_swap_index).copied().collect();

    let mut rest: Vec<_> = operations
        .iter()
        .skip(to_swap_index + 1)
        .take(operations.len() - to_swap_index - 1)
        .copied()
        .collect();

    let flipped_operation = match operations[to_swap_index] {
        Operation::Nop(x) => Operation::Jmp(x),
        Operation::Jmp(x) => Operation::Nop(x),
        Operation::Acc(_) => panic!("index shouldn't refer to acc"),
    };

    copy.push(flipped_operation);

    copy.append(&mut rest);

    copy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(920, find_solution())
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

        println!("{:#?}", new_vector);

        let acc = execute_until_same_line_reached(&new_vector);

        assert_eq!(8, acc.unwrap());
    }

    #[test]
    fn pieces() {
        const SPLIT_AT: usize = 5;
        let vec1: Vec<i32> = (0..=10).into_iter().collect();
        let vec2: Vec<&i32> = vec1.iter().take(SPLIT_AT).collect();

        let vec3: Vec<&i32> = vec1
            .iter()
            .skip(SPLIT_AT + 1)
            .take(vec1.len() - SPLIT_AT - 1)
            .collect();

        println!("{:?}", vec1);
        println!("{:?}", vec2);
        println!("{:?}", vec3);
    }
}
