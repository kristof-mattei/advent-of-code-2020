use std::fmt::Display;

use self::part_1::flip_board_part_1;
use self::part_2::flip_board_part_2;
use crate::shared::{Day, PartSolution};

mod part_1;
mod part_2;

#[derive(PartialEq, Eq)]
struct Board {
    number_of_rows: usize,
    number_of_cols: usize,
    v_now: Vec<Vec<Thing>>,
    v_next: Vec<Vec<Thing>>,
}

impl Board {
    fn new(cells: Vec<Vec<Thing>>) -> Self {
        Self {
            number_of_rows: cells.len(),
            number_of_cols: cells.first().map(Vec::len).unwrap_or_default(),
            v_now: cells.clone(),
            v_next: cells,
        }
    }
}

fn parse_lines(lines: &[&str]) -> Board {
    let mut cells = Vec::new();
    for line in lines {
        let mut row = Vec::new();

        for char in line.chars() {
            row.push(char.into());
        }

        cells.push(row);
    }

    Board::new(cells)
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Thing {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl Display for Thing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Thing::Floor => write!(f, "."),
            Thing::EmptySeat => write!(f, "L"),
            Thing::OccupiedSeat => write!(f, "#"),
        }
    }
}

impl From<Thing> for char {
    fn from(t: Thing) -> Self {
        match t {
            Thing::Floor => '.',
            Thing::EmptySeat => 'L',
            Thing::OccupiedSeat => '#',
        }
    }
}
impl From<char> for Thing {
    fn from(c: char) -> Self {
        match c {
            '.' => Thing::Floor,
            'L' => Thing::EmptySeat,
            '#' => Thing::OccupiedSeat,
            _ => unreachable!(),
        }
    }
}

fn count_occupied(board: &Board) -> usize {
    board
        .v_now
        .iter()
        .map(|row| row.iter().filter(|&&v| v == Thing::OccupiedSeat).count())
        .sum()
}

fn flip_board_until_stable_part_1(mut board: Board) -> usize {
    while flip_board_part_1(&mut board) {}

    count_occupied(&board)
}

fn flip_board_until_stable_part_2(mut board: Board) -> usize {
    while flip_board_part_2(&mut board) {}

    count_occupied(&board)
}
pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let board = parse_lines(&lines);

        let answer = flip_board_until_stable_part_1(board);

        PartSolution::USize(answer)
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let board = parse_lines(&lines);

        let answer = flip_board_until_stable_part_2(board);

        PartSolution::USize(answer)
    }
}

#[cfg(test)]
mod test {
    use super::Board;

    fn get_example() -> Vec<&'static str> {
        include_str!("example.txt").lines().collect()
    }

    fn pretty_print(board: &Board) -> String {
        let mut lines = Vec::new();
        for row in &board.v_now {
            let mut line = Vec::<char>::new();

            for value in row {
                line.push((*value).into());
            }

            lines.push(line.iter().collect::<String>());
        }

        lines.join("\n")
    }

    mod part_1 {
        use crate::day_11::part_1::flip_board_part_1;
        use crate::day_11::test::{get_example, pretty_print};
        use crate::day_11::{Solution, parse_lines};
        use crate::shared::{Day, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::USize(2406), (Solution {}).part_1());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let mut board = parse_lines(&lines);

            flip_board_part_1(&mut board);

            let expected = [
                "#.##.##.##",
                "#######.##",
                "#.#.#..#..",
                "####.##.##",
                "#.##.##.##",
                "#.#####.##",
                "..#.#.....",
                "##########",
                "#.######.#",
                "#.#####.##",
            ]
            .join("\n");

            assert_eq!(expected, pretty_print(&board));

            flip_board_part_1(&mut board);

            let expected = [
                "#.LL.L#.##",
                "#LLLLLL.L#",
                "L.L.L..L..",
                "#LLL.LL.L#",
                "#.LL.LL.LL",
                "#.LLLL#.##",
                "..L.L.....",
                "#LLLLLLLL#",
                "#.LLLLLL.L",
                "#.#LLLL.##",
            ]
            .join("\n");

            assert_eq!(expected, pretty_print(&board));
        }
    }

    mod part_2 {
        use crate::day_11::part_2::flip_board_part_2;
        use crate::day_11::test::{get_example, pretty_print};
        use crate::day_11::{Solution, parse_lines};
        use crate::shared::{Day, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::USize(2149), (Solution {}).part_2());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let mut board = parse_lines(&lines);

            flip_board_part_2(&mut board);

            let expected = [
                "#.##.##.##",
                "#######.##",
                "#.#.#..#..",
                "####.##.##",
                "#.##.##.##",
                "#.#####.##",
                "..#.#.....",
                "##########",
                "#.######.#",
                "#.#####.##",
            ]
            .join("\n");

            assert_eq!(expected, pretty_print(&board));

            flip_board_part_2(&mut board);

            // If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
            // If a seat is occupied (#) and five or more seats adjacent to it are also occupied, the seat becomes empty.
            // Otherwise, the seat's state does not change.

            let expected = [
                "#.LL.LL.L#",
                "#LLLLLL.LL",
                "L.L.L..L..",
                "LLLL.LL.LL",
                "L.LL.LL.LL",
                "L.LLLLL.LL",
                "..L.L.....",
                "LLLLLLLLL#",
                "#.LLLLLL.L",
                "#.LLLLL.L#",
            ]
            .join("\n");

            assert_eq!(expected, pretty_print(&board));
        }
    }
}
