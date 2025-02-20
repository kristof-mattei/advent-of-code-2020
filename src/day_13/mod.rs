use crate::shared::{Day, PartSolution};

fn parse_lines_part_1(lines: &[&str]) -> (usize, Vec<usize>) {
    let time = lines[0].parse::<usize>().unwrap();

    let buses = lines[1]
        .split(',')
        .filter(|&v| (v != "x"))
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    (time, buses)
}

fn parse_lines_part_2(lines: &[&str]) -> Vec<(usize, usize)> {
    let buses = lines[1]
        .split(',')
        .enumerate()
        .filter(|&(_, v)| (v != "x"))
        .map(|(i, v)| (i, v.parse::<usize>().unwrap()))
        .collect::<Vec<_>>();

    buses
}
fn find_closest(time: usize, buses: Vec<usize>) -> usize {
    let mut lowest_wait_time = usize::MAX;
    let mut lowest_wait_time_bus = 0;
    for bus in buses {
        let wait_time = bus - (time % bus);

        if wait_time < lowest_wait_time {
            lowest_wait_time = wait_time;
            lowest_wait_time_bus = bus;
        }
    }

    lowest_wait_time * lowest_wait_time_bus
}

fn find_one_minute_apart(mut buses: Vec<(usize, usize)>) -> usize {
    buses.sort_by_key(|b| b.0);

    // example starts with 7
    let mut step_size = buses.first().unwrap().1;
    let mut time = 0;

    buses.iter().skip(1).for_each(|(offset, bus)| {
        while (time + offset) % bus != 0 {
            time += step_size;
        }

        step_size *= bus;
    });

    time
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let (time, buses) = parse_lines_part_1(&lines);

        let score = find_closest(time, buses);

        score.into()
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let buses = parse_lines_part_2(&lines);

        let score = find_one_minute_apart(buses);

        score.into()
    }
}

#[cfg(test)]
mod test {
    fn get_example() -> Vec<&'static str> {
        include_str!("example.txt").lines().collect()
    }

    mod part_1 {
        use crate::day_13::test::get_example;
        use crate::day_13::{Solution, find_closest, parse_lines_part_1};
        use crate::shared::{Day, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::USize(119), (Solution {}).part_1());
        }

        #[test]
        fn example() {
            let lines: Vec<&str> = get_example();

            let (time, buses) = parse_lines_part_1(&lines);

            let score = find_closest(time, buses);

            assert_eq!(score, 295);
        }
    }

    mod part_2 {
        use crate::day_13::test::get_example;
        use crate::day_13::{Solution, find_one_minute_apart, parse_lines_part_2};
        use crate::shared::{Day, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::USize(1_106_724_616_194_525),
                (Solution {}).part_2()
            );
        }

        #[test]
        fn example() {
            let lines: Vec<&str> = get_example();

            let buses = parse_lines_part_2(&lines);

            let score = find_one_minute_apart(buses);

            assert_eq!(score, 1_068_781);
        }
    }
}
