fn get_sorted_crab_positions(puzzle_input: String) -> Vec<i64> {
    let mut crab_initial_positions = puzzle_input
        .trim()
        .split(",")
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    crab_initial_positions.sort();
    crab_initial_positions
}

fn calculate_crab_fuel_cost(initial_position: i64, target_position: i64) -> i64 {
    let distance = (initial_position - target_position).abs();
    distance * (distance + 1) / 2
}

pub(crate) fn solve_part_1(puzzle_input: String) -> u64 {
    let crab_initial_positions = get_sorted_crab_positions(puzzle_input);
    let target_position = crab_initial_positions[crab_initial_positions.len() / 2];
    crab_initial_positions
        .iter()
        .map(|p| (p - target_position).abs())
        .sum::<i64>() as u64
}

pub(crate) fn solve_part_2(puzzle_input: String) -> u64 {
    let crab_initial_positions = get_sorted_crab_positions(puzzle_input);
    let minimum_target_position = *crab_initial_positions.iter().min().unwrap();
    let maximum_target_position = *crab_initial_positions.iter().max().unwrap();
    // (target_position, total_fuel_cost)
    let mut target_options: Vec<(i64, i64)> = Vec::new();
    for target_position in minimum_target_position..=maximum_target_position {
        let total_fuel_cost = crab_initial_positions
            .iter()
            .map(|crab_initial_position| {
                calculate_crab_fuel_cost(*crab_initial_position, target_position)
            })
            .sum();
        target_options.push((target_position, total_fuel_cost));
    }
    target_options.sort_by(|a, b| a.1.cmp(&b.1));
    target_options[0].1 as u64
}

#[cfg(test)]
use super::test_helpers;

#[test]
fn test_calculate_crab_fuel_cost() {
    assert_eq!(calculate_crab_fuel_cost(1, 1), 0);
    assert_eq!(calculate_crab_fuel_cost(1, 2), 1);
    assert_eq!(calculate_crab_fuel_cost(1, 3), 3);
    assert_eq!(calculate_crab_fuel_cost(1, 4), 6);
}

#[test]
fn test_part_1() {
    assert_eq!(solve_part_1(test_helpers::load_puzzle_input(7)), 355764);
}

#[test]
fn test_part_2() {
    assert_eq!(solve_part_2(test_helpers::load_puzzle_input(7)), 99634572);
}
