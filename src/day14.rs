use std::collections::HashMap;

struct PairInsertionRule {
    first: char,
    second: char,
    insert: char,
}

pub(crate) fn solve_part_1(puzzle_input: String) -> String {
    let mut puzzle_part_iter = puzzle_input.trim().split("\n\n");
    let mut polymer_template = puzzle_part_iter
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    let pair_insertion_rules = puzzle_part_iter
        .next()
        .unwrap()
        .split("\n")
        .map(|line| {
            let mut line_parts_iter = line.split(" -> ");
            let mut pair_char_iter = line_parts_iter.next().unwrap().chars();
            let insert = line_parts_iter.next().unwrap().chars().next().unwrap();
            PairInsertionRule {
                first: pair_char_iter.next().unwrap(),
                second: pair_char_iter.next().unwrap(),
                insert,
            }
        })
        .collect::<Vec<PairInsertionRule>>();

    for _step in 1..=10 {
        let mut polymer_index: usize = 0;
        while polymer_index <= polymer_template.len() - 2 {
            for possible_rule in pair_insertion_rules.iter() {
                if polymer_template[polymer_index] == possible_rule.first
                    && polymer_template[polymer_index + 1] == possible_rule.second
                {
                    polymer_template.insert(polymer_index + 1, possible_rule.insert);
                    polymer_index += 1;
                    break;
                }
            }
            polymer_index += 1;
        }
    }

    let mut element_frequency: HashMap<char, u32> = HashMap::new();
    for element in polymer_template {
        if element_frequency.contains_key(&element) {
            *element_frequency.get_mut(&element).unwrap() += 1;
        } else {
            element_frequency.insert(element, 1);
        }
    }
    (element_frequency.values().max().unwrap() - element_frequency.values().min().unwrap())
        .to_string()
}

pub(crate) fn solve_part_2(_puzzle_input: String) -> String {
    "".to_string()
}

#[cfg(test)]
use super::test_helpers;

#[test]
fn test_part_1() {
    assert_eq!(solve_part_1(test_helpers::load_puzzle_input(14)), "2657");
}
