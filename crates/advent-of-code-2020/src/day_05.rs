use crate::shared::{Day, PartSolution};

fn parse_seat(seat_line: &str) -> (u32, u32) {
    const LOWER_BITS_ROW: u32 = 0;
    const UPPER_BITS_ROW: u32 = LOWER_BITS_ROW + 6;
    const LOWER_BITS_COL: u32 = UPPER_BITS_ROW + 1;
    const UPPER_BITS_COL: u32 = LOWER_BITS_COL + 2;

    let mut pieces = seat_line.chars();

    let mut row: u32 = 0;
    let mut column: u32 = 0;

    for i in 0..=UPPER_BITS_ROW {
        match pieces.next().unwrap() {
            'F' => (),
            'B' => row |= 0b1 << (UPPER_BITS_ROW - i),
            _ => panic!("Not F or B"),
        }
    }

    for i in 7..=UPPER_BITS_COL {
        match pieces.next().unwrap() {
            'L' => (),
            'R' => column |= 0b1 << (UPPER_BITS_COL - i),
            _ => panic!("Not L or R"),
        }
    }

    (row, column)
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("day_05/input.txt")
            .lines()
            .map(Into::into)
            .collect();

        let max = lines
            .iter()
            .map(|l| parse_seat(l))
            .map(|(r, c)| r * 8 + c)
            .max()
            .unwrap();

        PartSolution::U32(max)
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("day_05/input.txt")
            .lines()
            .map(Into::into)
            .collect();

        let mut items: Vec<u32> = lines
            .iter()
            .map(|l| parse_seat(l))
            .map(|(r, c)| r * 8 + c)
            .collect();

        items.sort_unstable();

        let mut previous = None;

        // let's find the gap
        for i in items {
            match previous {
                Some(p) if (i - p) > 1 => {
                    return PartSolution::U32(p + 1);
                },
                _ => previous = Some(i),
            }
        }

        panic!("Couldn't find item")
    }
}

#[cfg(test)]
mod tests {
    mod part_1 {
        use pretty_assertions::assert_eq;

        use crate::day_05::{Solution, parse_seat};
        use crate::shared::{Day as _, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U32(951), (Solution {}).part_1());
        }

        #[test]
        fn seat_id_1() {
            let seat = "FBFBBFFRLR";

            assert_eq!(parse_seat(seat), (44, 5));
        }

        #[test]
        fn seat_id_2() {
            let seat = "BFFFBBFRRR";

            assert_eq!(parse_seat(seat), (70, 7));
        }

        #[test]
        fn seat_id_3() {
            let seat = "FFFBBBFRRR";

            assert_eq!(parse_seat(seat), (14, 7));
        }
        #[test]
        fn seat_id_4() {
            let seat = "BBFFBBFRLL";

            assert_eq!(parse_seat(seat), (102, 4));
        }
    }

    #[cfg(test)]
    mod part_2 {
        use pretty_assertions::assert_eq;

        use crate::day_05::{Solution, parse_seat};
        use crate::shared::{Day as _, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U32(653), (Solution {}).part_2());
        }

        #[test]
        fn seat_id_1() {
            let seat = "FBFBBFFRLR";

            assert_eq!(parse_seat(seat), (44, 5));
        }

        #[test]
        fn seat_id_2() {
            let seat = "BFFFBBFRRR";

            assert_eq!(parse_seat(seat), (70, 7));
        }

        #[test]
        fn seat_id_3() {
            let seat = "FFFBBBFRRR";

            assert_eq!(parse_seat(seat), (14, 7));
        }
        #[test]
        fn seat_id_4() {
            let seat = "BBFFBBFRLL";

            assert_eq!(parse_seat(seat), (102, 4));
        }
    }
}
