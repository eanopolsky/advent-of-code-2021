use std::collections::HashMap;
use std::collections::HashSet;

fn parse_puzzle_input(puzzle_input: &String) -> Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)> {
    //Each element of the outer vector is a line from the input.
    //The first vector of HashSets inside the tuple contains all
    // possible output patterns.
    //The second vector of HashSets inside the tuple contains the four
    // output patterns for the row.
    puzzle_input
        .trim()
        .split("\n")
        .map(|line| {
            let mut line_parts = line.split(" | ");
            let ten_pattern_string = line_parts.next().unwrap();
            let four_pattern_string = line_parts.next().unwrap();
            let all_patterns = ten_pattern_string
                .split_whitespace()
                .map(|wire_pattern_string| string_to_set(wire_pattern_string))
                .collect();
            let output_patterns = four_pattern_string
                .split_whitespace()
                .map(|wire_pattern_string| string_to_set(wire_pattern_string))
                .collect();
            (all_patterns, output_patterns)
        })
        .collect()
}

fn string_to_set(character_string: &str) -> HashSet<char> {
    let mut character_set = HashSet::new();
    for character in character_string.chars() {
        character_set.insert(character);
    }
    character_set
}

pub(crate) fn solve_part_1(puzzle_input: String) -> u64 {
    let input_lines = parse_puzzle_input(&puzzle_input);
    let mut digit_count: u64 = 0;
    for input_line in input_lines {
        for output_pattern in input_line.1 {
            match output_pattern.len() {
                2 => digit_count += 1,
                4 => digit_count += 1,
                3 => digit_count += 1,
                7 => digit_count += 1,
                _ => (),
            }
        }
    }
    digit_count
}

pub(crate) fn solve_part_2(puzzle_input: String) -> u64 {
    let input_lines = parse_puzzle_input(&puzzle_input);
    let mut output_sum: u64 = 0;
    for input_line in input_lines {
        let all_patterns = input_line.0;
        let mut encode_map = HashMap::<u64, &HashSet<char>>::new();
        for pattern in &all_patterns {
            match pattern.len() {
                2 => encode_map.insert(1, pattern),
                4 => encode_map.insert(4, pattern),
                3 => encode_map.insert(7, pattern),
                7 => encode_map.insert(8, pattern),
                _ => None,
            };
        }
        for pattern in &all_patterns {
            if pattern.len() == 6
                && pattern
                    .intersection(encode_map.get(&1).unwrap())
                    .collect::<Vec<&char>>()
                    .len()
                    == 1
            {
                encode_map.insert(6, pattern);
                break;
            }
        }
        let right_top_segment_letter: char = *encode_map
            .get(&8)
            .unwrap()
            .difference(encode_map.get(&6).unwrap())
            .next()
            .unwrap();
        for pattern in &all_patterns {
            if pattern.len() == 5 && !pattern.contains(&right_top_segment_letter) {
                encode_map.insert(5, pattern);
                break;
            }
        }
        let mut left_bottom_segment_letter: char = '?';
        for missing_letter in encode_map
            .get(&8)
            .unwrap()
            .difference(encode_map.get(&5).unwrap())
        {
            if *missing_letter != right_top_segment_letter {
                left_bottom_segment_letter = *missing_letter;
                break;
            }
        }
        for pattern in &all_patterns {
            if pattern.len() != 5 || *encode_map.get(&5).unwrap() == pattern {
                continue;
            }
            if pattern.contains(&left_bottom_segment_letter) {
                encode_map.insert(2, pattern);
            } else {
                encode_map.insert(3, pattern);
            }
        }
        for pattern in &all_patterns {
            if pattern.len() != 6 || *encode_map.get(&6).unwrap() == pattern {
                continue;
            }
            if pattern.contains(&left_bottom_segment_letter) {
                encode_map.insert(0, pattern);
            } else {
                encode_map.insert(9, pattern);
            }
        }
        // Apparently this doesn't work because trait bounds are not satisfied:
        // let mut decode_map = HashMap::<&HashSet<char>, u64>::new();
        // for (digit, pattern) in encode_map.iter() {
        //     decode_map.insert(pattern, digit);
        // }

        let output_patterns = input_line.1;
        let mut output_value: u64 = 0;
        for output_pattern in &output_patterns {
            output_value *= 10;
            for (digit, pattern) in encode_map.iter() {
                if output_pattern == *pattern {
                    output_value += digit;
                    break;
                }
            }
        }
        output_sum += output_value;
    }
    output_sum
}

#[cfg(test)]
use super::test_helpers;

#[test]
fn test_part_1() {
    assert_eq!(solve_part_1(test_helpers::load_puzzle_input(8)), 421);
}

#[test]
fn test_part_2() {
    assert_eq!(solve_part_2(test_helpers::load_puzzle_input(8)), 986163);
}
