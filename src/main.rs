use clap::{App, Arg};
use std::cmp;
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
        (3, 2) => solve_day_3_part_2(puzzle_input),
        (4, 1) => solve_day_4_part_1(puzzle_input),
        (4, 2) => solve_day_4_part_2(puzzle_input),
        (5, 1) => solve_day_5_part_1(puzzle_input),
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
    let diagnostic_report = puzzle_input
        .split("\n")
        .filter(|report_number| (report_number.len() != 0));
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

fn solve_day_3_part_2(puzzle_input: String) -> u32 {
    let diagnostic_report: Vec<&str> = puzzle_input
        .split("\n")
        .filter(|report_number| -> bool { report_number.len() != 0 })
        .collect();
    let mut oxygen_generator_possibilities = diagnostic_report.clone();
    for bit_position in 0..12 {
        oxygen_generator_possibilities =
            trim_diagnostic_report(oxygen_generator_possibilities, bit_position, true);
        if oxygen_generator_possibilities.len() == 1 {
            break;
        }
    }

    let mut co2_scrubber_possibilities = diagnostic_report.clone();
    for bit_position in 0..12 {
        co2_scrubber_possibilities =
            trim_diagnostic_report(co2_scrubber_possibilities, bit_position, false);
        if co2_scrubber_possibilities.len() == 1 {
            break;
        }
    }

    u32::from_str_radix(oxygen_generator_possibilities[0], 2).unwrap()
        * u32::from_str_radix(co2_scrubber_possibilities[0], 2).unwrap()
}

fn trim_diagnostic_report(
    report: Vec<&str>,
    bit_position: usize,
    keep_most_common: bool,
) -> Vec<&str> {
    let report_length = report.len();
    let mut one_count_at_bit_position = 0;
    for report_element in report.iter() {
        if report_element.chars().collect::<Vec<char>>()[bit_position] == '1' {
            one_count_at_bit_position += 1;
        }
    }

    let keep_ones: bool = (one_count_at_bit_position * 2 >= report_length && keep_most_common)
        || (one_count_at_bit_position * 2 < report_length && !keep_most_common);

    let mut trimmed_report: Vec<&str> = Vec::new();
    for report_element in report.iter() {
        let char_at_bit_position = report_element.chars().collect::<Vec<char>>()[bit_position];
        if (char_at_bit_position == '1' && keep_ones) || (char_at_bit_position == '0' && !keep_ones)
        {
            trimmed_report.push(report_element);
        }
    }
    trimmed_report
}

#[derive(Debug)]
struct BingoSquare {
    number: u32,
    called: bool,
}

#[derive(Debug)]
struct BingoBoard {
    //each inner vector is a row
    squares: Vec<Vec<BingoSquare>>,
}

impl BingoBoard {
    fn mark_number(&mut self, called_number: &u32) {
        for x in 0..5 {
            for y in 0..5 {
                if self.squares[x][y].number == *called_number {
                    self.squares[x][y].called = true;
                }
            }
        }
    }

    fn is_winner(&self) -> bool {
        for row in &self.squares {
            if row
                .iter()
                .map(|square| square.called)
                .reduce(|called_1, called_2| called_1 && called_2)
                .unwrap()
            {
                return true;
            }
        }
        for column_number in 0..5 {
            if self
                .squares
                .iter()
                .map(|row| row[column_number].called)
                .reduce(|called_1, called_2| called_1 && called_2)
                .unwrap()
            {
                return true;
            }
        }
        false
    }

    fn sum_of_unmarked_squares(&self) -> u32 {
        self.squares
            .iter()
            .map(|row| -> u32 {
                row.iter()
                    .filter(|square| !square.called)
                    .map(|square| -> u32 { square.number })
                    .sum()
            })
            .sum()
    }
}

fn solve_day_4_part_1(puzzle_input: String) -> u32 {
    let mut section_iterator = puzzle_input.split("\n\n");

    let numbers_called: Vec<u32> = section_iterator
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    let mut boards: Vec<BingoBoard> = Vec::new();

    for board_section in section_iterator {
        let new_squares = board_section
            .split("\n")
            .filter(|line| line.len() != 0)
            .map(|line| -> Vec<BingoSquare> {
                line.split_whitespace()
                    .map(|num| num.parse::<u32>().unwrap())
                    .map(|num| BingoSquare {
                        number: num,
                        called: false,
                    })
                    .collect()
            })
            .collect::<Vec<Vec<BingoSquare>>>();
        boards.push(BingoBoard {
            squares: new_squares,
        });
    }

    for number_called in numbers_called.iter() {
        for board in &mut boards {
            board.mark_number(number_called);
            if board.is_winner() {
                return board.sum_of_unmarked_squares() * number_called;
            }
        }
    }
    panic!("Failed to find winning board.");
}

