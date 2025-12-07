use hashbrown::HashMap;

use crate::shared::{Day, PartSolution};

struct RuleWithPasswordPart1 {
    min: u32,
    max: u32,
    character: char,
    password: String,
}

impl RuleWithPasswordPart1 {
    pub fn is_valid(&self) -> bool {
        let mut counts = HashMap::new();

        self.password.chars().for_each(|c| {
            counts.entry(c).and_modify(|e| *e += 1).or_insert(1);
        });

        match counts.get(&self.character) {
            None => false,
            Some(t) => (self.min <= *t) && (*t <= self.max),
        }
    }
}

struct RuleWithPasswordPart2 {
    first_position: usize,
    second_position: usize,
    character: char,
    password: String,
}

impl RuleWithPasswordPart2 {
    pub fn is_valid(&self) -> bool {
        let p1_valid = self.password.chars().nth(self.first_position - 1) == Some(self.character);
        let p2_valid = self.password.chars().nth(self.second_position - 1) == Some(self.character);

        p1_valid ^ p2_valid
    }
}

fn parse_line(line: &str) -> (usize, usize, char, String) {
    // grammar:
    // <min>-<max> char: <password>
    let pieces: Vec<&str> = line.split(' ').collect();

    let min_max_vec: Vec<usize> = pieces[0]
        .split('-')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let char_with_colon_piece = pieces[1];
    let password = pieces[2];

    (
        *min_max_vec.first().unwrap(),
        *min_max_vec.get(1).unwrap(),
        char_with_colon_piece.chars().next().unwrap(),
        password.into(),
    )
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines = include_str!("day_02/input.txt");

        let valid_passwords = lines
            .lines()
            .map(parse_line)
            .map(|(min, max, character, password)| RuleWithPasswordPart1 {
                min: u32::try_from(min).unwrap(),
                max: u32::try_from(max).unwrap(),
                character,
                password,
            })
            .filter(RuleWithPasswordPart1::is_valid)
            .count();

        PartSolution::USize(valid_passwords)
    }

    fn part_2(&self) -> PartSolution {
        let lines = include_str!("day_02/input.txt");

        let valid_passwords = lines
            .lines()
            .map(parse_line)
            .map(
                |(first_position, second_position, character, password)| RuleWithPasswordPart2 {
                    first_position,
                    second_position,
                    character,
                    password,
                },
            )
            .filter(RuleWithPasswordPart2::is_valid)
            .count();

        PartSolution::USize(valid_passwords)
    }
}

#[cfg(test)]
mod tests {
    mod part_1 {
        use crate::day_02::Solution;
        use crate::shared::{Day as _, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::USize(620), (Solution {}).part_1());
        }
    }

    mod part_2 {
        use crate::day_02::Solution;
        use crate::shared::{Day as _, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::USize(727), (Solution {}).part_2());
        }
    }
}
