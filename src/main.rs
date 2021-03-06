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
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day21;
mod day22;

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

    let solution: String = match (day, part) {
        (1, 1) => day01::solve_part_1(puzzle_input).to_string(),
        (1, 2) => day01::solve_part_2(puzzle_input).to_string(),
        (2, 1) => day02::solve_part_1(puzzle_input).to_string(),
        (2, 2) => day02::solve_part_2(puzzle_input).to_string(),
        (3, 1) => day03::solve_part_1(puzzle_input).to_string(),
        (3, 2) => day03::solve_part_2(puzzle_input).to_string(),
        (4, 1) => day04::solve_part_1(puzzle_input).to_string(),
        (4, 2) => day04::solve_part_2(puzzle_input).to_string(),
        (5, 1) => day05::solve_part_1(puzzle_input).to_string(),
        (5, 2) => day05::solve_part_2(puzzle_input).to_string(),
        (6, 1) => day06::solve_part_1(puzzle_input).to_string(),
        (6, 2) => day06::solve_part_2(puzzle_input).to_string(),
        (7, 1) => day07::solve_part_1(puzzle_input).to_string(),
        (7, 2) => day07::solve_part_2(puzzle_input).to_string(),
        (8, 1) => day08::solve_part_1(puzzle_input).to_string(),
        (8, 2) => day08::solve_part_2(puzzle_input).to_string(),
        (9, 1) => day09::solve_part_1(puzzle_input).to_string(),
        (9, 2) => day09::solve_part_2(puzzle_input).to_string(),
        (10, 1) => day10::solve_part_1(puzzle_input).to_string(),
        (10, 2) => day10::solve_part_2(puzzle_input).to_string(),
        (11, 1) => day11::solve_part_1(puzzle_input).to_string(),
        (11, 2) => day11::solve_part_2(puzzle_input).to_string(),
        (12, 1) => day12::solve_part_1(puzzle_input).to_string(),
        (12, 2) => day12::solve_part_2(puzzle_input).to_string(),
        (13, 1) => day13::solve_part_1(puzzle_input).to_string(),
        (13, 2) => day13::solve_part_2(puzzle_input),
        (14, 1) => day14::solve_part_1(puzzle_input),
        (14, 2) => day14::solve_part_2(puzzle_input),
        (15, 1) => day15::solve_part_1(puzzle_input),
        (15, 2) => day15::solve_part_2(puzzle_input),
        (16, 1) => day16::solve_part_1(puzzle_input),
        (16, 2) => day16::solve_part_2(puzzle_input),
        (17, 1) => day17::solve_part_1(puzzle_input),
        (17, 2) => day17::solve_part_2(puzzle_input),
        (21, 1) => day21::solve_part_1(puzzle_input),
        (21, 2) => day21::solve_part_2(puzzle_input),
        (22, 1) => day22::solve_part_1(puzzle_input),
        (22, 2) => day22::solve_part_2(puzzle_input),
        _ => panic!(
            "The solution for day {} part {} is not implemented",
            day, part
        ),
    };
    println!("The solution to day {} part {} is {}", day, part, solution);
}
