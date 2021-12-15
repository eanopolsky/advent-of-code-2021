use std::collections::HashMap;

struct PairInsertionRule {
    first: char,
    second: char,
    insert: char,
}

fn parse_input(puzzle_input: String) -> (Vec<char>, Vec<PairInsertionRule>) {
    let mut puzzle_part_iter = puzzle_input.trim().split("\n\n");
    let polymer_template = puzzle_part_iter
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
    (polymer_template, pair_insertion_rules)
}

fn pair_to_element_frequency(
    first_element: char,
    second_element: char,
    steps: u32,
    pair_insertion_rules: &Vec<PairInsertionRule>,
    cache: &mut HashMap<(char, char, u32), HashMap<char, u64>>,
) -> HashMap<char, u64> {
    if cache.contains_key(&(first_element, second_element, steps)) {
        return cache
            .get(&(first_element, second_element, steps))
            .unwrap()
            .clone();
    }
    let mut element_frequency = HashMap::<char, u64>::new();
    if steps == 0 {
        if first_element == second_element {
            element_frequency.insert(first_element, 2);
        } else {
            element_frequency.insert(first_element, 1);
            element_frequency.insert(second_element, 1);
        }
        cache.insert(
            (first_element, second_element, steps),
            element_frequency.clone(),
        );
        return element_frequency;
    }
    let middle_element = pair_insertion_rules
        .iter()
        .filter(|rule| rule.first == first_element && rule.second == second_element)
        .map(|rule| rule.insert)
        .next()
        .unwrap();
    let first_frequencies = pair_to_element_frequency(
        first_element,
        middle_element,
        steps - 1,
        pair_insertion_rules,
        cache,
    );
    let second_frequencies = pair_to_element_frequency(
        middle_element,
        second_element,
        steps - 1,
        pair_insertion_rules,
        cache,
    );
    let result =
        reduce_adjacent_frequencies(&first_frequencies, &second_frequencies, middle_element);
    cache.insert((first_element, second_element, steps), result.clone());
    result
}

fn reduce_adjacent_frequencies(
    first_frequencies: &HashMap<char, u64>,
    second_frequencies: &HashMap<char, u64>,
    middle_element: char,
) -> HashMap<char, u64> {
    let mut reduced_frequencies: HashMap<char, u64> = first_frequencies.clone();
    for (element, second_count) in second_frequencies.iter() {
        match reduced_frequencies.get_mut(element) {
            Some(first_count) => {
                *first_count += second_count;
            }
            None => {
                reduced_frequencies.insert(*element, *second_count);
            }
        }
    }
    *reduced_frequencies.get_mut(&middle_element).unwrap() -= 1;
    reduced_frequencies
}

pub(crate) fn solve_part_1(puzzle_input: String) -> String {
    let (polymer_template, pair_insertion_rules) = parse_input(puzzle_input);

    let mut cache: HashMap<(char, char, u32), HashMap<char, u64>> = HashMap::new();
    let mut element_frequencies: HashMap<char, u64> = pair_to_element_frequency(
        polymer_template[0],
        polymer_template[1],
        10,
        &pair_insertion_rules,
        &mut cache,
    );

    let mut polymer_index: usize = 1;

    while polymer_index <= polymer_template.len() - 2 {
        let next_element_frequencies = pair_to_element_frequency(
            polymer_template[polymer_index],
            polymer_template[polymer_index + 1],
            10,
            &pair_insertion_rules,
            &mut cache,
        );
        element_frequencies = reduce_adjacent_frequencies(
            &element_frequencies,
            &next_element_frequencies,
            polymer_template[polymer_index],
        );
        polymer_index += 1;
    }

    (element_frequencies.values().max().unwrap() - element_frequencies.values().min().unwrap())
        .to_string()
}

pub(crate) fn solve_part_2(puzzle_input: String) -> String {
    let (polymer_template, pair_insertion_rules) = parse_input(puzzle_input);

    let mut cache: HashMap<(char, char, u32), HashMap<char, u64>> = HashMap::new();
    let mut element_frequencies: HashMap<char, u64> = pair_to_element_frequency(
        polymer_template[0],
        polymer_template[1],
        40,
        &pair_insertion_rules,
        &mut cache,
    );

    let mut polymer_index: usize = 1;

    while polymer_index <= polymer_template.len() - 2 {
        let next_element_frequencies = pair_to_element_frequency(
            polymer_template[polymer_index],
            polymer_template[polymer_index + 1],
            40,
            &pair_insertion_rules,
            &mut cache,
        );
        element_frequencies = reduce_adjacent_frequencies(
            &element_frequencies,
            &next_element_frequencies,
            polymer_template[polymer_index],
        );
        polymer_index += 1;
    }

    (element_frequencies.values().max().unwrap() - element_frequencies.values().min().unwrap())
        .to_string()
}

#[cfg(test)]
use super::test_helpers;

#[test]
fn test_part_1() {
    assert_eq!(solve_part_1(test_helpers::load_puzzle_input(14)), "2657");
}

#[test]
fn test_pair_to_element_frequency() {
    let mut cache: HashMap<(char, char, u32), HashMap<char, u64>> = HashMap::new();
    let frequency_test =
        pair_to_element_frequency('N', 'N', 0, &Vec::<PairInsertionRule>::new(), &mut cache);
    assert_eq!(*frequency_test.get(&'N').unwrap(), 2);
}

#[test]
fn test_part_2() {
    assert_eq!(
        solve_part_2(test_helpers::load_puzzle_input(14)),
        "2911561572630"
    );
}
