use crate::shared::{Day, PartSolution};

fn parse_lines(lines: &[&str]) -> Vec<Operation> {
    let mut result = Vec::new();

    for line in lines {
        let (op, value) = line.split_at(1);

        let v: i32 = value.parse().unwrap();

        let operation = match op.as_bytes()[0] {
            b'N' => Operation::MoveNorth(v),
            b'S' => Operation::MoveSouth(v),
            b'E' => Operation::MoveEast(v),
            b'W' => Operation::MoveWest(v),
            b'L' => Operation::RotateLeft(v),
            b'R' => Operation::RotateRight(v),
            b'F' => Operation::MoveForward(v),
            _ => panic!("AT THE DISCOTEQUE"),
        };

        result.push(operation);
    }

    result
}

enum Operation {
    MoveNorth(i32),
    MoveSouth(i32),
    MoveEast(i32),
    MoveWest(i32),
    RotateLeft(i32),
    RotateRight(i32),
    MoveForward(i32),
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::MoveNorth(v) => write!(f, "N{}", v),
            Operation::MoveSouth(v) => write!(f, "S{}", v),
            Operation::MoveEast(v) => write!(f, "E{}", v),
            Operation::MoveWest(v) => write!(f, "W{}", v),
            Operation::RotateLeft(v) => write!(f, "L{}", v),
            Operation::RotateRight(v) => write!(f, "R{}", v),
            Operation::MoveForward(v) => write!(f, "F{}", v),
        }
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn rotate(&mut self, how_much: i32) {
        let new = match self {
            Direction::North => how_much,
            Direction::South => 180 + how_much,
            Direction::East => 90 + how_much,
            Direction::West => 270 + how_much,
        };

        match new.rem_euclid(360) {
            0 => *self = Direction::North,
            90 => *self = Direction::East,
            180 => *self = Direction::South,
            270 => *self = Direction::West,
            _ => {
                panic!(
                    "Where we're going, we don't need roads... I mean, what? Where ARE we going?"
                )
            },
        }
    }
}

struct Ship {
    facing: Direction,
    location_x: i32,
    location_y: i32,
}

impl Ship {
    fn new() -> Self {
        Self {
            facing: Direction::East,
            location_x: 0,
            location_y: 0,
        }
    }

    fn process_operation(&mut self, operation: &Operation) {
        match operation {
            Operation::MoveNorth(v) => {
                self.location_y -= v;
            },
            Operation::MoveSouth(v) => {
                self.location_y += v;
            },
            Operation::MoveEast(v) => {
                self.location_x += v;
            },
            Operation::MoveWest(v) => {
                self.location_x -= v;
            },
            Operation::RotateLeft(l) => self.facing.rotate(-*l),
            Operation::RotateRight(r) => self.facing.rotate(*r),
            Operation::MoveForward(v) => match self.facing {
                Direction::North => self.process_operation(&Operation::MoveNorth(*v)),
                Direction::South => self.process_operation(&Operation::MoveSouth(*v)),
                Direction::East => self.process_operation(&Operation::MoveEast(*v)),
                Direction::West => self.process_operation(&Operation::MoveWest(*v)),
            },
        }
    }
}

struct ShipAndWaypoint {
    ship_location_x: i32,
    ship_location_y: i32,
    waypoint_location_x: i32,
    waypoint_location_y: i32,
}

impl ShipAndWaypoint {
    fn new() -> Self {
        Self {
            ship_location_x: 0,
            ship_location_y: 0,
            waypoint_location_x: 10,
            waypoint_location_y: -1,
        }
    }

    fn process_operation_part_2(&mut self, operation: &Operation) {
        match operation {
            Operation::MoveNorth(v) => {
                self.waypoint_location_y -= v;
            },
            Operation::MoveSouth(v) => {
                self.waypoint_location_y += v;
            },
            Operation::MoveEast(v) => {
                self.waypoint_location_x += v;
            },
            Operation::MoveWest(v) => {
                self.waypoint_location_x -= v;
            },
            Operation::RotateLeft(l) => {
                // CCW
                self.process_operation_part_2(&Operation::RotateRight(360 - *l));
            },
            Operation::RotateRight(r) => {
                // CW
                match r {
                    90 => {
                        self.waypoint_location_y *= -1;

                        std::mem::swap(
                            &mut self.waypoint_location_x,
                            &mut self.waypoint_location_y,
                        );
                    },
                    180 => {
                        self.waypoint_location_x *= -1;
                        self.waypoint_location_y *= -1;
                    },
                    270 => {
                        self.waypoint_location_x *= -1;

                        std::mem::swap(
                            &mut self.waypoint_location_x,
                            &mut self.waypoint_location_y,
                        );
                    },
                    _ => {
                        panic!("You spin me right round baby right round")
                    },
                }
            },
            Operation::MoveForward(v) => {
                self.ship_location_x += self.waypoint_location_x * *v;
                self.ship_location_y += self.waypoint_location_y * *v;
            },
        }
    }
}

fn pilot(operations: Vec<Operation>) -> i32 {
    let mut ship = Ship::new();

    for operation in operations {
        ship.process_operation(&operation);
    }

    ship.location_x.abs() + ship.location_y.abs()
}

fn pilot_part_2(operations: Vec<Operation>) -> i32 {
    let mut ship_and_waypoint = ShipAndWaypoint::new();

    for operation in operations {
        ship_and_waypoint.process_operation_part_2(&operation);
    }

    ship_and_waypoint.ship_location_x.abs() + ship_and_waypoint.ship_location_y.abs()
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let operations = parse_lines(&lines);

        let score = pilot(operations);

        score.into()
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let parsed = parse_lines(&lines);

        let score = pilot_part_2(parsed);

        score.into()
    }
}

#[cfg(test)]
mod test {
    fn get_example() -> Vec<&'static str> {
        include_str!("example.txt").lines().collect()
    }

    mod part_1 {
        use crate::day_12::test::get_example;
        use crate::day_12::{Solution, parse_lines, pilot};
        use crate::shared::{Day, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::I32(2847), (Solution {}).part_1());
        }

        #[test]
        fn example() {
            let lines: Vec<&str> = get_example();

            let operations = parse_lines(&lines);

            let score = pilot(operations);

            assert_eq!(score, 25);
        }
    }

    mod part_2 {
        use crate::day_12::test::get_example;
        use crate::day_12::{Solution, parse_lines, pilot_part_2};
        use crate::shared::{Day, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::I32(29839), (Solution {}).part_2());
        }

        #[test]
        fn example() {
            let lines: Vec<&str> = get_example();

            let operations = parse_lines(&lines);

            let score = pilot_part_2(operations);

            assert_eq!(score, 286);
        }
    }
}
