#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

mod day_1;
mod day_10;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod shared;
mod utils;

fn print_answer(day: u32, part: u32, result: &str) {
    println!("Answer to Day {}, part {} is ... {}", day, part, result);
}

fn main() {
    let solutions = vec![
        day_1::part_1::find_solution().map(|r| r.to_string()),
        day_1::part_2::find_solution().map(|r| r.to_string()),
        day_2::part_1::find_solution().map(|r| r.to_string()),
        day_2::part_2::find_solution().map(|r| r.to_string()),
        day_3::part_1::find_solution().map(|r| r.to_string()),
        day_3::part_2::find_solution().map(|r| r.to_string()),
        day_4::part_1::find_solution().map(|r| r.to_string()),
        day_4::part_2::find_solution().map(|r| r.to_string()),
        day_5::part_1::find_solution().map(|r| r.to_string()),
        day_5::part_2::find_solution().map(|r| r.to_string()),
        day_6::part_1::find_solution().map(|r| r.to_string()),
        day_6::part_2::find_solution().map(|r| r.to_string()),
        day_7::part_1::find_solution().map(|r| r.to_string()),
        day_7::part_2::find_solution().map(|r| r.to_string()),
        day_8::part_1::find_solution().map(|r| r.to_string()),
        day_8::part_2::find_solution().map(|r| r.to_string()),
        day_9::part_1::find_solution().map(|r| r.to_string()),
        day_9::part_2::find_solution().map(|r| r.to_string()),
        day_10::part_1::find_solution().map(|r| r.to_string()),
        day_10::part_2::find_solution().map(|r| r.to_string()),
    ];

    let mut day: u32 = 1;

    for day_solution in solutions.windows(2) {
        print_answer(day, 1, &day_solution[0]);
        print_answer(day, 2, &day_solution[1]);

        day += 1;
    }
}
