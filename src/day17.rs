pub(crate) fn solve_part_1(puzzle_input: String) -> String {
    let (_, _, y_target_min, _) = parse_puzzle_input(puzzle_input);
    let initial_y_velocity = (y_target_min + 1) * -1;
    (initial_y_velocity * (initial_y_velocity + 1) / 2).to_string()
}

pub(crate) fn solve_part_2(puzzle_input: String) -> String {
    let (x_target_min, x_target_max, y_target_min, y_target_max) = parse_puzzle_input(puzzle_input);

    let x_velocity_min: i64 =
        ((((x_target_min as f64) * 8.0 + 1.0).sqrt() - 1.0) / 2.0).ceil() as i64;
    // Expression is equivalent to v*(v+1)/2 = x_target_max solved for
    // v, which is the largest initial velocity that will cause the
    // probe to have zero horizontal velocity inside the target area:
    let x_velocity_stop_in_range_max =
        ((((x_target_max as f64) * 8.0 + 1.0).sqrt() - 1.0) / 2.0).floor() as i64;
    let x_velocity_max = x_target_max;

    let y_velocity_min = y_target_min;
    let y_velocity_max = (y_target_min + 1) * -1;
    println!(
        "{} {} {} {} {}",
        x_velocity_min,
        x_velocity_stop_in_range_max,
        x_velocity_max,
        y_velocity_min,
        y_velocity_max
    );
    let mut x_step_windows: Vec<(u64, Option<u64>)> = Vec::new();
    for x_velocity in x_velocity_min..=x_velocity_stop_in_range_max {
        let mut x_position = 0;
        let mut current_x_velocity = x_velocity;
        let mut step = 0;
        while x_position < x_target_min {
            x_position += current_x_velocity;
            current_x_velocity -= 1;
            step += 1;
        }
        x_step_windows.push((step, None));
    }
    for x_velocity in (x_velocity_stop_in_range_max + 1)..=x_velocity_max {
        let mut x_position = 0;
        let mut current_x_velocity = x_velocity;
        let mut step = 0;
        while x_position < x_target_min {
            x_position += current_x_velocity;
            current_x_velocity -= 1;
            step += 1;
        }
        let first_step_equal_or_over_minimum = step;
        while x_position <= x_target_max {
            x_position += current_x_velocity;
            current_x_velocity -= 1;
            step += 1;
        }
        let first_step_over_maximum = step;
        if first_step_equal_or_over_minimum != first_step_over_maximum {
            x_step_windows.push((
                first_step_equal_or_over_minimum,
                Some(first_step_over_maximum - 1),
            ));
        }
    }
    let mut y_step_windows: Vec<(u64, u64)> = Vec::new();
    for initial_y_velocity in y_velocity_min..=y_velocity_max {
        let mut y_position = 0;
        let mut current_y_velocity = initial_y_velocity;
        let mut step = 0;
        while y_position > y_target_max {
            y_position += current_y_velocity;
            current_y_velocity -= 1;
            step += 1;
        }
        let first_step_equal_or_under_maximum = step;
        while y_position >= y_target_min {
            y_position += current_y_velocity;
            current_y_velocity -= 1;
            step += 1;
        }
        let first_step_under_minimum = step;
        if first_step_equal_or_under_maximum != first_step_under_minimum {
            y_step_windows.push((
                first_step_equal_or_under_maximum,
                first_step_under_minimum - 1,
            ));
        }
    }
    let mut valid_initial_velocity_combinations: u64 = 0;
    for x_step_window in x_step_windows.iter() {
        for y_step_window in y_step_windows.iter() {
            let first_valid_x_step = x_step_window.0;
            let first_valid_y_step = y_step_window.0;
            let last_valid_y_step = y_step_window.1;
            match x_step_window.1 {
                None => {
                    if first_valid_x_step <= last_valid_y_step {
                        valid_initial_velocity_combinations += 1;
                    }
                }
                Some(last_valid_x_step) => {
                    if first_valid_x_step <= last_valid_y_step
                        && last_valid_x_step >= first_valid_y_step
                    {
                        valid_initial_velocity_combinations += 1;
                    }
                }
            }
        }
    }
    valid_initial_velocity_combinations.to_string()
}

fn parse_puzzle_input(puzzle_input: String) -> (i64, i64, i64, i64) {
    let mut ranges = puzzle_input.trim()[15..].split(", y=");
    let mut x_range = ranges.next().unwrap().split("..");
    let x_target_min = x_range.next().unwrap().parse::<i64>().unwrap();
    let x_target_max = x_range.next().unwrap().parse::<i64>().unwrap();
    let mut y_range = ranges.next().unwrap().split("..");
    let y_target_min = y_range.next().unwrap().parse::<i64>().unwrap();
    let y_target_max = y_range.next().unwrap().parse::<i64>().unwrap();
    (x_target_min, x_target_max, y_target_min, y_target_max)
}

#[cfg(test)]
use super::test_helpers;

#[test]
fn test_part_1() {
    assert_eq!(solve_part_1(test_helpers::load_puzzle_input(17)), "11781");
}

#[test]
fn test_part_2() {
    assert_eq!(solve_part_2(test_helpers::load_puzzle_input(17)), "4531");
}
