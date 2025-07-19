use crate::shared::{Day, PartSolution};

fn descent_and_go_right(
    lines: &[String],
    row: usize,
    col: usize,
    mut trees: u32,
    down: usize,
    right: usize,
) -> u32 {
    match lines.get(row) {
        Some(line) => {
            if line.chars().nth(col) == Some('#') {
                trees += 1;
            }

            descent_and_go_right(
                lines,
                row + down,
                (col + right) % line.len(),
                trees,
                down,
                right,
            )
        },
        None => trees,
    }
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("day_03/input.txt")
            .lines()
            .map(Into::into)
            .collect();

        PartSolution::U32(descent_and_go_right(&lines, 0, 0, 0, 1, 3))
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("day_03/input.txt")
            .lines()
            .map(Into::into)
            .collect();

        let result1 = descent_and_go_right(&lines, 0, 0, 0, 1, 1);
        let result2 = descent_and_go_right(&lines, 0, 0, 0, 1, 3);
        let result3 = descent_and_go_right(&lines, 0, 0, 0, 1, 5);
        let result4 = descent_and_go_right(&lines, 0, 0, 0, 1, 7);
        let result5 = descent_and_go_right(&lines, 0, 0, 0, 2, 1);

        PartSolution::U32(result1 * result2 * result3 * result4 * result5)
    }
}

#[cfg(test)]
mod tests {
    mod part_1 {
        use crate::day_03::Solution;
        use crate::shared::{Day as _, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U32(191), (Solution {}).part_1());
        }
    }
    mod part_2 {
        use crate::day_03::Solution;
        use crate::shared::{Day as _, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U32(1_478_615_040), (Solution {}).part_2());
        }
    }
}
