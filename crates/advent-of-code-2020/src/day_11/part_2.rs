use super::{Board, Thing};

pub(super) fn flip_board_part_2(board: &mut Board) -> bool {
    let mut any_cell_changed = false;

    for row_index in 0..board.number_of_rows {
        for col_index in 0..board.number_of_cols {
            let (cell, changed) = get_seat_next_state_part_2(board, row_index, col_index);

            board.v_next[row_index][col_index] = cell;

            if changed {
                any_cell_changed = true;
            }
        }
    }

    std::mem::swap(&mut board.v_next, &mut board.v_now);

    any_cell_changed
}

struct NeighborIteratorPart2<'a> {
    start_row_index: usize,
    start_col_index: usize,
    board: &'a Board,
    directions_to_go: Vec<Direction>,
}

impl<'a> NeighborIteratorPart2<'a> {
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

impl Iterator for NeighborIteratorPart2<'_> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(direction) = self.directions_to_go.pop() {
            // find the first non-floor tile in the direction
            match direction {
                Direction::LeftTop => {
                    let mut row_index = self.start_row_index;
                    let mut col_index = self.start_col_index;

                    while let (Some(r_i), Some(c_i)) =
                        (row_index.checked_sub(1), col_index.checked_sub(1))
                    {
                        row_index = r_i;
                        col_index = c_i;

                        if self.board.v_now[row_index][col_index] != Thing::Floor {
                            return Some((row_index, col_index));
                        }
                    }
                },
                Direction::Top => {
                    let mut row_index = self.start_row_index;

                    while let Some(r_i) = row_index.checked_sub(1) {
                        row_index = r_i;

                        if self.board.v_now[row_index][self.start_col_index] != Thing::Floor {
                            return Some((row_index, self.start_col_index));
                        }
                    }
                },
                Direction::TopRight => {
                    let mut row_index = self.start_row_index;
                    let mut col_index = self.start_col_index;

                    while let (Some(r_i), Some(c_i)) = (
                        row_index.checked_sub(1),
                        (col_index + 1 < self.board.number_of_cols).then_some(col_index + 1),
                    ) {
                        row_index = r_i;
                        col_index = c_i;

                        if self.board.v_now[row_index][col_index] != Thing::Floor {
                            return Some((row_index, col_index));
                        }
                    }
                },
                Direction::Right => {
                    let mut col_index = self.start_col_index;

                    while let Some(c_i) =
                        (col_index + 1 < self.board.number_of_cols).then_some(col_index + 1)
                    {
                        col_index = c_i;

                        if self.board.v_now[self.start_row_index][col_index] != Thing::Floor {
                            return Some((self.start_row_index, col_index));
                        }
                    }
                },

                Direction::BottomRight => {
                    let mut row_index = self.start_row_index;
                    let mut col_index = self.start_col_index;

                    while let (Some(r_i), Some(c_i)) = (
                        (row_index + 1 < self.board.number_of_rows).then_some(row_index + 1),
                        (col_index + 1 < self.board.number_of_cols).then_some(col_index + 1),
                    ) {
                        row_index = r_i;
                        col_index = c_i;

                        if self.board.v_now[row_index][col_index] != Thing::Floor {
                            return Some((row_index, col_index));
                        }
                    }
                },
                Direction::Bottom => {
                    let mut row_index = self.start_row_index;

                    while let Some(r_i) =
                        (row_index + 1 < self.board.number_of_rows).then_some(row_index + 1)
                    {
                        row_index = r_i;

                        if self.board.v_now[row_index][self.start_col_index] != Thing::Floor {
                            return Some((row_index, self.start_col_index));
                        }
                    }
                },
                Direction::BottomLeft => {
                    let mut row_index = self.start_row_index;
                    let mut col_index = self.start_col_index;

                    while let (Some(r_i), Some(c_i)) = (
                        (row_index + 1 < self.board.number_of_rows).then_some(row_index + 1),
                        col_index.checked_sub(1),
                    ) {
                        row_index = r_i;
                        col_index = c_i;

                        if self.board.v_now[row_index][col_index] != Thing::Floor {
                            return Some((row_index, col_index));
                        }
                    }
                },
                Direction::Left => {
                    let mut col_index = self.start_col_index;

                    while let Some(c_i) = col_index.checked_sub(1) {
                        col_index = c_i;

                        if self.board.v_now[self.start_row_index][col_index] != Thing::Floor {
                            return Some((self.start_row_index, col_index));
                        }
                    }
                },
            }
        }

        None
    }
}

fn get_seat_next_state_part_2(board: &Board, row_index: usize, col_index: usize) -> (Thing, bool) {
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
                NeighborIteratorPart2::new(board, row_index, col_index)
            {
                if board.v_now[n_row_index][n_col_index] == Thing::OccupiedSeat {
                    return (Thing::EmptySeat, false);
                }
            }

            (Thing::OccupiedSeat, true)
        },
        Some(&Thing::OccupiedSeat) => {
            // occupied seat with >=5 neighbors occupied becomes empty

            let mut occupied: u32 = 0;

            for (n_row_index, n_col_index) in
                NeighborIteratorPart2::new(board, row_index, col_index)
            {
                if board.v_now[n_row_index][n_col_index] == Thing::OccupiedSeat {
                    occupied += 1;
                }

                if occupied >= 5 {
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
