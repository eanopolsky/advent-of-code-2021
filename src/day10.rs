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

fn is_line_corrupted(navigation_line: &str) -> bool {
    match find_first_illegal_character(navigation_line) {
        Some(_) => true,
        None => false,
    }
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

fn get_closing_characters(navigation_line: &str) -> Vec<char> {
    let mut stack = Vec::<char>::new();
    for navigation_char in navigation_line.chars() {
        match navigation_char {
            '(' | '[' | '{' | '<' => stack.push(navigation_char),
            ')' | ']' | '}' | '>' => {
                stack.pop();
                ()
            }
            _ => panic!("Invalid navigation_char"),
        }
    }
    let mut closing_characters = Vec::<char>::new();
    for remaining_char in stack.iter().rev() {
        match remaining_char {
            '(' => closing_characters.push(')'),
            '[' => closing_characters.push(']'),
            '{' => closing_characters.push('}'),
            '<' => closing_characters.push('>'),
            _ => panic!("Invalid closing character"),
        }
    }

    closing_characters
}

fn score_closing_characters(closing_characters: Vec<char>) -> u64 {
    let mut score: u64 = 0;
    for closing_character in closing_characters {
        score *= 5;
        match closing_character {
            ')' => score += 1,
            ']' => score += 2,
            '}' => score += 3,
            '>' => score += 4,
            _ => panic!("Invalid closing character"),
        }
    }
    score
}

pub(crate) fn solve_part_1(puzzle_input: String) -> u64 {
    puzzle_input
        .trim()
        .split("\n")
        .filter_map(find_first_illegal_character)
        .map(calculate_character_points)
        .sum()
}

pub(crate) fn solve_part_2(puzzle_input: String) -> u64 {
    let mut line_scores = puzzle_input
        .trim()
        .split("\n")
        .filter(|line| !is_line_corrupted(line))
        .map(get_closing_characters)
        .map(score_closing_characters)
        .collect::<Vec<u64>>();
    line_scores.sort();
    line_scores[line_scores.len() / 2]
}

#[cfg(test)]
use super::test_helpers;

#[test]
fn test_part_1() {
    assert_eq!(solve_part_1(test_helpers::load_puzzle_input(10)), 462693);
}

#[test]
fn test_part_2() {
    assert_eq!(
        solve_part_2(test_helpers::load_puzzle_input(10)),
        3094671161
    );
}
