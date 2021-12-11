fn find_first_illegal_character(navigation_line: &str) -> Option<char> {
    let mut stack = Vec::<char>::new();
    for navigation_char in navigation_line.chars() {
        if navigation_char == '('
            || navigation_char == '['
            || navigation_char == '{'
            || navigation_char == '<'
        {
            stack.push(navigation_char);
            continue;
        }
        let expected_companion = match navigation_char {
            ')' => '(',
            ']' => '[',
            '}' => '{',
            '>' => '<',
            _ => panic!("Invalid navigation_char"),
        };
        if expected_companion != stack.pop().unwrap() {
            return Some(navigation_char);
        }
    }
    None
}

fn calculate_character_points(illegal_char: char) -> u64 {
    match illegal_char {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Invalid illegal_char"),
    }
}

pub(crate) fn solve_part_1(puzzle_input: String) -> u64 {
    puzzle_input
        .trim()
        .split("\n")
        .filter_map(find_first_illegal_character)
        .map(calculate_character_points)
        .sum()
}

#[cfg(test)]
use super::test_helpers;

#[test]
fn test_part_1() {
    assert_eq!(solve_part_1(test_helpers::load_puzzle_input(10)), 462693);
}

// #[test]
// fn test_part_2() {
//     assert_eq!(solve_part_2(test_helpers::load_puzzle_input(9)), 959136);
// }
