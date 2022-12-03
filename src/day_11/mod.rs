use std::{collections::HashSet, fmt::Display};

use crate::shared::{Day, PartSolution};

fn get_neighbors(
    board: &[Vec<Thing>],
    row_index: usize,
    column_index: usize,
) -> HashSet<(usize, usize)> {
    let mut neighbors = HashSet::new();

    let rows = board.len();
    let columns = board.get(0).map(Vec::len).unwrap_or_default();

    // clockwise:
    // left up
    // up
    // right up
    // right
    // right down
    // bottom
    // left down
    // left

    let can_go_left = column_index > 0;
    let can_go_up = row_index > 0;

    let can_go_right = column_index + 1 < columns;
    let can_go_down = row_index + 1 < rows;

    // left up
    if can_go_left && can_go_up {
        neighbors.insert((row_index - 1, column_index - 1));
    }

    // up
    if can_go_up {
        neighbors.insert((row_index - 1, column_index));
    }

    // right up
    if can_go_up && can_go_right {
        neighbors.insert((row_index - 1, column_index + 1));
    }

    // right
    if can_go_right {
        neighbors.insert((row_index, column_index + 1));
    }

    // right down
    if can_go_down && can_go_right {
        neighbors.insert((row_index + 1, column_index + 1));
    }

    // down
    if can_go_down {
        neighbors.insert((row_index + 1, column_index));
    }

    // left down
    if can_go_down && can_go_left {
        neighbors.insert((row_index + 1, column_index - 1));
    }

    // left
    if can_go_left {
        neighbors.insert((row_index, column_index - 1));
    }

    neighbors
}

fn get_seat_next_state(board: &[Vec<Thing>], row_index: usize, col_index: usize) -> Thing {
    match board.get(row_index).and_then(|row| row.get(col_index)) {
        Some(Thing::Floor) => Thing::Floor,
        Some(Thing::EmptySeat) => {
            // empty seat with no occupied seats becomes occupied
            // meaning if at least one of the seats is occupied we remain empty
            for (n_row_index, n_col_index) in get_neighbors(board, row_index, col_index) {
                if board[n_row_index][n_col_index] == Thing::OccupiedSeat {
                    return Thing::EmptySeat;
                }
            }

            Thing::OccupiedSeat
        },
        Some(Thing::OccupiedSeat) => {
            // occupied seat with >=4 neighbors occupied becomes empty

            let mut occupied = 0;

            for (n_row_index, n_col_index) in get_neighbors(board, row_index, col_index) {
                if board[n_row_index][n_col_index] == Thing::OccupiedSeat {
                    occupied += 1;
                }

                if occupied >= 4 {
                    return Thing::EmptySeat;
                }
            }

            Thing::OccupiedSeat
        },
        None => {
            panic!("Whut?")
        },
    }
}

fn parse_lines(lines: &[&str]) -> Vec<Vec<Thing>> {
    let mut board = Vec::new();
    for line in lines {
        let mut row = Vec::new();

        for char in line.chars() {
            row.push(char.into());
        }

        board.push(row);
    }

    board
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

fn flip_board(board: &[Vec<Thing>]) -> Vec<Vec<Thing>> {
    let mut new_board = vec![vec![Thing::Floor; board[0].len()]; board.len()];

    for row_index in 0..board.len() {
        for col_index in 0..board[row_index].len() {
            new_board[row_index][col_index] = get_seat_next_state(board, row_index, col_index);
        }
    }

    new_board
}

fn flip_board_until_stable(mut board: Vec<Vec<Thing>>) -> usize {
    let mut stable = false;
    while !stable {
        let new_board = flip_board(&board);

        if new_board == board {
            stable = true;
        }

        board = new_board;
    }

    return board
        .iter()
        .map(|row| row.iter().filter(|&&v| v == Thing::OccupiedSeat).count())
        .sum();
}

fn pretty_print(board: &[Vec<Thing>]) -> String {
    let mut lines = Vec::new();
    for row in board {
        let mut line = Vec::<char>::new();

        for value in row {
            line.push((*value).into());
        }

        lines.push(line.iter().collect::<String>());
    }

    lines.join("\n")
}
pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let board = parse_lines(&lines);

        let answer = flip_board_until_stable(board);

        PartSolution::USize(answer)
    }

    fn part_2(&self) -> PartSolution {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    fn get_example() -> Vec<&'static str> {
        include_str!("example.txt")
            .lines()
            .map(Into::into)
            .collect()
    }

    mod part_1 {
        use super::super::*;
        use crate::{
            day_11::test::get_example,
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U32(12855), (Solution {}).part_1());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let board = parse_lines(&lines);

            let new_board = flip_board(&board);

            println!("{}", pretty_print(&new_board));

            let expected = vec![
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

            assert_eq!(expected, pretty_print(&new_board));

            // let score = calculate_score_part_1(rounds);

            // assert_eq!(score, 15);
        }
    }

    mod part_2 {
        use crate::{
            day_02::Solution,
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U32(13726), (Solution {}).part_2());
        }

        #[test]
        fn example() {}
    }
}
