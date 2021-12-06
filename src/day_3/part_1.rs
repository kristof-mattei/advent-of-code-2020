fn descent_and_go_right(lines: &[String], row: usize, col: usize, mut trees: u32) -> u32 {
    match lines.get(row) {
        Some(line) => {
            if line.chars().nth(col) == Some('#') {
                trees += 1;
            }

            descent_and_go_right(lines, row + 1, (col + 3) % line.len(), trees)
        }
        None => trees,
    }
}

// https://adventofcode.com/2020/day/3
pub fn find_solution() -> u32 {
    let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

    descent_and_go_right(&lines, 0, 0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(191, find_solution());
    }
}
