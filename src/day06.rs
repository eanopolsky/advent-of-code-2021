fn simulate_lanternfish_day(lanternfish: &mut [u64; 9]) {
    let parent_lanternfish = lanternfish[0];
    for i in 0..8 {
        lanternfish[i] = lanternfish[i + 1];
    }
    lanternfish[6] += parent_lanternfish;
    lanternfish[8] = parent_lanternfish;
}

fn get_lanternfish(puzzle_input: String) -> [u64; 9] {
    let mut lanternfish: [u64; 9] = [0; 9];
    for lanternfish_due_day in puzzle_input
        .trim()
        .split(",")
        .map(|lanterfish_due_string| lanterfish_due_string.parse::<usize>().unwrap())
    {
        lanternfish[lanternfish_due_day] += 1;
    }
    lanternfish
}

pub(crate) fn solve_part_1(puzzle_input: String) -> u64 {
    let mut lanternfish = get_lanternfish(puzzle_input);
    for _day in 0..80 {
        //println!("After {} day(s), lanternfish: {:?}", day, lanternfish);
        simulate_lanternfish_day(&mut lanternfish);
    }
    lanternfish.iter().sum::<u64>()
}

pub(crate) fn solve_part_2(puzzle_input: String) -> u64 {
    let mut lanternfish = get_lanternfish(puzzle_input);
    for _day in 0..256 {
        //println!("After {} day(s), lanternfish: {:?}", day, lanternfish);
        simulate_lanternfish_day(&mut lanternfish);
    }
    lanternfish.iter().sum::<u64>()
}

#[cfg(test)]
use super::test_helpers;

#[test]
fn test_part_1() {
    assert_eq!(solve_part_1(test_helpers::load_puzzle_input(6)), 362666);
}

#[test]
fn test_part_2() {
    assert_eq!(
        solve_part_2(test_helpers::load_puzzle_input(6)),
        1640526601595
    );
}
