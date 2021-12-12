use std::collections::HashMap;

use crate::shared::{Day, PartSolution};

fn parse_group_of_answers(group: &[String]) -> u32 {
    let mut answers: Vec<char> = Vec::new();

    for line in group {
        line.chars().for_each(|p| answers.push(p));
    }

    answers.sort_unstable();
    answers.dedup();

    answers.len() as u32
}

fn count_of_questions_answered_by_everybody(group: &[String]) -> u32 {
    let mut count_of_answers: HashMap<char, u32> = HashMap::new();

    for line in group {
        let mut duplicate_answer_per_line_check: Vec<char> = line.chars().into_iter().collect();
        duplicate_answer_per_line_check.sort_unstable();
        duplicate_answer_per_line_check.dedup();

        assert_eq!(duplicate_answer_per_line_check.len(), line.len());

        for c in duplicate_answer_per_line_check {
            let count = *(count_of_answers.get(&c).unwrap_or(&0));

            let _ = count_of_answers.insert(c, count + 1);
        }
    }

    let mut total_answers_that_everybody_answered: u32 = 0;
    let people_in_group = group.len() as u32;

    for (_, value) in count_of_answers {
        if value == people_in_group {
            total_answers_that_everybody_answered += 1;
        }
    }

    total_answers_that_everybody_answered
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let mut all_answers: Vec<u32> = Vec::new();

        let groups = lines.split(String::is_empty);

        for group in groups {
            let unique_answers_in_group = parse_group_of_answers(group);

            all_answers.push(unique_answers_in_group);
        }

        PartSolution::U32(all_answers.iter().sum::<u32>())
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let mut all_answers: Vec<u32> = Vec::new();

        let groups = lines.split(String::is_empty);

        for group in groups {
            let unique_answers_in_group = count_of_questions_answered_by_everybody(group);

            all_answers.push(unique_answers_in_group);
        }

        PartSolution::U32(all_answers.iter().sum::<u32>())
    }
}

#[cfg(test)]
mod tests {
    mod part_1 {
        use crate::{
            day_06::{parse_group_of_answers, Solution},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U32(6521), (Solution {}).part_1());
        }

        #[test]
        fn answer_set_1() {
            let answer_set = ["abc".to_string()];

            assert_eq!(parse_group_of_answers(&answer_set), 3);
        }

        #[test]
        fn answer_set_2() {
            let answer_set = ["a".to_string(), "b".to_string(), "c".to_string()];

            assert_eq!(parse_group_of_answers(&answer_set), 3);
        }

        #[test]
        fn answer_set_3() {
            let answer_set = ["ab".to_string(), "ac".to_string()];

            assert_eq!(parse_group_of_answers(&answer_set), 3);
        }

        #[test]
        fn answer_set_4() {
            let answer_set = [
                "a".to_string(),
                "a".to_string(),
                "a".to_string(),
                "a".to_string(),
            ];

            assert_eq!(parse_group_of_answers(&answer_set), 1);
        }

        #[test]
        fn answer_set_5() {
            let answer_set = ["b".to_string()];

            assert_eq!(parse_group_of_answers(&answer_set), 1);
        }
    }

    mod part_2 {
        use crate::{
            day_06::{count_of_questions_answered_by_everybody, Solution},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U32(3305), (Solution {}).part_2());
        }

        #[test]
        fn answer_set_1() {
            let answer_set = ["abc".to_string()];

            assert_eq!(count_of_questions_answered_by_everybody(&answer_set), 3);
        }

        #[test]
        fn answer_set_2() {
            let answer_set = ["a".to_string(), "b".to_string(), "c".to_string()];

            assert_eq!(count_of_questions_answered_by_everybody(&answer_set), 0);
        }

        #[test]
        fn answer_set_3() {
            let answer_set = ["ab".to_string(), "ac".to_string()];

            assert_eq!(count_of_questions_answered_by_everybody(&answer_set), 1);
        }

        #[test]
        fn answer_set_4() {
            let answer_set = [
                "a".to_string(),
                "a".to_string(),
                "a".to_string(),
                "a".to_string(),
            ];

            assert_eq!(count_of_questions_answered_by_everybody(&answer_set), 1);
        }

        #[test]
        fn answer_set_5() {
            let answer_set = ["b".to_string()];

            assert_eq!(count_of_questions_answered_by_everybody(&answer_set), 1);
        }
    }
}
