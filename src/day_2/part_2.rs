use crate::utils::read_file;

struct RuleWithPassword {
    first_position: usize,
    second_position: usize,
    character: char,
    password: String,
}

impl RuleWithPassword {
    pub fn is_valid(&self) -> bool {
        let p1_valid = self.password.chars().nth(self.first_position - 1) == Some(self.character);
        let p2_valid = self.password.chars().nth(self.second_position - 1) == Some(self.character);

        p1_valid ^ p2_valid
    }
}

fn parse_line(line: &str) -> RuleWithPassword {
    // grammar:
    // <min>-<max> char: <password>
    let pieces: Vec<&str> = line.split(' ').collect();

    let first_second_vec: Vec<usize> = pieces[0]
        .split('-')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let char_with_colon_piece = pieces[1];
    let password = pieces[2];

    RuleWithPassword {
        first_position: *first_second_vec.get(0).unwrap(),
        second_position: *first_second_vec.get(1).unwrap(),
        character: char_with_colon_piece.chars().next().unwrap(),
        password: password.into(),
    }
}

// https://adventofcode.com/2020/day/2
pub fn find_solution() -> u32 {
    let lines = include_str!("input.txt");

    let valid_passwords = lines
        .lines()
        .map(parse_line)
        .filter(RuleWithPassword::is_valid)
        .count();

    Ok(valid_passwords as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(727, find_solution());
    }
}
