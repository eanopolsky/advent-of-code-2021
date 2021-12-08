pub(crate) fn solve_part_1(puzzle_input: String) -> u64 {
    let mut crab_initial_positions = puzzle_input
        .trim()
        .split(",")
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    crab_initial_positions.sort();
    let target_position = crab_initial_positions[crab_initial_positions.len() / 2];
    crab_initial_positions
        .iter()
        .map(|p| (p - target_position).abs())
        .sum::<i64>() as u64
}
