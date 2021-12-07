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

pub(crate) fn solve_part_1(puzzle_input: String) -> u32 {
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

pub(crate) fn solve_part_2(puzzle_input: String) -> u32 {
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

#[cfg(test)]
use super::test_helpers;

#[test]
fn test_part_1() {
    assert_eq!(solve_part_1(test_helpers::load_puzzle_input(2)), 1804520);
}

#[test]
fn test_part_2() {
    assert_eq!(solve_part_2(test_helpers::load_puzzle_input(2)), 1971095320);
}
