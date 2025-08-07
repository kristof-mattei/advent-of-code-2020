use super::{Board, Thing};

pub(super) fn flip_board_part_1(board: &mut Board) -> bool {
    let mut any_cell_changed = false;

    for row_index in 0..board.number_of_rows {
        for col_index in 0..board.number_of_cols {
            let (cell, changed) = get_seat_next_state(board, row_index, col_index);

            board.v_next[row_index][col_index] = cell;

            if changed {
                any_cell_changed = true;
            }
        }
    }

    std::mem::swap(&mut board.v_next, &mut board.v_now);

    any_cell_changed
}
struct NeighborIteratorPart1<'a> {
    start_row_index: usize,
    start_col_index: usize,
    board: &'a Board,
    directions_to_go: Vec<Direction>,
}

impl<'a> NeighborIteratorPart1<'a> {
    fn new(board: &'a Board, row_index: usize, col_index: usize) -> Self {
        Self {
            board,
            start_row_index: row_index,
            start_col_index: col_index,
            directions_to_go: vec![
                Direction::LeftTop,
                Direction::Top,
                Direction::TopRight,
                Direction::Right,
                Direction::BottomRight,
                Direction::Bottom,
                Direction::BottomLeft,
                Direction::Left,
            ],
        }
    }
}

enum Direction {
    Left,
    LeftTop,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
}

impl Iterator for NeighborIteratorPart1<'_> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(direction) = self.directions_to_go.pop() {
            // find the first non-floor tile in the direction
            match direction {
                Direction::LeftTop => {
                    if let (Some(r_i), Some(c_i)) = (
                        self.start_row_index.checked_sub(1),
                        self.start_col_index.checked_sub(1),
                    ) && self.board.v_now[r_i][c_i] != Thing::Floor
                    {
                        return Some((r_i, c_i));
                    }
                },
                Direction::Top => {
                    if let Some(r_i) = self.start_row_index.checked_sub(1)
                        && self.board.v_now[r_i][self.start_col_index] != Thing::Floor
                    {
                        return Some((r_i, self.start_col_index));
                    }
                },
                Direction::TopRight => {
                    if let (Some(r_i), Some(c_i)) = (
                        self.start_row_index.checked_sub(1),
                        (self.start_col_index + 1 < self.board.number_of_cols)
                            .then_some(self.start_col_index + 1),
                    ) && self.board.v_now[r_i][c_i] != Thing::Floor
                    {
                        return Some((r_i, c_i));
                    }
                },
                Direction::Right => {
                    if let Some(c_i) = (self.start_col_index + 1 < self.board.number_of_cols)
                        .then_some(self.start_col_index + 1)
                        && self.board.v_now[self.start_row_index][c_i] != Thing::Floor
                    {
                        return Some((self.start_row_index, c_i));
                    }
                },

                Direction::BottomRight => {
                    if let (Some(r_i), Some(c_i)) = (
                        (self.start_row_index + 1 < self.board.number_of_rows)
                            .then_some(self.start_row_index + 1),
                        (self.start_col_index + 1 < self.board.number_of_cols)
                            .then_some(self.start_col_index + 1),
                    ) && self.board.v_now[r_i][c_i] != Thing::Floor
                    {
                        return Some((r_i, c_i));
                    }
                },
                Direction::Bottom => {
                    if let Some(r_i) = (self.start_row_index + 1 < self.board.number_of_rows)
                        .then_some(self.start_row_index + 1)
                        && self.board.v_now[r_i][self.start_col_index] != Thing::Floor
                    {
                        return Some((r_i, self.start_col_index));
                    }
                },
                Direction::BottomLeft => {
                    if let (Some(r_i), Some(c_i)) = (
                        (self.start_row_index + 1 < self.board.number_of_rows)
                            .then_some(self.start_row_index + 1),
                        self.start_col_index.checked_sub(1),
                    ) && self.board.v_now[r_i][c_i] != Thing::Floor
                    {
                        return Some((r_i, c_i));
                    }
                },
                Direction::Left => {
                    if let Some(c_i) = self.start_col_index.checked_sub(1)
                        && self.board.v_now[self.start_row_index][c_i] != Thing::Floor
                    {
                        return Some((self.start_row_index, c_i));
                    }
                },
            }
        }

        None
    }
}

fn get_seat_next_state(board: &Board, row_index: usize, col_index: usize) -> (Thing, bool) {
    match board
        .v_now
        .get(row_index)
        .and_then(|row| row.get(col_index))
    {
        Some(&Thing::Floor) => (Thing::Floor, false),
        Some(&Thing::EmptySeat) => {
            // empty seat with no occupied seats becomes occupied
            // meaning if at least one of the seats is occupied we remain empty
            for (n_row_index, n_col_index) in
                NeighborIteratorPart1::new(board, row_index, col_index)
            {
                if board.v_now[n_row_index][n_col_index] == Thing::OccupiedSeat {
                    return (Thing::EmptySeat, false);
                }
            }

            (Thing::OccupiedSeat, true)
        },
        Some(&Thing::OccupiedSeat) => {
            // occupied seat with >=4 neighbors occupied becomes empty

            let mut occupied = 0_u32;

            for (n_row_index, n_col_index) in
                NeighborIteratorPart1::new(board, row_index, col_index)
            {
                if board.v_now[n_row_index][n_col_index] == Thing::OccupiedSeat {
                    occupied += 1;
                }

                if occupied >= 4 {
                    return (Thing::EmptySeat, true);
                }
            }

            (Thing::OccupiedSeat, false)
        },
        None => {
            panic!("Whut?")
        },
    }
}