fn solve_day_4_part_2(puzzle_input: String) -> u32 {
    let mut section_iterator = puzzle_input.split("\n\n");

    let numbers_called: Vec<u32> = section_iterator
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    let mut boards: Vec<BingoBoard> = Vec::new();

    for board_section in section_iterator {
        let new_squares = board_section
            .split("\n")
            .filter(|line| line.len() != 0)
            .map(|line| -> Vec<BingoSquare> {
                line.split_whitespace()
                    .map(|num| num.parse::<u32>().unwrap())
                    .map(|num| BingoSquare {
                        number: num,
                        called: false,
                    })
                    .collect()
            })
            .collect::<Vec<Vec<BingoSquare>>>();
        boards.push(BingoBoard {
            squares: new_squares,
        });
    }

    for number_called in numbers_called.iter() {
        for board in &mut boards {
            board.mark_number(number_called);
        }
        let non_winning_boards_left: u32 = boards
            .iter()
            .map(|board| {
                if !board.is_winner() {
                    return 1;
                }
                0
            })
            .sum();
        println!("non-winning boards left: {}", non_winning_boards_left);
        if non_winning_boards_left > 0 {
            boards = boards
                .into_iter()
                .filter(|board| !board.is_winner())
                .collect();
            continue;
        }
        return boards.last().unwrap().sum_of_unmarked_squares() * number_called;
    }
    panic!("Failed to find last winning board.");
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct VentLine {
    start_point: Point,
    end_point: Point,
}

impl VentLine {
    fn is_horizontal_or_vertical(&self) -> bool {
        if self.is_horizontal() || self.is_vertical() {
            return true;
        }
        false
    }

    fn is_horizontal(&self) -> bool {
        if self.start_point.y == self.end_point.y {
            return true;
        }
        false
    }

    fn is_vertical(&self) -> bool {
        if self.start_point.x == self.end_point.x {
            return true;
        }
        false
    }

    fn get_points_on_line(&self) -> Vec<Point> {
        let mut points_on_line = Vec::new();
        if self.is_horizontal() {
            let x_range = match self.start_point.x < self.end_point.x {
                true => (self.start_point.x)..=(self.end_point.x),
                false => (self.end_point.x)..=(self.start_point.x),
            };
            for x in x_range {
                points_on_line.push(Point {
                    x: x,
                    y: self.start_point.y,
                });
            }
        } else if self.is_vertical() {
            let y_range = match self.start_point.y < self.end_point.y {
                true => (self.start_point.y)..=(self.end_point.y),
                false => (self.end_point.y)..=(self.start_point.y),
            };
            for y in y_range {
                points_on_line.push(Point {
                    x: self.start_point.x,
                    y: y,
                });
            }
        } else {
            panic!(
                "Getting points on lines that are neither horizontal nor vertical is not supported"
            );
        }
        points_on_line
    }
}

fn solve_day_5_part_1(puzzle_input: String) -> u32 {
    let vent_lines = puzzle_input
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line_definition| -> VentLine {
            // println!("Processing input line: {}", line_definition);
            let mut vent_points = line_definition
                .split(" -> ")
                .map(|point_definition| -> Point {
                    let mut ordinate_iter = point_definition
                        .split(",")
                        .map(|n| -> i32 { (n.parse::<i32>()).unwrap() });

                    Point {
                        x: ordinate_iter.next().unwrap(),
                        y: ordinate_iter.next().unwrap(),
                    }
                });

            VentLine {
                start_point: vent_points.next().unwrap(),
                end_point: vent_points.next().unwrap(),
            }
        })
        .collect::<Vec<VentLine>>();

    let horizontal_and_vertical_vent_lines = vent_lines
        .iter()
        .filter(|vent_line| vent_line.is_horizontal_or_vertical())
        .collect::<Vec<&VentLine>>();

    let max_x: usize = horizontal_and_vertical_vent_lines
        .iter()
        .map(|vent_line| cmp::max(vent_line.start_point.x, vent_line.end_point.x))
        .max()
        .unwrap() as usize;
    let max_y: usize = horizontal_and_vertical_vent_lines
        .iter()
        .map(|vent_line| cmp::max(vent_line.start_point.y, vent_line.end_point.y))
        .max()
        .unwrap() as usize;

    println!("Got max_x: {}, max_y: {}", max_x, max_y);
    let mut ocean_floor: Vec<Vec<u32>> = vec![vec![0; max_x + 1]; max_y + 1];

    // for vent_line in &horizontal_and_vertical_vent_lines {
    //     println!("{:#?}", vent_line);
    // }

    // println!(
    //     "Points on line at offset 1: {:?}",
    //     horizontal_and_vertical_vent_lines[0].get_points_on_line()
    // );

    for vent_lines in &horizontal_and_vertical_vent_lines {
        for vent_line_point in vent_lines.get_points_on_line() {
            ocean_floor[vent_line_point.y as usize][vent_line_point.x as usize] += 1;
        }
    }
    let mut overlap_count: u32 = 0;
    for ocean_floor_row in ocean_floor {
        for vent_count in ocean_floor_row {
            if vent_count > 1 {
                overlap_count += 1;
            }
        }
    }

    overlap_count
}
