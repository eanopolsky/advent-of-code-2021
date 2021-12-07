pub(crate) fn solve_part_1(puzzle_input: String) -> u32 {
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

pub(crate) fn solve_part_2(puzzle_input: String) -> u32 {
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

#[cfg(test)]
use super::test_helpers;

#[test]
fn test_part_1() {
    assert_eq!(solve_part_1(test_helpers::load_puzzle_input(1)), 1676);
}

#[test]
fn test_part_2() {
    assert_eq!(solve_part_2(test_helpers::load_puzzle_input(1)), 1706);
}
