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

    let solution: u32 = match (day, part) {
        (1, 1) => solve_day_1_part_1(puzzle_input),
        (1, 2) => solve_day_1_part_2(puzzle_input),
        (2, 1) => solve_day_2_part_1(puzzle_input),
        (2, 2) => solve_day_2_part_2(puzzle_input),
        (3, 1) => solve_day_3_part_1(puzzle_input),
        _ => panic!(
            "The solution for day {} part {} is not implemented",
            day, part
        ),
    };
    println!("The solution to day {} part {} is {}", day, part, solution);
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

fn solve_day_1_part_2(puzzle_input: String) -> u32 {
    let mut sonar_sweep_report = puzzle_input
        .split("\n")
        .filter_map(|reading: &str| -> Option<u32> { reading.parse::<u32>().ok() });

    let mut increases: u32 = 0;

    let mut previous_window = [
        sonar_sweep_report.next().unwrap(),
        sonar_sweep_report.next().unwrap(),
        sonar_sweep_report.next().unwrap(),
    ];

    for new_reading in sonar_sweep_report {
        let current_window = [previous_window[1], previous_window[2], new_reading];
        if current_window.iter().sum::<u32>() > previous_window.iter().sum::<u32>() {
            increases += 1;
        }
        previous_window = current_window;
    }

    increases
}

enum SubMovement {
    Forward(u32),
    Down(u32),
    Up(u32),
}

fn parse_sub_movements(puzzle_input: String) -> Vec<SubMovement> {
    puzzle_input
        .split("\n")
        .filter_map(|movement: &str| -> Option<SubMovement> {
            if movement == "" {
                return None;
            }
            let mut components = movement.split(" ");
            let direction = components
                .next()
                .expect("Failed to find direction in component");
            let magnitude = components
                .next()
                .expect("Failed to find magnitude in component")
                .parse::<u32>()
                .expect("Failed to parse numeric magnitude");
            match direction {
                "forward" => Some(SubMovement::Forward(magnitude)),
                "down" => Some(SubMovement::Down(magnitude)),
                "up" => Some(SubMovement::Up(magnitude)),
                _ => panic!("Unsupported direction in component"),
            }
        })
        .collect()
}

fn solve_day_2_part_1(puzzle_input: String) -> u32 {
    let sub_directions = parse_sub_movements(puzzle_input);
    let sub_directions = sub_directions.iter();

    let mut depth: u32 = 0;
    let mut horizontal_position: u32 = 0;
    for sub_direction in sub_directions {
        match sub_direction {
            SubMovement::Forward(n) => {
                horizontal_position += n;
            }
            SubMovement::Down(n) => {
                depth += n;
            }
            SubMovement::Up(n) => {
                depth -= n;
            }
        }
    }

    depth * horizontal_position
}

fn solve_day_2_part_2(puzzle_input: String) -> u32 {
    let sub_directions = parse_sub_movements(puzzle_input);
    let sub_directions = sub_directions.iter();

    let mut aim: i32 = 0;
    let mut depth: u32 = 0;
    let mut horizontal_position: u32 = 0;
    for sub_direction in sub_directions {
        match sub_direction {
            SubMovement::Forward(n) => {
                horizontal_position += n;
                depth = ((depth as i32) + aim * *n as i32) as u32;
            }
            SubMovement::Down(n) => {
                aim += *n as i32;
            }
            SubMovement::Up(n) => {
                aim -= *n as i32;
            }
        }
    }

    depth * horizontal_position
}

fn solve_day_3_part_1(puzzle_input: String) -> u32 {
    let diagnostic_report =
        puzzle_input
            .split("\n")
            .filter_map(|report_number: &str| -> Option<&str> {
                if report_number == "" {
                    return None;
                }
                Some(report_number)
            });
    let mut report_size = 0;
    let mut bit_counts: [u32; 12] = [0; 12];
    for report_entry in diagnostic_report {
        report_size += 1;
        for (i, report_char) in report_entry.chars().enumerate() {
            if report_char == '1' {
                bit_counts[i] += 1;
            }
        }
    }
    println!(
        "report_size = {}, bit_counts = {:?}",
        report_size, bit_counts
    );
    let mut gamma_rate = 0;
    for count in bit_counts {
        if count * 2 > report_size {
            gamma_rate += 1;
        }
        gamma_rate *= 2;
    }
    gamma_rate /= 2;
    let epsilon_rate = 2_u32.pow(12) - 1 - gamma_rate;
    epsilon_rate * gamma_rate
}
