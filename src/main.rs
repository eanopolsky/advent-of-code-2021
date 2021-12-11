use clap::{App, Arg};
use std::fs;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

#[cfg(test)]
mod test_helpers;

fn main() {
    let matches = App::new("Advent of Code 2021 Solver")
        .version("1.0")
        .author("Eric Anopolsky <eric@anopolsky.com>")
        .about("Solves challenges from Advent of Code 2021")
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .value_name("DAY_NUMBER")
                .help("The day of the problem to be solved")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("part")
                .short("p")
                .long("part")
                .value_name("PROBLEM_PART")
                .help("The part of the problem to be solved (1 or 2)")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Path to a file containing the problem input")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let day: u8 = matches
        .value_of("day")
        .expect("A day must be provided.")
        .parse::<u8>()
        .expect("A day must be specified as an integer.");
    let part: u8 = matches
        .value_of("part")
        .expect("A part must be provided.")
        .parse::<u8>()
        .expect("A part must be specified as an integer (1 or 2)");
    let input_file_path: &str = matches
        .value_of("input")
        .expect("The path to an input file must be specified.");

    let puzzle_input: String = fs::read_to_string(input_file_path)
        .expect("Something went wrong while reading the input file.");

    let solution: u64 = match (day, part) {
        (1, 1) => day01::solve_part_1(puzzle_input).into(),
        (1, 2) => day01::solve_part_2(puzzle_input).into(),
        (2, 1) => day02::solve_part_1(puzzle_input).into(),
        (2, 2) => day02::solve_part_2(puzzle_input).into(),
        (3, 1) => day03::solve_part_1(puzzle_input).into(),
        (3, 2) => day03::solve_part_2(puzzle_input).into(),
        (4, 1) => day04::solve_part_1(puzzle_input).into(),
        (4, 2) => day04::solve_part_2(puzzle_input).into(),
        (5, 1) => day05::solve_part_1(puzzle_input).into(),
        (5, 2) => day05::solve_part_2(puzzle_input).into(),
        (6, 1) => day06::solve_part_1(puzzle_input),
        (6, 2) => day06::solve_part_2(puzzle_input),
        (7, 1) => day07::solve_part_1(puzzle_input),
        (7, 2) => day07::solve_part_2(puzzle_input),
        (8, 1) => day08::solve_part_1(puzzle_input),
        (8, 2) => day08::solve_part_2(puzzle_input),
        (9, 1) => day09::solve_part_1(puzzle_input),
        (9, 2) => day09::solve_part_2(puzzle_input),
        (10, 1) => day10::solve_part_1(puzzle_input),
        _ => panic!(
            "The solution for day {} part {} is not implemented",
            day, part
        ),
    };
    println!("The solution to day {} part {} is {}", day, part, solution);
}
