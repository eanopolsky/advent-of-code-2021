use clap::{App, Arg};
use std::fs;

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

    match (day, part) {
        (1, 1) => println!(
            "The solution to day {} part {} is {}",
            day,
            part,
            solve_day_1_part_1(puzzle_input)
        ),
        _ => println!(
            "The solution for day {} part {} is not implemented",
            day, part
        ),
    }
}

fn solve_day_1_part_1(puzzle_input: String) -> u32 {
    let mut sonar_sweep_report = puzzle_input.split("\n");
    let mut increases: u32 = 0;
    let mut previous_reading: u32 = sonar_sweep_report
        .next()
        .expect("Failed to find the first reading in the sonar sweep report.")
        .parse::<u32>()
        .expect("Failed to parse the first reading in the sonar sweep report.");

    for current_reading in sonar_sweep_report {
        if current_reading == "" {
            break;
        }
        let current_reading = current_reading
            .parse::<u32>()
            .expect("Failed to parse subsequent reading in the sonar sweep report.");
        if current_reading > previous_reading {
            increases += 1;
        }
        previous_reading = current_reading;
    }
    return increases;
}
